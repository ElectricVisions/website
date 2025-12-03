use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

use website::post;

#[macro_export]
macro_rules! refute {
  ($cond:expr $(,)?) => { assert!(!$cond) };
  ($cond:expr, $($arg:tt)+) => { assert!(!$cond, $($arg)+) };
}

pub const MD_FILENAME: &str = "2020-01-01-test.md";
pub const RS_FILENAME: &str = "2020-01-02-rust.rs";

pub struct TempPathConfig {
  posts: TempDir,
  pages: TempDir,
  artifacts: TempDir,
  public: TempDir,
  public_posts: TempDir,
}

impl TempPathConfig {
  pub fn as_path_config(&self) -> post::PathConfig {
    post::PathConfig {
      posts: self.posts.path().to_path_buf(),
      pages: self.pages.path().to_path_buf(),
      artifacts: self.artifacts.path().to_path_buf(),
      public: self.public.path().to_path_buf(),
      public_posts: self.public_posts.path().to_path_buf(),
    }
  }
}

#[cfg(test)]
pub fn setup() -> TempPathConfig {
  let posts = TempDir::with_prefix("posts").unwrap();
  let pages = TempDir::with_prefix("pages").unwrap();
  let public = TempDir::with_prefix("public").unwrap();
  let public_posts = TempDir::with_prefix_in("public_posts", public.path()).unwrap();
  let artifacts = TempDir::with_prefix("artifacts").unwrap();

  TempPathConfig {
    posts,
    pages,
    artifacts,
    public,
    public_posts,
  }
}

fn create_page(path: &Path) {
  let mut file = File::create(path).unwrap();
  file.write_all(r#"mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.cs
updated: 2020-01-02
tags: game


# A Title

Some intro text
"#.as_bytes()).unwrap();
}

fn create_code(path: &Path) {
  let mut file = File::create(path).unwrap();
  file.write_all(r#"/**
mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
tags: rust

# Rust Example

Brief intro

*/
pub fn hello_world() {
  println!("Hello World");
}
"#.as_bytes()).unwrap();
}

pub fn make_post(dirs: &post::PathConfig) -> post::Metadata {
  let post = post::Metadata {
    name: "2020-01-01-test".to_string(),
    title: "A Title".to_string(),
    created: "2020-01-01".to_string(),
    updated: "2020-01-02".to_string(),
    tags: "game".to_string(),
    intro: "Some intro text\n".to_string(),
  };

  create_page(&dirs.posts.join(MD_FILENAME));

  post
}

pub fn make_code(dirs: &post::PathConfig) {
  create_code(&dirs.posts.join(RS_FILENAME));
}

pub fn make_artifact(dirs: &post::PathConfig) -> post::Metadata {
  let post = make_post(dirs);

  fs::copy(dirs.posts.join(MD_FILENAME), dirs.artifacts.join(MD_FILENAME)).unwrap();

  post
}

pub fn make_page(dirs: &post::PathConfig) -> &str {
  let filename = "about";
  create_page(&dirs.pages.join(format!("{filename}.md")));

  filename
}
