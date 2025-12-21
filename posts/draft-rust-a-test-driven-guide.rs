/**
mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
tags: rust

# Rust: A Test Driven Guide

A tour of Rust for the experienced programmer driven by tests.
We're all writing tests, right?!
I wrote this to help me learn the language as efficiently as possible.
It's not exhaustive (but it is long!).
It should allow me to start writing some code.
It'll also serve as a reference.

One thing to note about the style.
I've gone for 2 spaces for indentation due to many years as a Ruby programmer.
This combined with an 80 character line length allows more splits for files in
Neovim.

#### Contents
{{TOC:2-3}}

## Linting with clippy
clippy is a linting tool that checks for common mistakes and provides
suggestions.
Some Cargo (introduced next) settings I use for building this blog.

```rust
// Cargo.toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
missing_panics_doc = "allow"
must_use_candidate = "allow"
```

You can also apply the above clippy workspace-wide
```toml
// Cargo.toml (workspace):
[workspace.lints.clippy]
all = "warn"
pedantic = "warn"

// member/Cargo.toml:
[lints]
workspace = true
```

It's too strict for this guide so we'll leave it off.
However, pedantic mode is generally recommended as it allows you to
catch potential issues earlier.

[Clippy docs](https://doc.rust-lang.org/clippy/)

## Cargo

For projects, you'll want to use `cargo` which is a build tool and package manager.

`src/main.rs` is the entry point of the default binary.
`target/debug/` is where build artifacts end up in debug mode.
`target/release/` is where build artifacts end up in release mode.

```bash
cargo new proj_name     # Create a new Rust project in proj_name/
cargo build             # Builds a project in debug mode
cargo build --release   # Builds a project in release mode
cargo run               # Runs the default binary
cargo check             # Checks for errors without building. Fast.
```

More info:
[Cargo docs](https://doc.rust-lang.org/cargo/)

## [rust-script](https://rust-script.org/)

For basic stuff that doesn't need any crates (libraries) you can run
`rustc script.rs` and it'll compile a `./script` binary. But if you want to add
add a few crates..

`rust-script` compiles and runs one-off scripts from any folder.
You can also add a crate description to the script to allow additional
dependencies to be included.

```rust
{{../scripts/rust-script-test}}
```

`rust-script`'s wrap the script in a `main()` function unless you specify one.
Our tests need to run at the module level so we'll need to add a `main()`.
*/

fn main() {}

/**
## Macros

Macros are a way to define reusable code. They're similar to functions but
they're expanded by the compiler.
This one creates a nice `refute!` macro that's the opposite of `assert!`.
I'm a Ruby developer and brought this over from the Minitest syntax. I think
it's easier to read than `assert!(!cond)`. I won't go into macro syntax here
(perhaps a future post).
*/

macro_rules! refute {
  ($cond:expr $(,)?) => { assert!(!$cond) };
  ($cond:expr, $($arg:tt)+) => { assert!(!$cond, $($arg)+) };
}

/**
## Attributes

The `#[test]` *attribute* marks a function as a test. It'll get picked up
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

/**
The `should_panic` attribute tells the test runner to expect a panic.
If it doesn't panic then the test will fail. Watch when you run `cargo t -- --nocapture`. You'll
see it panics but all the tests pass.
A panic is like an exception in other languages. However Rust uses the Result
type to indicate success or failure (See [Error Handling][errorhandling]).
*/
#[test]
#[should_panic(expected = "assertion `left == right` failed\n  left: 1\n right: 2")]
fn failing_test() {
  assert_eq!(1, 2);
}

/**
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

/**
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

/**
## Basic Types

[Docs](https://doc.rust-lang.org/book/ch03-02-data-types.html)

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

  assert_eq!(int8, -1);
  assert_eq!(unsigned8, 255);
  assert!((float64 - float32).abs() < f64::EPSILON); // Roughly equal
  assert_eq!(default32, 1);
  assert_eq!(ch, 'a');
  assert_eq!(imoji, '😻');
  assert!(t);
  refute!(f);              // The refute! macro from above
  assert_eq!(unit, ());

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

/**
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

/**
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

/**
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

  // Array slices can be created with from_index..to_index same as string slices
  assert_eq!(c[2..4], [3, 3]);
}

/**
## Functions

This is a function, just like our `main()` function at the start.
Functions must specify their parameter and return types (if any).

Notice the lack of semicolons at the end of the string literals.
Rust has implicit returns.
This is indicating it'll return a `&str` type.
If we had a semicolon it would return the unit type, `()`.
*/

fn conditional_msg(value: u32) -> &'static str {
  if value < 5 {
    "less than 5"
  } else {
    "greater than 5"
  }
}

/**
## Structs

Structs are a way to group data together. They can be used to create
custom types. You can also define methods on them. Methods are functions
that are associated with a struct.
*/

struct Rect {
  width: u32,
  height: u32,
}

/**

### Associated Functions

`impl` allows us to add associated functions to a struct.
`new` and `square` are constructors. They return an instance of Rect.
`area` is a method because it's first parameter is `self`.
`Self` is an alias for the struct type, in this case `Rect`.
*/

