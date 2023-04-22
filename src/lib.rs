#![no_std]
#![feature(generic_const_exprs)]

pub enum Check<const EXPRESSION: bool> {}

pub trait Passed {}
pub trait Failed {}

impl Passed for Check<true> {}
impl Failed for Check<false> {}

macro_rules! check_module {
    ($($mod:ident)*; $($name:ident: |$($param:ident),+| $check:expr)*) => {
        macro_rules! apply_module {
            ($m:ident) => {

                pub mod $m {
                    $(

                        pub enum $name<$(const $param: $m,)*> {}

                        impl<$(const $param: $m,)*> $name<$($param,)*> {
                            /// Rust const generic evaluation is strange
                            ///
                            /// ----------------------------------
                            /// Check<{ A && B }> is Too complex
                            /// ----------------------------------
                            ///
                            /// ----------------------------------
                            /// const fn not_complex<const A: bool, const B: bool>() -> bool {
                            ///     A && B
                            /// }
                            /// Check<{ not_complex::<A, B> } is Simple
                            /// ----------------------------------
                            ///
                            const fn simplify() -> bool { $check }
                        }

                        macro_rules! impl_result {
                            ($result:ident) => {
                                impl<$(const $param: $m,)*> $crate::$result for $name<$($param,)*>
                                    where $crate::Check<{ $name::<$($param,)*>::simplify() }>: $crate::$result {}
                            }
                        }

                        impl_result!(Passed);
                        impl_result!(Failed);
                    )*
                }
            }
        }

        $(
            apply_module!($mod);
        )*
    };
}

check_module!(i8 i16 i32 i64;
    Equals: |A, B| A == B
    Positive: |T| T > 0
    Negative: |T| T < 0
    Zero: |T| T == 0
);

check_module!(bool;
    Conjuct: |A, B| A && B
    Disjunct: |A, B| A || B
);

mod tests {
    use super::{Check, Failed, Passed};

    struct Something<const T: i32>
    where
        crate::i32::Positive<T>: Passed;

    fn tester() {
        let works = Something::<5>;
        // let doesnt_work = Something::<-1>;
    }
}
