use std::fs::File;
use tempfile::TempDir;

use crate::common;
use crate::refute;
use website::io::*;

#[test]
fn load_template_reads_file_from_templates_directory() {
  let template = load_template("nav");
  assert!(template.contains("<nav>"));
}

#[test]
fn paths_in_dir_returns_paths_of_all_files_in_directory() {
  let temp_dir = TempDir::with_prefix("posts").unwrap();
  let temp_path = temp_dir.path();

  File::create(temp_path.join("non-md-file")).unwrap();
  File::create(temp_path.join("2018-04-08-language-and-framework.md")).unwrap();
  File::create(temp_path.join("2018-04-15-javascript-physics-engines.md")).unwrap();
  let mut paths = paths_in_dir(temp_path, &["md"]);
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

#[test]
fn remove_stale_html_posts_removes_html_posts_that_no_longer_exist() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();
  File::create(paths.artifacts.join("2018-04-08-language-and-framework.md")).unwrap();
  File::create(paths.public_posts.join("2018-04-08-language-and-framework.html")).unwrap();
  File::create(paths.public_posts.join("2018-04-15-javascript-physics-engines.html")).unwrap();

  remove_stale_html_posts(&dirs.as_path_config());

  assert!(paths.public_posts.join("2018-04-08-language-and-framework.html").exists());
  refute!(paths.public_posts.join("2018-04-15-javascript-physics-engines.html").exists());
}