impl Rect {
  fn new(width: u32, height: u32) -> Self {
    Self { width, height } // Shorthand for Self { width: width, height: height }
  }

  fn square(size: u32) -> Self {
    Self { width: size, height: size }
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }
}

#[test]
fn structs() {
  let rect = Rect::new(10, 20);

  assert_eq!(rect.width, 10);
  assert_eq!(rect.height, 20);
  assert_eq!(rect.area(), 200);

  let square = Rect::square(10);
  assert_eq!(square.width, 10);
  assert_eq!(square.height, 10);
  assert_eq!(square.area(), 100);
}

/**
## Enums

Enums are like Unions types in functional languages.
Great for pattern matching.
And you can add data to them.
You can't compare enums directly, instead, use `matches!`.
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
    // _ => "Some other value", // Use this if you don't want to handle all values
  }
}

#[test]
fn enums() {
  assert_eq!(which_enum(Message::Write(String::from("Hello"))), "Write Hello");
  assert_eq!(which_enum(Message::Move { x: 1, y: 2 }), "Move 1 2");
  assert_eq!(which_enum(Message::ChangeColor(1, 2, 3)), "ChangeColor 1 2 3");
  assert_eq!(which_enum(Message::Quit), "Quit");

  let msg = Message::Write(String::from("Hello"));
  assert!(matches!(msg, Message::Write(_)));
}

/**
## Type Aliases

Type aliases are a way to give a type a new name.
Here, the first one gives a name to a tuple.
The second, assigns a more convenient name to an enum.
*/

type MyPoint = (i32, i32);
enum VeryLongEnumNameForDoingStuffWithNumbers { Add, Subtract }
type Operations = VeryLongEnumNameForDoingStuffWithNumbers;

#[test]
fn type_aliases() {
  let p: MyPoint = (1, 2);
  assert_eq!(p, (1, 2));

  assert!(matches!(Operations::Add, VeryLongEnumNameForDoingStuffWithNumbers::Add));
}

/**
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

/**
## Error Handling

For simple programs you can use `panic!` to stop execution and print a message.
Setting `RUST_BACKTRACE=1` env var will print a backtrace.
*/
#[test]
#[should_panic(expected = "Crash and burn")]
fn panics() {
  panic!("Crash and burn");
}

/**
As opposed to exceptions or return codes in some languages, errors are handled
with the Result type. This is a type that can either be Ok(value) or Err(error).
*/
fn divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
    Err("Div by zero".to_string())
  } else {
    Ok(a / b)
  }
}

/**
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

/**
## Traits

Traits are a way to define shared behavior for types.
They're similar to interfaces in other languages.
* Unlike interfaces, method names don't collide.
  You specify which trait you are implementing
* You can provide default implementations
* Types can implement multiple traits
*/

trait Describe {
  fn describe(&self) -> String;
}

trait Area {
  fn area(&self) -> f64;
}

struct Circle {
  radius: f64,
}

impl Describe for Circle {
  fn describe(&self) -> String {
    format!("A circle with radius {}", self.radius)
  }
}

impl Area for Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * self.radius * self.radius
  }
}

struct Square {
  side: f64,
}

impl Describe for Square {
  fn describe(&self) -> String {
    format!("A square with side {}", self.side)
  }
}

impl Area for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }
}

#[test]
fn traits() {
  let circle = Circle { radius: 2.0 };
  assert_eq!(circle.describe(), "A circle with radius 2");
  assert!((circle.area() - 12.566370614359172).abs() < f64::EPSILON);

  let square = Square { side: 3.0 };
  assert_eq!(square.describe(), "A square with side 3");
  assert!((square.area() - 9.0).abs() < f64::EPSILON);
}

/**
Traits can also have default implementations. Types can override them or use
the default.
*/

trait Greet {
  fn greet(&self) -> String {
    "Hello!".to_string()  // Default implementation
  }
}

struct Person {
  name: String,
}

impl Greet for Person {
  fn greet(&self) -> String {
    format!("Hello, I'm {}!", self.name)  // Override the default
  }
}

struct Robot;

impl Greet for Robot {
  // Uses default implementation - no override needed
}

#[test]
fn trait_defaults() {
  let person = Person { name: "Alice".to_string() };
  assert_eq!(person.greet(), "Hello, I'm Alice!");

  let robot = Robot;
  assert_eq!(robot.greet(), "Hello!");
}

/**
## Borrowing

Borrowing is a way to share data without copying it.
It's a central concept in Rust.
It's useful for performance and memory management.
The compiler will enforce that you don't have multiple mutable references to the same data.
It also means you don't have to worry about freeing memory.

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

/**
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

/**
## Closures

Closures are anonymous functions that can capture outer variables. This is useful
for example, to pass different implementations to a iterators.
*/

fn do_something_with(list: &[i32], closure: impl Fn(i32) -> i32) -> Vec<i32>{
  list.iter().map(|&x| closure(x)).collect()
}


#[test]
fn basic_closure() {
  let outer_var = 42;
  let closure = |i| outer_var + i; // This can be passed elsewhere and subsequently called

  assert_eq!(do_something_with(&[1, 2, 3], closure), &[43, 44, 45]);
}

