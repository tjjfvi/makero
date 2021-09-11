By default, macros are processed top-down; for example, `foo!(bar!())` will
process the `foo` macro first, using the literal tokens `bar` `!` `(` `)`, and
`bar` will only be processed if `foo` outputs those tokens verbatim.

Enter `makero`. Inside `makero` blocks, invoked helper macros will be
processed bottom-up; the below `main` macro outputs `true`, but removing
`makero` would cause it to output `false` instead, as the `is_x` macro
would see `make_x` `!` `(` `)` instead of `x`.

```rust
use makero::makero;
makero! {
  macro_rules! main {
    () => { is_x!(make_x!()) };
  }

  macro_rules! is_x {
    (x) => { true };
    ($($x:tt)*) => { false };
  }

  macro_rules! make_x {
    () => { x };
  }
}
let out = main!();
assert_eq!(out, true);
```

The `makero` macro accepts one or more `macro_rules!` items; only the
top-most one will be externally visible.

Attributes can be applied to the resulting macro by applying them to the
top-most `macro_rules!` definition.
