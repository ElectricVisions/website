use std::path::{Path, PathBuf};
use std::fs;
use crate::path;
use crate::post::PathConfig;

pub fn load_template(name: &str) -> String {
  fs::read_to_string(format!("templates/{name}.html")).unwrap()
}

pub fn paths_in_dir(path: &Path) -> Vec<PathBuf> {
  let mut paths: Vec<PathBuf> =
    fs::read_dir(path)
    .unwrap()
    .map(|entry| entry.unwrap().path())
    .collect();
  paths.sort();
  paths.reverse();
  paths
}

pub fn remove_stale_html_posts(paths: &PathConfig) {
  println!("## Remove any HTML posts that no longer exist");

  for html_file in paths_in_dir(&paths.public_posts) {
    let md_file = path::markdown(&paths.posts, html_file.file_stem().unwrap().to_str().unwrap());
    if !path::exists(&md_file) {
      println!("  Removing {}", html_file.to_str().unwrap());
      std::fs::remove_file(html_file).unwrap();
    }
  }
}
