# Chapter 2 Notes - Programming a Guessing Game

# Variables

```rust
let x = 5;      // Immutable by default
let mut y = 6;  // `mut` keyword to make them mutable.

let s = String::new("Hello");  // ::new is an associated function, not a method.

func(&x);     // Function which accepts an immutable reference.
func(&mut y)  // Fucntion which accepts a muttable reference.

// When a `Result` is returned, the .expect method can be used to handle
// errors. .expect causes program to error out and crash.
func(&mut y)
.expect("Error message")
// If .expect is not used when a `Result` is returned, a compiler warning is given.

// Interpolation can be inline, or in-tuple.
println!("x = {x} and y + 2 = {}", y + 2);
// Result: x = 5 and y + 2 = 12
```

# Installing Crates

Add deps to `Cargo.toml:[dependencies]`, semver applies.
`Cargo.lock` works like lockfiles do for any other lang.

`cargo build` will install any missing deps.
`cargo update` will update any deps, matching constraints in the toml.
