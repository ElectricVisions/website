use std::fs::File;
use tempfile::TempDir;

use website::io::*;

#[test]
fn test_load_template() {
  let template = load_template("nav");
  assert!(template.contains("<nav>"));
}

#[test]
fn test_paths_in_dir() {
  let temp_dir = TempDir::with_prefix("posts").unwrap();
  let temp_path = temp_dir.path();
  File::create(temp_path.join("2018-04-08-language-and-framework.md")).unwrap();
  File::create(temp_path.join("2018-04-15-javascript-physics-engines.md")).unwrap();
  let mut paths = paths_in_dir(temp_path);
  paths.reverse();


  assert_eq!(
    paths[0],
    temp_path.join("2018-04-08-language-and-framework.md"),
  );
  assert_eq!(
    paths[1],
    temp_path.join("2018-04-15-javascript-physics-engines.md"),
  );
}

