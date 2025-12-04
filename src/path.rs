use std::path::{Path};
use std::fs;
use std::time::SystemTime;

pub fn markdown(path: &Path, name: &str) -> String {
  let mut md_file = path.join(name);
  md_file.set_extension("md");

  md_file.to_str().unwrap().to_string()
}

pub fn html(path: &Path, name: &str) -> String {
  let mut html_file = path.join(name);
  html_file.set_extension("html");

  html_file.to_str().unwrap().to_string()
}

pub fn modified(path: &str) -> SystemTime {
  if !exists(path) { return SystemTime::UNIX_EPOCH; }

  let metadata = fs::metadata(path).unwrap();
  metadata.modified().unwrap()
}

pub fn exists(path: &str) -> bool {
  fs::metadata(path).is_ok()
}
