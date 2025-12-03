use std::fs::File;

use crate::common;
use website::markdown;
use website::post;

#[test]
fn sets_up_metadata() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();
  let expected = vec![common::make_post(&paths)];

  let actual = post::build_all(&paths);

  assert_eq!(expected, actual);
}

#[test]
fn returns_correct_path() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();
  File::create(paths.artifacts.join(common::MD_FILENAME)).unwrap();

  let posts = vec![common::make_post(&paths)];

  markdown::to_html_posts(&posts, &paths);
  assert!(paths.public_posts.join("2020-01-01-test.html").exists());
}

