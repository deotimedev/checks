#![no_std]
#![feature(generic_const_exprs)]

pub enum Check<const E: bool> {}

pub trait Conclusion<const R: bool> {}

// These can be represented by a trait alias to `Conclusion` when supported.
pub trait Passed {}
pub trait Failed {}

impl<T: Conclusion<true>> Passed for T {}
impl<T: Conclusion<false>> Failed for T {}

// Theoretically, all custom checks could just be represented as a
// type alias of `Check`, however currently it complains that it would
// be an unconstrained bound.
macro_rules! check_module {
    ($($mod:ident)* => $($(#[doc = $doc:expr])?$name:ident$((passes: $($($pass:expr),*);*))?: |$($param:ident),+$(,)?| $check:expr)*) => {
        macro_rules! apply_module {
            ($m:ident) => {

                pub mod $m {
                    $(
                        $(#[doc = $doc])*
                        /// ```no_run
                        #[doc = concat!("use checks::{ Passed, ", stringify!($m), "::* };")]
                        #[doc = concat!("struct Test<", $(concat!("const ", stringify!($param), ": ", stringify!($m), ", ")),*, ">")]
                        #[doc = concat!("\twhere ", stringify!($name), "<", stringify!($($param),*), ">: Passed;")]
                        ///
                        /// // All compile
                        $($(
                            #[doc = concat!("let works = Test::<", stringify!($($pass),*), ">;")]
                        )*)*
                        /// ```
                        pub enum $name<$(const $param: $m,)*> {}

                        impl<$(const $param: $m),*> $name<$($param),*> {
                            pub const fn simplify() -> bool { $check }
                        }

                        impl<$(const $param: $m),*> $crate::Conclusion<{ $name::<$($param),*>::simplify() }> for $name<$($param),*> {}
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
            Less: |A, B| A < B
            Greater: |A, B| A > B
            LessOrEqual: |A, B| A <= B
            GreaterOrEqual: |A, B| A >= B
            $($(
            $name: |$($param,)*| $check
            )*)*
        );
    }

}

// check_number_module!(u8 u16 u32 u64 u128 usize);
// check_number_module!(i8 i16 i32 i64 i128 isize =>
//     Positive: |T| T > 0
//     Negative: |T| T < 0
// );

check_module!(bool =>
    And: |A, B| A && B
    Or: |A, B| A || B
    Xor: |A, B| A ^ B
    Negate: |T| !T
);

// Todo: Uppercase and Lowercase when const is supported.
// Todo: Maybe emoji check as well 👀?
check_module!(char =>
    /// So what this is is glitchy
    Ascii: |C| C.is_ascii()
    Digit: |C| C.is_ascii_digit()
    Alphabetic(
        passes: 'a'; 'b'; 'b'; 'c'
    ): |C| C.is_ascii_alphabetic()
    Alphanumeric: |C| C.is_ascii_alphanumeric()
    Blank: |C| C.is_ascii_whitespace()
);

// mod tests {
//     use super::{Check, Failed, Passed};
//
//     struct PositiveOnly<const T: i32>
//     where
//         crate::i32::Positive<T>: Passed;
//
//     struct AsciiOnly<const C: char>
//     where
//         crate::char::Ascii<C>: Passed;
//
//     fn positive_test() {
//         crate::i32::Positive::<5>::passes();
//         // let doesnt_work = PositiveOnly::<-1>;
//     }
//
//     fn ascii_test() {
//         let works = AsciiOnly::<'a'>;
//         // let doesnt_work = AsciiOnly::<'🦀'>;
//     }
// }