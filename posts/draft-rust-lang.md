mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
tags: rust


# Rust: A guide for experienced programmers

A whirlwind tour of the syntax and key features for the experienced programmer.
I wrote this for me.
Hopefully others will find it useful.

Bevy is a 2D and 3D game engine for Rust with complementary ECS to boot.
I wanted to look into using Bevy to re-ignite my game development journey.
For that, I needed to learn Rust.
So here is a guide to getting started with the language.
It's aimed at experienced developers.
I'm going for concise code, not necessarily well formatted.

#### Contents
{{TOC:2-3}}

## Hello World!

```rust
// main.rs
fn main() { println!("Hello, world!"); } // Minimal Hello, world program.

rustc main.rs
./main
Hello, world!
```

## Cargo

`src/main.rs` is the entry point of the default binary.
`target/debug/` is where build artifacts end up in debug mode.
`target/release/` is where build artifacts end up in release mode.

    cargo new proj_name     # Create a new Rust project in proj_name/
    cargo build             # Builds a project in debug mode
    cargo build --release   # Builds a project in release mode
    cargo run               # Runs the default binary
    cargo check             # Checks for errors without building. Fast.

More info at https://doc.rust-lang.org/cargo/.

## Variables

```rust
let x = 5;                // Defines an immutable variable, x (32-bit int)
let mut y: i64 = 6;       // A mutable variable, y (64-bit int)
y = 7;                    // Change the value to 7
const A_CONST: u32 = 10;  // Define a constant with a value of 10
let z = 1.2;              // A 64 bit float
```

Constants
* Cannot be marked as `mut`able.
* Must be annotated with a type (e.g. 32-bit unsigned integer as above)
* Can be declared in any scope
* May only be set to a constant expression (they cannot be computed at runtime)

Variables may be shadowed, as in, the same name can be used in in the same or
nested scope and it'll override the one previously defined, allowing variable
name reuse.

## Types

### Integers
Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

The number indicates the number of bits.
`isize` and `usize` means architecture dependent.
Generally used for collection indexes.

Literals are generally prefixed to denote the base.
Decimal with no prefix, hex: 0xff, octal: 0o77, binary: 0b1111_0101,
byte: b'A'. `_` can be used as a separator.

Note: Overflow checks are not done in release builds.

### Floats
Two types, `f32` and `f64`.

```rust
let x = 5.0;      // f64
let y: f32 = 3.0; // f32
```

### Operations

Usual operators: `+`, `-`, `*`, `/`, `%`. The last being modulo or remainder
There are many more: https://doc.rust-lang.org/book/appendix-02-operators.html.

### Boolean

```rust
  let t = true;
  let f: bool = false;
```

### Char

```rust
let c = 'z';
let h: char = 'Z';
let imoji = '😻';
```

### Tuple

```rust
let tup = (12, 3.4, 23); // Different types
let (x, y, z) = tup;
let last_one = tup.2;
```

### Array

```rust
let a = [1, 2, 3];      // Same types, length is fixed
let b: [i32, 5] = [1, 2, 3, 4, 5];
let c = [3; 5];         // 5 elements all set to the value 3
let first_item = a[0];
```

## Functions

Parameter types and return types must be specified in function signatures.

```rust
fn function_name(param: i32) -> i32 {  // types must be specified
  param + 1                     // returns are implicit and no ; is needed
}
```

## Control Flow

### if

```rust
let number = 4;
if number < 5 {
  // This branch will run
} else if number < 10 {
  //...
} else {
  //...
}

let number = if condition { ... } else { ... }; // `if` is an expression
```

### loop

```rust
let result = loop {
  // Loop forever!
  break 1; // Actually don't! You can also use `continue` to skip an iteration
}; // result => 1
```

### while

```rust
while number != 0 {
  //...
}
```

### for

```rust
let a = [1, 2, 3, 4];

for i in a {
  println!("Value is: {i}");
}
```

### match

I'm familiar with this one from F#. It works pretty much the same way.

```rust
let value = 4;
match value {
  1 => println!("One!"),
  2 | 3 => println!("Two or three!"),
  a if value < 10 => println!("Single digit: {}", a),
  _ => println!("Some other value"),
}
```


## Borrowing

When passing parameters to functions they can be copied (most primitive types),
moved (most other types) or borrowed (immutably or mutably).
To "borrow" a variable you pass it by reference (`&some_var`).

Most types are moved by default.
Types that are copied by default have the Copy trait
e.g. primitive types such as integers, floats, booleans, chars.
Arrays & tuples of Copy types are also Copy by default.

```rust
#[derive(Copy, Clone)] // <--- Make Point a Copy type
struct Point {         // As it's types are all Copy this works
  x: f32,              // <- A Copy type
  y: f64,              // <- A Copy type
}
```

`str` is not a Copy type.
Use `&str` to pass a reference which is an immutable borrow.

### Rules

One mutable OR multiple immutable references at a time.


## Error Handling

No exceptions in Rust.
You must handle ALL errors.
Again this is quite common in functional languages.
It's called Railway programming in some circles.
Using the Result type.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
    Err("Div by zero".to_string())
  } else {
    Ok(a / b)
  }
}

match divide(10.0, 0.0) {
  Err(msg) => println!("{}", msg),
  Ok(answer) => println!("The answer is {}.", answer),
}
```


## Iterators

## Standard Library

### Reading directories and files