#[test]
fn closures() {
  let mut haystack = vec![1, 2];
  haystack.push(3);
  let contains = move |needle| haystack.contains(needle);

  // haystack is now owned by the closure and cannot be used outside of it

  assert!(contains(&1));
  refute!(contains(&4));
}

/**
## Iterators

Iterators are lazy, meaning they don't do anything until you consume them.
Common methods include `map`, `filter`, `collect`, `fold`, `enumerate`, and more.
*/

#[test]
fn iterators() {
  let nums = vec![1, 2, 3, 4, 5];

  // map: transform each element
  let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
  assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

  // filter: keep only elements that match a condition
  let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();
  assert_eq!(evens, vec![2, 4]);

  // Chaining: combine multiple operations
  let result: Vec<i32> = nums.iter()
    .filter(|&&x| x > 2)
    .map(|x| x * 10)
    .collect();
  assert_eq!(result, vec![30, 40, 50]);

  // fold: reduce to a single value
  let sum: i32 = nums.iter().fold(0, |acc, x| acc + x);
  assert_eq!(sum, 15);

  // enumerate: get index with each element
  let indexed: Vec<(usize, i32)> = nums.iter().copied().enumerate().collect();
  assert_eq!(indexed, vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]);

  // for loops use iterators under the hood
  let mut total = 0;
  for n in nums.iter() {
    total += n;
  }
  assert_eq!(total, 15);
}

/**
More iterator methods: `take`, `skip`, `zip`, `any`, `all`, `find`.
*/

#[test]
fn more_iterators() {
  let nums = vec![1, 2, 3, 4, 5];

  // take: get first n elements
  let first_three: Vec<i32> = nums.iter().take(3).copied().collect();
  assert_eq!(first_three, vec![1, 2, 3]);

  // skip: skip first n elements
  let skip_two: Vec<i32> = nums.iter().skip(2).copied().collect();
  assert_eq!(skip_two, vec![3, 4, 5]);

  // zip: combine two iterators
  let letters = vec!['a', 'b', 'c'];
  let paired: Vec<(i32, char)> = nums.iter().copied().zip(letters.iter().copied()).collect();
  assert_eq!(paired, vec![(1, 'a'), (2, 'b'), (3, 'c')]);

  // any: check if any element matches
  assert!(nums.iter().any(|&x| x > 4));
  refute!(nums.iter().any(|&x| x > 10));

  // all: check if all elements match
  assert!(nums.iter().all(|&x| x > 0));
  refute!(nums.iter().all(|&x| x > 2));

  // find: get first element matching condition
  assert_eq!(nums.iter().find(|&&x| x > 3), Some(&4));
  assert_eq!(nums.iter().find(|&&x| x > 10), None);
}

## Standard Library

### Reading directories and files

The standard library provides `std::fs` for filesystem operations.
*/

use std::fs;
use std::io::Write;

#[test]
fn reading_files() {
  // Create a temporary file for testing
  let test_file = "/tmp/rust_guide_test.txt";
  fs::write(test_file, "Hello, Rust!").expect("Failed to write file");

  // Read entire file to string
  let contents = fs::read_to_string(test_file).expect("Failed to read file");
  assert_eq!(contents, "Hello, Rust!");

  // Read as bytes
  let bytes = fs::read(test_file).expect("Failed to read file");
  assert_eq!(bytes, b"Hello, Rust!");

  // Clean up
  fs::remove_file(test_file).expect("Failed to remove file");
}

#[test]
fn working_with_directories() {
  let test_dir = "/tmp/rust_guide_test_dir";

  // Create a directory
  fs::create_dir_all(test_dir).expect("Failed to create directory");

  // Create some files in the directory
  fs::write(format!("{}/file1.txt", test_dir), "Content 1").expect("Failed to write");
  fs::write(format!("{}/file2.txt", test_dir), "Content 2").expect("Failed to write");

  // Read directory entries
  let entries: Vec<String> = fs::read_dir(test_dir)
    .expect("Failed to read directory")
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.file_name().to_string_lossy().to_string())
    .collect();

  assert_eq!(entries.len(), 2);
  assert!(entries.contains(&"file1.txt".to_string()));
  assert!(entries.contains(&"file2.txt".to_string()));

  // Clean up
  fs::remove_dir_all(test_dir).expect("Failed to remove directory");
}

/**
### Path manipulation

Use `std::path::Path` and `std::path::PathBuf` for working with file paths.
*/

use std::path::Path;

#[test]
fn path_operations() {
  let path = Path::new("/tmp/example/file.txt");

  assert_eq!(path.parent(), Some(Path::new("/tmp/example")));
  assert_eq!(path.file_name(), Some(std::ffi::OsStr::new("file.txt")));
  assert_eq!(path.extension(), Some(std::ffi::OsStr::new("txt")));

  // Check if path exists (will be false for this example)
  refute!(path.exists());

  // Join paths
  let dir = Path::new("/tmp");
  let file_path = dir.join("test.txt");
  assert_eq!(file_path, Path::new("/tmp/test.txt"));
}
