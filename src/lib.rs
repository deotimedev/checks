#![feature(generic_const_exprs)]
#![feature(trait_alias)]


pub trait Check<const EXPRESSION: bool> {}

pub trait Passed = Check<true>;
pub trait Failed = Check<false>;

pub enum Condition<const CONDITION: bool> {}
impl<const CONDITION: bool> Check<CONDITION> for Condition<CONDITION> {}

pub enum Equals<const A: i32, const B: i32> {}
impl<const A: i32, const B: i32> Check<{ A == B }> for Equals<A, B> {}

struct Test<const T: i32>(i32)
    where Equals<T, 2>: Passed;

// #[macro_export]
// macro_rules! define_check {
//     ($name:ident for $cc:ident, ($($param:ident)+) => $check:expr) => {
//         $(
//         pub trait $name = Check<$($param),*>;
//         )*
//
//     }
// }



#[test]
fn test_condition() {

}