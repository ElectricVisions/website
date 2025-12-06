use std::path::{PathBuf};

pub mod cloudflare;
pub mod index;
pub mod io;
pub mod markdown;
pub mod path;
pub mod post;
pub mod rs2md;
pub mod string;

use crate::post::PathConfig;

pub fn build() {
  let paths = setup_config();

  // posts -> artifacts
  println!("## Converting .rs files to .md");
  markdown::from_rs_or_md_to_md(&paths);

  // artifacts
  println!("## Populating post metadata");
  let posts = post::build_all(&paths);

  // artifacts -> public/posts
  println!("## Generating HTML posts");
  markdown::to_html_posts(&posts, &paths);

  // pages -> public
  println!("## Generating HTML pages (e.g. about, 404)");
  markdown::to_html_pages(vec!["about", "404"], &paths);

  // public/posts
  println!("## Post-processing posts");
  post::post_process(&posts, &paths);

  // public/posts
  println!("## Removing stale HTML posts");
  io::remove_stale_html_posts(&paths);

  // public/index.html
  println!("## Generating index.html");
  index::generate(&posts, &paths);
}

pub fn deploy() {
  let paths = setup_config();
  // remove drafts from artifacts and public/posts
  io::remove_drafts(&paths);

  // remove draft <article>s from public/index.html
  index::remove_drafts(&paths);

  // deploy to Cloudflare Pages
  cloudflare::deploy();

  // Call build to restore drafts
  build();
}

fn setup_config() -> PathConfig {
  PathConfig {
    posts: PathBuf::from("posts"),
    pages: PathBuf::from("pages"),
    artifacts: PathBuf::from("artifacts"),
    public: PathBuf::from("public"),
    public_posts: PathBuf::from("public/posts"),
  }
}

