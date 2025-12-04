use std::path::PathBuf;

pub mod rs2md;
pub mod io;
pub mod post;
pub mod markdown;
pub mod path;

use crate::post::PathConfig;

pub fn run() {
  let paths = PathConfig {
    posts: PathBuf::from("posts"),
    pages: PathBuf::from("pages"),
    artifacts: PathBuf::from("artifacts"),
    public: PathBuf::from("public"),
    public_posts: PathBuf::from("public/posts"),
  };

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
  post::generate_index(posts, &paths);
}
