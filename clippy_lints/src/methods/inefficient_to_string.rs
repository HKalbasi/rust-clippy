use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::source::snippet_with_applicability;
use clippy_utils::ty::{is_type_diagnostic_item, walk_ptrs_ty_depth};
use clippy_utils::{match_def_path, paths};
use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_middle::ty::{self, Ty};
use rustc_span::symbol::{sym, Symbol};

use super::INEFFICIENT_TO_STRING;

/// Checks for the `INEFFICIENT_TO_STRING` lint
pub fn check<'tcx>(cx: &LateContext<'tcx>, expr: &hir::Expr<'_>, method_name: Symbol, args: &[hir::Expr<'_>]) {
    if_chain! {
        if args.len() == 1 && method_name == sym!(to_string);
        if let Some(to_string_meth_did) = cx.typeck_results().type_dependent_def_id(expr.hir_id);
        if match_def_path(cx, to_string_meth_did, &paths::TO_STRING_METHOD);
        if let Some(substs) = cx.typeck_results().node_substs_opt(expr.hir_id);
        let arg_ty = cx.typeck_results().expr_ty_adjusted(&args[0]);
        let self_ty = substs.type_at(0);
        let (deref_self_ty, deref_count) = walk_ptrs_ty_depth(self_ty);
        if deref_count >= 1;
        if specializes_tostring(cx, deref_self_ty);
        then {
            span_lint_and_then(
                cx,
                INEFFICIENT_TO_STRING,
                expr.span,
                &format!("calling `to_string` on `{}`", arg_ty),
                |diag| {
                    diag.help(&format!(
                        "`{}` implements `ToString` through a slower blanket impl, but `{}` has a fast specialization of `ToString`",
                        self_ty, deref_self_ty
                    ));
                    let mut applicability = Applicability::MachineApplicable;
                    let arg_snippet = snippet_with_applicability(cx, args[0].span, "..", &mut applicability);
                    diag.span_suggestion(
                        expr.span,
                        "try dereferencing the receiver",
                        format!("({}{}).to_string()", "*".repeat(deref_count), arg_snippet),
                        applicability,
                    );
                },
            );
        }
    }
}

/// Returns whether `ty` specializes `ToString`.
/// Currently, these are `str`, `String`, and `Cow<'_, str>`.
fn specializes_tostring(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    if ty.kind() == &ty::Str {
        return true;
    }

    if is_type_diagnostic_item(cx, ty, sym::String) {
        return true;
    }

    if let ty::Adt(adt, substs) = ty.kind() {
        match_def_path(cx, adt.did, &paths::COW) && substs.type_at(1).is_str()
    } else {
        false
    }
}
