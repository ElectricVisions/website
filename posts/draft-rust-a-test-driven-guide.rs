/*
mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
tags: rust

# Rust: A Test Driven Guide

A quick tour of Rust for the experienced programmer.
I wrote this to help me learn the language as efficiently as possible.
It's not exhaustive, but should allow me to start writing some code.

One thing to note about the style.
I've gone for 2 spaces for indentation due to many years as a Ruby programmer.
This combined with an 80 character line length allows more splits for files in
Neovim.

#### Contents
{{TOC:2-3}}

## Linting with clippy
clippy is a linting tool that checks for common mistakes and provides suggestions

It's a bit strict for our examples so we'll leave it off. But you would enable
like this:
```rust
#![warn(clippy::all, clippy::pedantic)]
```

## [rust-script](https://rust-script.org/)

For basic stuff that doesn't need any crates (libraries) you can just run
`rustc script.rs` and it'll compile a `./script` binary. However, for this
blog I needed something more.

`rust-script` compiles and runs one-off scripts from any folder.
You can also add a crate description to the script to allow additional
dependencies to be included.

```rust
{{../scripts/rust-script-test}}
```

For tests to run we need a main function. This would normally be the
entry point of the program.

## Cargo

For projects, you'll want to use `cargo` which is a build tool and package manager.

`src/main.rs` is the entry point of the default binary.
`target/debug/` is where build artifacts end up in debug mode.
`target/release/` is where build artifacts end up in release mode.

    cargo new proj_name     # Create a new Rust project in proj_name/
    cargo build             # Builds a project in debug mode
    cargo build --release   # Builds a project in release mode
    cargo run               # Runs the default binary
    cargo check             # Checks for errors without building. Fast.

More info at [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/).
*/

fn main() {}

/*
## Macros
Macros are a way to define reusable code. They're similar to functions but
they're expanded by the compiler.
This one creates a nice `refute!` macro that's the opposite of `assert!`.
I'm a Ruby developer and brought this over from the Minitest syntax. I think
it's easier to read than `assert!(!cond)`.
*/
macro_rules! refute {
  ($cond:expr $(,)?) => { assert!(!$cond) };
  ($cond:expr, $($arg:tt)+) => { assert!(!$cond, $($arg)+) };
}

/*
The #[test] **attribute** marks a function as a test. It'll get picked up
 by the test runner and run when you run `cargo test` (or `cargo t`).
*/
#[test]
fn hello_world() {           // This is a function
  println!("Hello, world!"); // Run with cargo t -- --nocapture to see output
  assert_eq!(1, 1);          // Asserts that the left and right values are equal

  // Ordering of assertion values is not important
  let left = "value";  // When values don't match it'll output the "left" and
  let right = "value"; // "right" values so it's easy to identify which is which.
  assert_eq!(left, right);
}

/*
The `should_panic` attribute tells the test runner to expect a panic.
If it doesn't panic then the test will fail. Watch when you run `cargo t -- --nocapture`. You'll
see it panics but all the tests pass.
A panic is like an exception in other languages. However Rust uses the Result
type to indicate success or failure.
*/
#[test]
#[should_panic(expected = "assertion `left == right` failed\n  left: 1\n right: 2")]
fn failing_test() {
  assert_eq!(1, 2);
}

/*
## Variables
Variables are immutable by default. Use `mut` to make them mutable.
*/
#[test]
fn variables() {
  let x = 2;                    // An immutable variable
  let mut y = 5;                // A mutable variable
  y += x;
  assert_eq!(y, 7);

  const A_CONST: f32 = 1.2;     // Constants cannot be marked as `mut`able, must
                                // have a type, can be declared in any scope and
                                // may only be set to a constant expression.
  let z = 1.2;
  assert_eq!(A_CONST, z);

}

/*
Variables may be shadowed, as in, the same name can be used in the same or
nested scope and it'll override the one previously defined, allowing variable
name reuse.
*/
#[test]
fn variable_shadowing() {
  let a = 1;
  assert_eq!(a, 1);
  let a = 2;                    // Second declaration shadows the first
  assert_eq!(a, 2);             // Useful for reusing names.
}

