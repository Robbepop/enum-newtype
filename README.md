# `#[enum_newtype]` Proc. Macro Attribute

⚠️ DISCLAIMER: DO NOT USE THIS IN PRODUCTION CODE! ⚠️

This crate provides a proc. macro `#[enum_newtype]` that can be applied on to Rust `enum` types
and change their internal structure so that all their variants become newtype wrappers around newly
generated structs representing the variants.

This has the following benefits:

- It is possible to refer to `enum` variants by their type.
- This potentially makes a big `enum` type more modular breaking it up into smaller pieces.
- In some cases `enum` pattern matching can become simpler and more modular again.

However, during implementation of this crate we found several drawbacks:

- Restructuring the Rust `enum` introduces confusing programming experience
  since the underlying structure does not match the structure shown on the display.
- Unfortunately as of today the `rust-analyer` IDE plugin is equally confused about
  the generated code and will not be able to provide help when working on the generated
  `enum` type and its variant types.

Due to the drawbacks mentioned above I do not recommend using this proc. macro or anything similar.
Even if it means more work up front it is recommended to properly spell out your types instead of generating them.
This might change if the `rust-analyzer` IDE plugin can properly help out with the rough edges of
the generated code.
