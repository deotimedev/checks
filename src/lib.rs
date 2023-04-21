#![feature(generic_const_exprs)]


pub enum Check<const EXPRESSION: bool> {}

pub trait Passed {}
pub trait Failed {}

pub enum Equals<const A: i32, const B: i32> {}
pub enum Not<const EXPRESSION: bool> {}
impl<const A: i32, const B: i32> Passed for Equals<A, B>
    where Check<{ A == B }>: Passed {
}

impl<const A: i32, const B: i32> Failed for Equals<A, B>
    where Check<{ A == B }>: Failed {
}

struct Test<const T: i32>(i32)
    where Equals<T, 2>: Passed;

// macro_rules! const_category {
//     (numbers) => (i8 i16 i32 i64)
// }
//
// #[macro_export]
// macro_rules! define_check {
//     ($name:ident for $($const_type:ty)|+, |$($param:ident),+| $check:expr) => {
//         macro_rules! create_check_for_ctype {
//             ($ctype:ty) => {
//                 pub enum $name<const $($param)*: $ctype> {}
//             }
//         }
//
//         $(
//             create_check_for_ctype!($const_type);
//         )*
//
//
//     };
//
//     // (@internal $name:ident, $($ctype:ty) +, $constants:tt, $params:tt, $check:expr) => {
//     //
//     // }
// }

// define_check!(Equals for i32, (A, B) => A == B);



#[test]
fn test_condition() {
    define_check!(Hello for i32 | bool, |A| true);
}