use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::in_constant;
use clippy_utils::source::snippet_opt;
use clippy_utils::ty::is_isize_or_usize;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::LateContext;
use rustc_middle::ty::{self, FloatTy, Ty};

use super::{utils, CAST_LOSSLESS};

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, cast_op: &Expr<'_>, cast_from: Ty<'_>, cast_to: Ty<'_>) {
    if !should_lint(cx, expr, cast_from, cast_to) {
        return;
    }

    // The suggestion is to use a function call, so if the original expression
    // has parens on the outside, they are no longer needed.
    let mut applicability = Applicability::MachineApplicable;
    let opt = snippet_opt(cx, cast_op.span);
    let sugg = opt.as_ref().map_or_else(
        || {
            applicability = Applicability::HasPlaceholders;
            ".."
        },
        |snip| {
            if should_strip_parens(cast_op, snip) {
                &snip[1..snip.len() - 1]
            } else {
                snip.as_str()
            }
        },
    );

    span_lint_and_sugg(
        cx,
        CAST_LOSSLESS,
        expr.span,
        &format!(
            "casting `{}` to `{}` may become silently lossy if you later change the type",
            cast_from, cast_to
        ),
        "try",
        format!("{}::from({})", cast_to, sugg),
        applicability,
    );
}

fn should_lint(cx: &LateContext<'_>, expr: &Expr<'_>, cast_from: Ty<'_>, cast_to: Ty<'_>) -> bool {
    // Do not suggest using From in consts/statics until it is valid to do so (see #2267).
    if in_constant(cx, expr.hir_id) {
        return false;
    }

    match (cast_from.is_integral(), cast_to.is_integral()) {
        (true, true) => {
            let cast_signed_to_unsigned = cast_from.is_signed() && !cast_to.is_signed();
            let from_nbits = utils::int_ty_to_nbits(cast_from, cx.tcx);
            let to_nbits = utils::int_ty_to_nbits(cast_to, cx.tcx);
            !is_isize_or_usize(cast_from)
                && !is_isize_or_usize(cast_to)
                && from_nbits < to_nbits
                && !cast_signed_to_unsigned
        },

        (true, false) => {
            let from_nbits = utils::int_ty_to_nbits(cast_from, cx.tcx);
            let to_nbits = if cast_to.kind() == &ty::Float(FloatTy::F32) {
                32
            } else {
                64
            };
            from_nbits < to_nbits
        },

        (_, _) => cast_from.kind() == &ty::Float(FloatTy::F32) && cast_to.kind() == &ty::Float(FloatTy::F64),
    }
}

fn should_strip_parens(cast_expr: &Expr<'_>, snip: &str) -> bool {
    if let ExprKind::Binary(_, _, _) = cast_expr.kind {
        if snip.starts_with('(') && snip.ends_with(')') {
            return true;
        }
    }
    false
}
