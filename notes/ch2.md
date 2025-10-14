# Chapter 2 Notes - Programming a Guessing Game

## Variables

```rust
let x = 5;      // Immutable by default, all inferred number are of type i32.
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

// Redefining a varible is called shadowing.
let mut og = 8;
let g: u32 = 7;
```

## Matching and Exceptions.

Match blocks match on patterns, each possiblity is called an arm. Each arm defines the `Result` and and anon function to handle the result.

```rust
// ...
match guess.cmp(&secret) {
    Ordering::Less => println!("Too small."),
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => {
        println!("Correct");
        break;
    }
}
```

Matching is how exception handling is done.

```rust
// ... loop
    let guess: u32 = match guess.trim().parse() {
        Ok(n) => n,  // In case of Result.Ok, return value.
        // In case of Result.Err, handle
        Err(_) => {  // _ is a catch-all for any error type.
            println!("Not a number, try again.");
            continue; // Start loop over again.
        }
    };
```


## Installing Crates

Add deps to `Cargo.toml:[dependencies]`, semver applies.
`Cargo.lock` works like lockfiles do for any other lang.

`cargo build` will install any missing deps.
`cargo update` will update any deps, matching constraints in the toml.
