use website::rs2md::*;

#[test]
fn comment_block_is_converted_to_markdown() {
  let input = r#"/*
# A Heading
This is a function.

Plus some other information.
*/

fn a_function() {
  println!("Hello, world!");
}

/*
## A Subheading
Some more text.
*/
"#;

let expected = r#"# A Heading
This is a function.

Plus some other information.

```rust
fn a_function() {
  println!("Hello, world!");
}
```

## A Subheading
Some more text.
"#;
  let output = from_rs(input);
  assert_eq!(expected.to_string(), output);

}

