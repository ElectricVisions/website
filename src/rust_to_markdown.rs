fn push_lines(lines: &Vec<&str>, markdown: &mut String) {
  for line in lines {
    push_line(line, markdown);
  }
}

fn push_line(line: &str, markdown: &mut String) {
  markdown.push_str(line);
  markdown.push('\n');
}

enum CodeBlock {
  Start,
  End,
  Comment,
  FirstLine,
  Code,
}

pub fn code_to_markdown(content: &str) -> String {
  let mut markdown = String::new();
  let mut action = CodeBlock::FirstLine;
  let mut buffer = vec![];

  for line in content.lines() {
    match line.trim_end() {
      "/*" if matches!(action, CodeBlock::FirstLine) => action = CodeBlock::Comment,
      "/*" if !matches!(action, CodeBlock::FirstLine) => action = CodeBlock::End,
      "*/" => action = CodeBlock::Start,
      x if matches!(action, CodeBlock::Comment) => buffer.push(x),
      x if matches!(action, CodeBlock::Code) => buffer.push(x),
      _ => (),
    }


    match action {
      CodeBlock::Start => {
        if !buffer.is_empty() {
          push_lines(&buffer, &mut markdown);
          buffer.clear();
        }
        action = CodeBlock::Code;
      },
      CodeBlock::End => {
        if !buffer.is_empty() {
          let starts_with_newline =
            if buffer.first().unwrap() == &"" { buffer.remove(0); true } else { false };
          let ends_with_newline =
            if buffer.ends_with(&[""]) { buffer.pop(); true } else { false };

          if starts_with_newline { push_line("", &mut markdown); }
          push_line("```rust", &mut markdown);
          push_lines(&buffer, &mut markdown);
          push_line("```", &mut markdown);
          if ends_with_newline { push_line("", &mut markdown); }
          buffer.clear();
        }
        action = CodeBlock::Comment;
      },
      _ => (),
    }
  }

  markdown
}

#[cfg(test)]
mod tests {
  use super::*;

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
    let output = code_to_markdown(input);
    assert_eq!(expected.to_string(), output);

  }
}
