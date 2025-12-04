use std::path::{Path, PathBuf};
use std::fs;
use crate::path;
use crate::post::PathConfig;

pub fn load_template(name: &str) -> String {
  fs::read_to_string(format!("templates/{name}.html")).unwrap()
}

pub fn paths_in_dir(path: &Path, extensions: &[&str]) -> Vec<PathBuf> {
  let mut paths: Vec<PathBuf> =
    fs::read_dir(path)
    .unwrap()
    .filter_map(|entry| {
      let path = entry.unwrap().path();

      let ext = path.extension().unwrap_or_default().to_str().unwrap();
      if extensions.contains(&ext) { Some(path) } else { None }
    }).collect();
  paths.sort();
  paths.reverse();
  paths
}

pub fn remove_stale_html_posts(paths: &PathConfig) {
  for html_file in paths_in_dir(&paths.public_posts, &["html"]) {
    let md_file = path::markdown(&paths.artifacts, html_file.file_stem().unwrap().to_str().unwrap());
    if !path::exists(&md_file) {
      println!("  Removing {}", html_file.to_str().unwrap());
      std::fs::remove_file(html_file).unwrap();
    }
  }
}
