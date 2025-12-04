use std::fs;

use website::markdown::*;
use crate::common;

#[test]
fn to_html_posts_adds_html_posts_to_public_posts_directory() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();

  let posts = vec![common::make_artifact(&paths)];
  to_html_posts(&posts, &paths);

  let expected = "<h1 id=\"atitle\">A Title</h1>\n\n<p>Some intro text</p>\n";
  let actual = fs::read_to_string(paths.public_posts.join("2020-01-01-test.html")).unwrap();
  assert!(actual.starts_with("<!DOCTYPE html>"));
  assert!(actual.contains(expected));
}

#[test]
fn to_html_pages_adds_html_pages_to_public_directory() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();

  let pages = vec![common::make_page(&paths)];

  to_html_pages(pages, &paths);

  let actual = fs::read_to_string(paths.public.join("about.html")).unwrap();
  assert!(actual.starts_with("<!DOCTYPE html>"));
  assert!(actual.contains("<h1 id=\"about\">About</h1>\n\n<p>Some stuff about me</p>"));
}

#[test]
fn from_rs_or_md_to_md_converts_any_rs_files_and_copies_over_any_md_files() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();

  common::make_post(&paths);
  common::make_code(&paths);

  from_rs_or_md_to_md(&paths);

  assert!(paths.artifacts.join("2020-01-01-test.md").exists());
  assert!(paths.artifacts.join("2020-01-02-rust.md").exists());
}
