// run-rustfix

#![allow(unused_variables, dead_code, clippy::redundant_pattern_matching, clippy::op_ref)]
#![warn(clippy::equatable_if_let, clippy::equatable_matches)]

#[derive(PartialEq)]
enum Enum {
    TupleVariant(i32, u64),
    RecordVariant { a: i64, b: u32 },
    UnitVariant,
    Recursive(Struct),
}

#[derive(PartialEq)]
struct Struct {
    a: i32,
    b: bool,
}

#[derive(Clone, Copy)]
enum NotPartialEq {
    A,
    B,
}

#[derive(Clone, Copy)]
enum NotStructuralEq {
    A,
    B,
}

impl PartialEq for NotStructuralEq {
    fn eq(&self, _: &NotStructuralEq) -> bool {
        false
    }
}

#[derive(PartialEq)]
enum Generic<A, B> {
    VA(A),
    VB(B),
    VC,
}

#[derive(PartialEq)]
struct Generic2<A, B> {
    a: A,
    b: B,
}

fn main() {
    let a = 2;
    let b = 3;
    let c = Some(2);
    let d = Struct { a: 2, b: false };
    let e = Enum::UnitVariant;
    let f = NotPartialEq::A;
    let g = NotStructuralEq::A;
    let h: Generic<Enum, NotPartialEq> = Generic::VC;
    let i: Generic<Enum, NotStructuralEq> = Generic::VC;
    let j = vec![1, 2, 3, 4];
    let k = Some(&false);
    let l = Generic2 {
        a: Generic2 { a: "xxxx", b: 3 },
        b: Generic2 {
            a: &Enum::UnitVariant,
            b: false,
        },
    };
    let m = Generic2 { a: 3, b: 5 };
    let n = Some("xxxx");
    let mut o = j.iter();

    // true

    if let Some(2) = c {}
    if let Struct { a: 2, b: false } = d {}
    if let Enum::TupleVariant(32, 64) = e {}
    if let Enum::RecordVariant { a: 64, b: 32 } = e {}
    if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}
    if let Generic::VA(Enum::UnitVariant) = i {}
    if let [7, 5] = j[1..3] {}
    if let [1, 2, 3, 4] = j[..] {}
    if let Some(true) = k {}
    if let Some(&false) = k {}
    if let Some(true) = &k {}
    if let Some(false) = &&k {}
    if let Generic2 {
        a: Generic2 { a: "yyy", b: 3 },
        b: Generic2 {
            a: Enum::UnitVariant,
            b: false,
        },
    } = l
    {}
    if let Generic2 { a: 3, b: 5 } = m {}
    if let Some("yyy") = n {}
    let _ = matches!(c, Some(2));

    while let Some(2) = o.next() {}

    // false

    if let 2 | 3 = a {}
    if let x @ 2 = a {}
    if let Some(3 | 4) = c {}
    if let Struct { a, b: false } = d {}
    if let Struct { a: 2, b: x } = d {}
    if let NotPartialEq::A = f {}
    if let NotStructuralEq::A = g {}
    if let Some(NotPartialEq::A) = Some(f) {}
    if let None = Some(f) {}
    if let Some(NotStructuralEq::A) = Some(g) {}
    if let Generic::VA(Enum::UnitVariant) = h {}
    if let Generic::VB(NotPartialEq::A) = h {}
    if let Generic::VC = h {}
    if let Generic::VB(NotStructuralEq::A) = i {}
    if let [7, _] = j[2..] {}
    if let [1, 2 | 5, 3, 4] = j[..] {}
    if let [2, ..] = j[..] {}

    let _ = matches!(c, Some(x));
    let _ = matches!(c, Some(x) if x == 2);
    let _ = matches!(c, Some(2) if 3 > 5);

    while let Some(4 | 7) = o.next() {}
}
