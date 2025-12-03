use std::path::Path;

use website::path::*;

#[test]
fn markdown_returns_correct_path() {
  let path = markdown(Path::new("posts"), "test");
  assert_eq!(path, "posts/test.md");
}

#[test]
fn html_returns_correct_path() {
  let path = html(Path::new("public/posts"), "test");
  assert_eq!(path, "public/posts/test.html");
}

