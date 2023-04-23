# checks
Adds several compile time checks for const generics.

Idea was based off of [this reddit post](https://www.reddit.com/r/rust/comments/gt067a/implementing_a_trait_only_for_types_satisfying_a/).

## Examples:
```rust
#![feature(generic_const_exprs)]

struct AlphabeticTest<const C: char>
	where checks::char::Alphabetic<C>: checks::Passed;

// All letters are alphabetic, check passes.
let pass = AlphabeticTest::<'a'>; // Compiles
let pass = AlphabeticTest::<'b'>; // Compiles
let pass = AlphabeticTest::<'c'>; // Compiles

// Letters are not alphabetic, check fails.
let fail = AlphabeticTest::<'1'>; // Compile error!
let fail = AlphabeticTest::<'2'>; // Compile error!
let fail = AlphabeticTest::<'3'>; // Compile error!
```

Custom checks can be implemented as well:
```rust
const fn is_binary(num: u8) -> bool { num == 0 || num == 1 }
struct BinaryOnly<const N: u8> where
    Check<{ is_binary(N) }>: Passed;

let pass = BinaryOnly::<0>; // Compiles
let pass = BinaryOnly::<1>; // Compiles

let fail = BinaryOnly::<2>; // Compiler error!
```

Checks can also be defined with the `check!` macro:
```rust
check! { u8 =>
    /// Matches all binary numbers (0, 1)
    Binary(
        passes: 0, 1
        fails: 2, 3, 4
    ): |N| (N == 0 || N == 1)
}
```

Full list of checks can be found on the [docs.rs page](https://docs.rs/checks/latest).