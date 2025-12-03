use std::fs;
use std::fs::File;
use std::io::Write;

use crate::{ post::{Metadata, PathConfig}, path, io, rs2md };

// artifacts/*.md -> public/posts/*.html
pub fn to_html_posts(posts: &Vec<Metadata>, paths: &PathConfig) {
  for p in posts {
    let md_file = path::markdown(&paths.artifacts, &p.name);
    let html_file = path::html(&paths.public_posts, &p.name);

    to_html_page(&md_file, &html_file);
  }
}

pub fn to_html_pages(pages: Vec<&str>, paths: &PathConfig) {
  for p in pages {
    to_html_page(
      paths.pages.join(format!("{p}.md")).to_str().unwrap(),
      paths.public.join(format!("{p}.html")).to_str().unwrap(),
    );
  }
}

pub fn to_html_page(input: &str, output: &str) {
  if path::modified(input) > path::modified(output) {
    run_mmd(input, output);
  }
}

pub fn from_rs_or_md_to_md(config: &PathConfig) {
  let paths = io::paths_in_dir(&config.posts);
  for p in paths {
    let contents = fs::read_to_string(&p).unwrap();

    let ext = p.extension().unwrap().to_str().unwrap();
    let basename = p.file_stem().unwrap().to_str().unwrap();
    let contents =
      if ext == "rs" {
        rs2md::from_rs(&contents)
      } else {
        contents
      };

    fs::write(path::markdown(&config.artifacts, basename), &contents).unwrap();
  }
}

fn run_mmd(input: &str, output: &str) -> bool {
  let response = std::process::Command::new("multimarkdown")
    .arg("parse")
    .arg("-r")
    .arg(input)
    .output()
    .unwrap();

  if response.status.success() {
    let mut buffer = File::create(output).unwrap();
    buffer.write_all(response.stdout.as_slice()).unwrap();
  }

  response.status.success()
}
