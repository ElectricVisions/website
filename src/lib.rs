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
    artifacts: PathBuf::from("artifacts"),
    public: PathBuf::from("public"),
    public_posts: PathBuf::from("public/posts"),
  };

  println!("## Populating post metadata");
  let mut posts = post::build_all(&paths);

  println!("## Converting .rs files to .md");
  markdown::from_rs_or_md_to_md(&paths);

  println!("## Generating HTML posts");
  markdown::to_html_posts(&posts, &paths);

  println!("## Generating HTML pages (e.g. about, 404)");
  markdown::to_html_pages(vec!["about", "404"]);

  println!("## Removing stale HTML posts");
  io::remove_stale_html_posts(&paths);

  println!("## Post-processing posts");
  post::post_process(&mut posts);

  println!("## Generating index.html");
  post::generate_index(posts);
}
