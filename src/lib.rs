#![no_std]
#![feature(generic_const_exprs)]

pub enum Check<const E: bool> {}

pub trait Conclusion<const R: bool> {}

// These can be represented by a trait alias to `Conclusion` when supported.
pub trait Passed {}
pub trait Failed {}

impl<T: Conclusion<true>> Passed for T {}
impl<T: Conclusion<false>> Failed for T {}

macro_rules! check_module {
    ($($mod:ident)* => $($name:ident: |$($param:ident),+$(,)?| $check:expr)*) => {
        macro_rules! apply_module {
            ($m:ident) => {

                pub mod $m {
                    $(

                        // Theoretically, all custom checks could just be represented as a
                        // type alias of `Check`, however currently it complains that it would
                        // be an unconstrained bound.
                        pub enum $name<$(const $param: $m,)*> {}

                        impl<$(const $param: $m,)*> $name<$($param,)*> {
                            /// Rust const generic evaluation is strange
                            ///
                            /// ----------------------------------
                            /// Check<{ A && B }> -> Too complex
                            /// ----------------------------------
                            ///
                            /// ----------------------------------
                            /// const fn apparently_not_complex<const A: bool, const B: bool>() -> bool {
                            ///     A && B
                            /// }
                            ///
                            /// Check<{ apparently_not_complex::<A, B> } -> Simple
                            /// ----------------------------------
                            ///
                            const fn simplify() -> bool { $check }
                        }

                        impl<$(const $param: $m,)*> $crate::Conclusion<{ $name::<$($param,)*>::simplify() }> for $name<$($param,)*> {}

                    )*
                }
            }
        }

        $(
            apply_module!($mod);
        )*
    };
}

macro_rules! check_number_module {
    ($($mod:ident)*$(=> $($name:ident: |$($param:ident),+| $check:expr)*)?) => {
        check_module!($($mod)* =>
            Equals: |A, B| A == B
            Zero: |T| T == 0
            Even: |T| T % 2 == 0
            Odd: |T| T % 2 == 1
            $($(
            $name: |$($param,)*| $check
            )*)*
        );
    }

}

check_number_module!(u8 u16 u32 u64 u128 usize);
check_number_module!(i8 i16 i32 i64 i128 isize =>
    Positive: |T| T > 0
    Negative: |T| T < 0
);

check_module!(bool =>
    Conjuct: |A, B| A && B
    Disjunct: |A, B| A || B
    Negate: |T| !T
);

mod tests {
    use super::{Check, Failed, Passed};

    struct Something<const T: i32>
    where
        crate::i32::Positive<T>: Passed;

    #[test]
    fn tester() {
        let test = 55_usize;
        let works = Something::<5>;
        // let doesnt_work = Something::<-1>;
    }
}

#[cfg(any(a))]
fn needs_a() {}