/*
## Basic Types
*/
#[test]
fn basic_types() {
  // isize and usize are architecture-dependent and used for collection indexing.
  let int8: i8 = -1;       // Signed integers are i8, i16, i32, i64, i128, isize
  let unsigned8: u8 = 255; // Unsigned integers are u8, u16, u32, u64, u128, usize
  let float32 = 1.0;       // Floats f32 by default
  let float64: f64 = 1.0;
  let default32 = 1;       // Default type is i32
  let ch = 'a';            // char type, supports ASCII and Unicode
  let imoji = '😻';        // A unicode char type
  let t = true;            // Boolean type
  let f: bool = false;     // Boolean type with explicit type
  let unit = ();           // The unit type another one pulled from functional languages

  // TODO: Reorder these asserts to match the order of the types above.
  assert!(t);
  refute!(f);              // The refute! macro from above

  assert_eq!(int8, -1);
  assert_eq!(unsigned8, 255);
  assert!((float64 - float32).abs() < f64::EPSILON); // Roughly equal
  assert_eq!(default32, 1);
  assert_eq!(ch, 'a');

  let tuple = (1, "hello", 3);      // Tuples can contain any type
  let (one, hello, three) = tuple;  // They can be destructured like so

  assert_eq!(one, 1);
  assert_eq!(hello, "hello");
  assert_eq!(three, 3);
  assert_eq!("hello", tuple.1);   // Or accessed by index

  assert_eq!(0xff, 255);          // Hexadecimal literals are prefixed with 0x
  assert_eq!(0o77, 63);           // Octal literals are prefixed with 0o
  assert_eq!(0b1111_1111, 255);   // Binary literals are prefixed with 0b
  assert_eq!(b'A', 65);           // Byte literals are prefixed with b
  assert_eq!(1_000_000, 1000000); // Underscores can be used for clarity

  // Note: Overflow checks are not done in release builds.
}

/*
## Operators

See [Rust operators](https://doc.rust-lang.org/book/appendix-02-operators.html)
for the full list.
*/
#[test]
fn operators() {
  assert_eq!(1 + 1, 2);           // Addition
  assert_eq!(3 - 1, 2);           // Subtraction
  assert_eq!(2 * 3, 6);           // Multiplication
  assert_eq!(6 / 2, 3);           // Division
  assert_eq!(6 % 2, 0);           // Remainder/Modulo
}

/*
## Strings
*/
#[test]
fn strings() {
  let str: &str = "Hello this is a str type"; // fixed size, immutable, borrowed reference
  let string: String = str.to_string().replace("str", "String"); // Heap allocated, mutable, growable, owned

  assert_eq!(str, "Hello this is a str type");
  assert_eq!(string, "Hello this is a String type");

  assert_eq!(&str[..5], "Hello");   // Slices are a way to get a reference to a part of a string
  assert_eq!(&string[..5], "Hello");
}

/*
## Arrays
*/
#[test]
fn arrays() {
  let a = [1, 2, 3];              // Arrays are fixed-size, same-type values
  assert_eq!(a[1], 2);

  let b: [i32; 3] = [1, 2, 3];    // Array type can be specified
  assert_eq!(b[1], 2);

  let c = [3; 5];                 // Array with a fixed size can be initialized
  assert_eq!(c, [3, 3, 3, 3, 3]);
  assert_eq!(c[2..4], [3, 3]);    // Array slices can be created with from_index..to_index same as string slices
}

/*
## Structs
Structs are a way to group data together. They can be used to create
custom types. You can also define methods on them.
*/
struct User {
  name: String,
  email: String,
  active: bool,
}

/*
`impl` allows us to add methods to a struct.
This is a method on the User struct
Constructors (new) are a way to create a new instance of a struct
*/
impl User {
  fn new(name: &str, email: &str, active: bool) -> Self {
    Self { name: name.to_string(), email: email.to_string(), active }
  }
}

#[test]
fn structs() {
  let user = User::new("Joe", "joe@example.com", true);

  assert_eq!(user.name, "Joe");
  assert_eq!(user.email, "joe@example.com");
  assert!(user.active);
}

/*
## Functions
This is a function, just like our `main()` function at the start.
Functions must specify their parameter and return types (if any).

Notice the lack of semicolons at the end of the string literals.
Rust has implicit returns. This is indicating it'll return a `&str` type.
If we had a semicolon it would return a `()` type.
*/
fn conditional_msg(value: u32) -> &'static str {
  if value < 5 {
    "less than 5"
  } else {
    "greater than 5"
  }
}

/*
## Control Flow
*/
#[test]
fn control_flow() {
  // if expressions
  assert_eq!("less than 5", conditional_msg(4));    // function from above
  assert_eq!("greater than 5", conditional_msg(7));

  let mut x = if true { 1 } else { 2 }; // If expressions
  assert_eq!(x, 1);

  // loop
  let result = loop {               // Loop forever!
    if x < 2 { x += 1; continue; }  // Skip to the next iteration
    break x;                        // Let's break out of the loop
  };
  assert_eq!(result, 2);

  // while
  let mut number = 5;
  while number != 0 {
    number -= 1;
  }
  assert_eq!(number, 0);

  // match
  let value = 4;
  let result =
    match value {
      1 => "One!",
      2 | 3 => "Two or three!",
      a if a < 10 => "Single digit",
      _ => "Some other value",
    };
  assert_eq!(result, "Single digit");
}

