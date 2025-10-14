# Chapter 3 - Common Programming Concepts

## 3.1 - Variables and Mutability

Variables are immutable by default. Use `mut` keyword to make mutable.

Constants are immutable, cannot be made mutable, and must have a type annotation. They cannot be assigned at runtime.

`const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;`

Shadowing allows for redefining variables. Shadows are scoped to their block. Shadowing is reuse of the same variable name, the type can be changed.

```rust
let x = 5;
let x = x + 1;  // Shadow of x, equals 6.

{  // A new block.
    let x = x * 2;
    println!("{x}");  // Will print 12, old shadow * 2.
}

println!("{x}");  // Will print 6, the current shadow in scope.
```

## 3.2 - Data Types

Scalars are types which represent a single value. Four primary scalars: integers, floating-points, booleans, and characters.

Intergers are default `i32`, When an integer overflows, it wraps around. If a `u8` value is 257, it's 1, and so on.

Floats default to `f64`.

Typical arthemic symbols apply: +, -, *, /, %

Bools are one byte, `true` or `false`.

Compound types are those which hold mutliple value, two types: tuples, arrays.

Tuples are fixed length, multi-type variables: `let tup: (u32, f4) = (400, 40.0);`.
Tuples use dot-notation for access: `tup.0` for 400.
`()` is a special tuple called the *unit*, a valueless tuple, which is returned by expressions automatically if they do not return something.

Arrays are fixed length, of a single type: `let a = [1, 2, 3];`. They are allocated on the stack, not the heap. Arrays use bracket access: `a[0]` for 1. Out of bounds array access will result in runtime error.

## 3.3 - Functions

Rust uses snake_case convention for function names. `fn` is keyword. Rust does not care where a function is defined within a scope. Function defintions are statements, statements do not return values.

```rust
// No use of "return", no semicolon.
fn five() -> i32 {
    5
}

fn explicit_five() -> i32 {
    return 5;
}
```

Expressions return values, such as `5 + 6`. Calling a function is an expression, as is calling a macro. Scope blocks are expressions too. Expressions do not end in a semicolon.

```rust
let y = {
    let x = 5;
    x + 1  // y is assigned to 6. No semicolon.
};

let y = {
    let x = 5;
    x + 1;  // Returns nothing, as the semicolon makes it a statement.
};
```

## 3.4 - Comments
Comments are `//`, on each line of the comment. Documentation comments are discussed in Ch14.

## 3.5 - Control Flow

### If

`if` expressions work like all other langs. `if` statements must eval to a bool, `if 3` is invalid, because Rust does not implicitly cast a positive int to `true`, same with `0`.

```rust
let num = 5;

// If.
if num > 10 {
    // Do somthing.
} else {
    // Other thing.
}

// If, else
if num == 0 {
    // Do somthing.
} else if num > 0 {
    // Other thing.
} // And so on.

// If in an assignment, since if is an expression.
let con = true;
let num = if con { 2 } else { 3 };  // Both must be same time.
```

### Loops

Three types of loops:

- `loop`: Infinite until manually broken.
  - `continue`/`break` apply to the inner most loop in scope.
  - loops can be labeled: `'my_loop: loop { break 'my_loop; }`
- `while`: Infinite until condition is met.
- `for`: Allows for iteration thru a collection, such as an array. `for el in arr {...}`
