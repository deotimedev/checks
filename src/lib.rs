#![no_std]
#![feature(generic_const_exprs)]

pub enum Check<const E: bool> {}

pub trait Conclusion<const R: bool> {}

pub trait Passed {}
pub trait Failed {}

impl<T: Conclusion<true>> Passed for T {}
impl<T: Conclusion<false>> Failed for T {}

// Theoretically, all custom checks could just be represented as a
// type alias of `Check`, however currently the compiler complains
// that it would be an unconstrained bound
macro_rules! check_module {
    ($($mod:ident)* => $($(#[doc = $doc:expr])?$name:ident$((passes: $($($pass:literal);*),* fails: $($($fail:literal);*),*))?: |$($param:ident),+| $check:expr)*) => {
        macro_rules! apply_module {
            ($m:ident) => {

                pub mod $m {
                    $(

                        $(#[doc = $doc])*
                        /// ```no_run
                        /// #![feature(generic_const_exprs)]
                        #[doc = concat!("use checks::{ Passed, ", stringify!($m), "::* };")]
                        ///
                        // TODO: fix ugly trailing comma here
                        #[doc = concat!("struct ", stringify!($name), "Test<", $(concat!("const ", stringify!($param), ": ", stringify!($m), ", ")),*, ">")]
                        #[doc = concat!("\twhere ", stringify!($name), "<", stringify!($($param),*), ">: Passed;")]
                        ///
                        $($(
                            #[doc = concat!("let works = ", stringify!($name),"Test::<", stringify!($($pass),*), ">; // Success!")]
                        )*)*
                        /// ```

                        /// ```compile_fail
                        $($(
                            #[doc = concat!("let doesnt_work = ", stringify!($name),"Test::<", stringify!($($fail),*), ">; // Compile error!")]
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

        $(apply_module!($mod);)*
    };
}

macro_rules! check_number_module {
    ($($mod:ident)*$( => $($extra:tt)*)?) => {
        check_module!($($mod) * =>
            /// Matches when LHS equals RHS.
            Equals(
                passes: 1;1, 5;5
                fails: 1;10, 3;2
            ): |A, B| A == B

            /// Matches when number is zero.
            Zero(
                passes: 0
                fails: 1, 2, 3
            ): |T| T == 0

            /// Matches when number is even.
            Even(
                passes: 2, 4, 6
                fails: 1, 3, 5
            ): |T| T % 2 == 0

            /// Matches when number is odd.
            Odd(
                passes: 1, 3, 5
                fails: 2, 4, 6
            ): |T| T % 2 == 1

            /// Matches when LHS is less than RHS.
            Less(
                passes: 1;3, 5;10
                fails: 5;3, 7;7
            ): |A, B| A < B

            /// Matches when LHS is greater than RHS.
            Greater(
                passes: 3;2, 11;5
                fails: 5;10, 6;6
            ): |A, B| A > B

            /// Matches when LHS is less than or equal than RHS.
            LessOrEqual(
                passes: 3;5, 9;9
                fails: 5;2, 9;5
            ): |A, B| A <= B

            /// Matches when LHS is greater than or equal than RHS.
            GreaterOrEqual(
                passes: 9;2, 5;5
                fails: 2;3, 8;4
            ): |A, B| A >= B

            $($($extra)*)*
        );
    }
}

check_number_module!(u8 u16 u32 u64 u128 usize);
check_number_module!(i8 i16 i32 i64 i128 isize =>
    /// Matches when number is above 0.
    Positive(
        passes: 1, 50, 3
        fails: 0, -55, -3
    ): |T| T > 0

    /// Matches when number is below 0.
    Negative(
        passes: -1, -34, -5
        fails: 0, 1, 55
    ): |T| T < 0
);

check_module!(bool =>

    /// Matches when both booleans are true.
    And(
        passes: true;true
        fails: true;false, false;true, false;false
    ): |A, B| A && B

    /// Matches when one boolean is true.
    Or(
        passes: true;false, false;true
        fails: false;false
    ): |A, B| A || B

    /// Matches exclusive-or booleans.
    Xor(
        passes: true;false, false;true
        fails: true;true, false;false
    ): |A, B| A ^ B

    /// Matches the negated form of this boolean.
    Negate(
        passes: false
        fails: true
    ): |T| !T
);

// Todo: Uppercase and Lowercase when const is supported.
// Todo: Maybe emoji check as well 👀?
check_module!(char =>
    /// Matches all ascii characters.
    Ascii(
        passes: 'a', '1', '&', '~'
        fails: '¶', '®', 'ℜ', '∑'
    ): |C| C.is_ascii()

    /// Matches digit characters (0-9).
    Digit(
        passes: '0', '1', '2'
        fails: 'a', 'b', 'c'
    ): |C| C.is_ascii_digit()

    /// Matches all alphabetical characters (a-z).
    Alphabetic(
        passes: 'a', 'b', 'c'
        fails: '1', '2', '3'
    ): |C| C.is_ascii_alphabetic()

    /// Matches all alphanumeric characters (a-z, 0-9).
    Alphanumeric(
        passes: 'a', '1', 'z', '9'
        fails: '&', '$', '-'
    ): |C| C.is_ascii_alphanumeric()

    /// Matches blank character (' ').
    Blank(
        passes: ' '
        fails: 'a', '1'
    ): |C| C.is_ascii_whitespace()
);
