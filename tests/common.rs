use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile::TempDir;

use website::post;

#[macro_export]
macro_rules! refute {
  ($cond:expr $(,)?) => { assert!(!$cond) };
  ($cond:expr, $($arg:tt)+) => { assert!(!$cond, $($arg)+) };
}

pub const MD_FILENAME: &str = "2020-01-01-test.md";
pub const HTML_FILENAME: &str = "2020-01-01-test.html";
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
  let artifacts = TempDir::with_prefix("artifacts").unwrap();
  let public = TempDir::with_prefix("public").unwrap();
  let public_posts = TempDir::with_prefix_in("public_posts", public.path()).unwrap();

  TempPathConfig {
    posts,
    pages,
    artifacts,
    public,
    public_posts,
  }
}

fn create_page(path: &Path, post: &post::Metadata) {
  let mut file = File::create(path).unwrap();
  let title = &post.title;
  let intro = &post.intro;
  let updated =
    if post.updated.is_empty() {
      "".to_string()
    } else {
      format!("updated: {}\n", post.updated)
    };

  file.write_all(format!(r#"mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.cs
{updated}tags: game

# {title}

{intro}
"#).as_bytes()).unwrap();
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

fn create_html(path: &Path) {
  let mut file = File::create(path).unwrap();
  file.write_all(r#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
<head>
	<meta charset="utf-8"/>
	<link type="text/css" rel="stylesheet" href="/css/main.css"/>
	<title>About</title>
	<meta name="created" content="2018-04-02"/>
	<meta name="updated" content="2023-10-24"/>
	<meta name="tags" content="about game"/>
</head>
<body>
  <article class="post">
    <header>
      <div class="tags">{tags}</div>
      <div class="dates">
        <span class="created">{created}</span>
        <span class="updated">{updated}</span>
      </div>
    </header>
    <h1>About</h1>
    <p>Some intro text</p>
  </article>
</body>
</html>
"#.as_bytes()).unwrap();
}

pub fn make_post(dirs: &post::PathConfig) -> post::Metadata {
  let post = post::Metadata {
    name: "2020-01-01-test".to_string(),
    title: "A Title".to_string(),
    created: "2020-01-01".to_string(),
    updated: "".to_string(),
    tags: "game".to_string(),
    intro: "Some intro text\n".to_string(),
  };

  create_page(&dirs.posts.join(MD_FILENAME), &post);

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
  let post = post::Metadata {
    name: "2020-01-01-about".to_string(),
    title: "About".to_string(),
    created: "2020-01-01".to_string(),
    updated: "2020-01-02".to_string(),
    tags: "game".to_string(),
    intro: "Some stuff about me\n".to_string(),
  };

  create_page(&dirs.pages.join(format!("{filename}.md")), &post);

  filename
}

pub fn make_html(dirs: &post::PathConfig) -> PathBuf {
  let path = dirs.public_posts.join(HTML_FILENAME);
  create_html(&path);

  path
}
