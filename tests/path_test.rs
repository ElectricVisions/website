use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;
use website::path::*;

#[test]
fn markdown_returns_correct_path() {
  let path = markdown(Path::new("posts"), "test");
  assert_eq!(path, "posts/test.md");
}

#[test]
fn html_returns_correct_path() {
  let path = html(Path::new("public/posts"), "test");
  assert_eq!(path, "public/posts/test.html");
}

#[test]
fn modified_returns_modified_time() {
  let tempfile = tempfile::NamedTempFile::new().unwrap();
  let time = modified(tempfile.path().to_str().unwrap());
  assert!(SystemTime::now().duration_since(time).unwrap() < Duration::from_secs(1));
}

#[test]
fn modified_returns_epoch_when_file_does_not_exist() {
  let dir = tempfile::tempdir().unwrap();
  let time = modified(dir.path().join("non-existant-file").to_str().unwrap());
  assert_eq!(time, SystemTime::UNIX_EPOCH);
}

#[test]
fn exists_returns_true_for_existing_file() {
  let tempfile = tempfile::NamedTempFile::new().unwrap();
  assert!(exists(tempfile.path().to_str().unwrap()));
}