/*
## Enums
Enums are like Unions types in functional languages.
*/
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

fn which_enum(msg: Message) -> String {
  match msg {
    Message::Quit => "Quit".to_string(),
    Message::Move { x, y } => format!("Move {} {}", x, y),
    Message::Write(text) => format!("Write {}", text),
    Message::ChangeColor(r, g, b) => format!("ChangeColor {} {} {}", r, g, b),
    // Enums don't require a catch-all arm (_ -> ...) but you must handle all values if you don't
  }
}

#[test]
fn enums() {
  assert_eq!(which_enum(Message::Write(String::from("Hello"))), "Write Hello");
  assert_eq!(which_enum(Message::Move { x: 1, y: 2 }), "Move 1 2");
  assert_eq!(which_enum(Message::ChangeColor(1, 2, 3)), "ChangeColor 1 2 3");
  assert_eq!(which_enum(Message::Quit), "Quit");
}

/*
## Traits
Traits are a way to define shared behavior for types. They're similar to
interfaces in other languages.

The Copy trait is used to make a type copyable. This is useful for types
that are expensive to copy, like Strings.

[TODO: Not finished]
*/
trait SomeTrait {
  fn copy(&self) -> Self;
}

#[test]
fn copy_trait() {
  let x = 5;
  let y = x;
  assert_eq!(x, y);
}

/*
## Borrowing
Borrowing is a way to share data without copying it. This is useful for
performance and memory management. The compiler will enforce that you
don't have multiple references to the same data. It also means you don't
have to worry about freeing memory.

Add traits Copy & Clone and as long as the types are all Copy the whole struct can be copied.
Debug and PartialEq traits are so we can use assert_eq! to compare structs.
*/
#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

#[test]
fn borrowing() {
  let p1 = Point { x: 1, y: 2 };
  let p2 = p1; // p1 is copied to p2 and both are still valid

  assert_eq!(p1, Point {x: 1, y: 2});
  assert_eq!(p2, Point {x: 1, y: 2});

  let s = String::from("Hello");
  let r1 = &s;  // Immutable reference to s
  let r2 = &s;  // Second immutable reference to s OK
  //let r3 = &mut s; // Cannot borrow as mutable because there are immutable references

  assert_eq!(r1, "Hello");
  assert_eq!(r2, "Hello");

  let mut s2 = String::from("There"); // Needs to be mutable to borrow as mutable
  let r3 = &mut s2;                   // No other references to so mutable borrow is OK
  assert_eq!(r3, "There");

  *r3 = String::from("World");
  assert_eq!(r3, "World");
}

/*
## Lifetimes
Lifetimes are a way to specify how long a reference is valid. This is
useful for functions that return references to data. It also means that
you can't accidentally return a reference to data that will go out of
scope.
*/
#[allow(clippy::needless_lifetimes)]
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) { // These lifetimes can be inferred
  println!("x is {}, y is {}", x, y);           // but shown to explain how they work.
}

#[allow(clippy::extra_unused_lifetimes)]
fn failed_borrow<'a>() {
  let _x = 12;
  // let _y: &'a i32 = &_x; // This would fail because x's lifetime will end at the
  // end of the function, but 'a lifetime is determined by the caller of the
  // function which will be longer than x's.
}

#[test]
fn lifetimes() {
  let (i, j) = (4, 9);
  print_refs(&i, &j);
  failed_borrow();
}

/*
## Error Handling
As opposed to exceptions or return codes some languages, errors are handled
with the Result type. This is a type that can either be Ok(value) or Err(error).
*/
fn divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
    Err("Div by zero".to_string())
  } else {
    Ok(a / b)
  }
}

/*
Result types can be used in match expressions to handle errors.
*/
fn handle_divide(a: f64, b: f64) -> String {
  match divide(a, b) {
    Err(msg) => msg,
    Ok(answer) => format!("The answer is {}.", answer),
  }
}

#[test]
fn error_handling() {
  assert_eq!(handle_divide(10.0, 0.0), "Div by zero");
  assert_eq!(handle_divide(10.0, 2.0), "The answer is 5.");
}

/*
## Closures
*/
#[test]
fn closures() {
  let mut haystack = vec![1, 2];
  haystack.push(3);
  let contains = move |needle| haystack.contains(needle);

  // haystack is now owned by the closure and cannot be used outside of it

  assert!(contains(&1));
  refute!(contains(&4));
}

/*
## Iterators

## Standard Library

### Reading directories and files
*/
