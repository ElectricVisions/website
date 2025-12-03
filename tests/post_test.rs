use std::fs::File;
use std::io::Write;

use crate::common;
use website::markdown;
use website::post;

const MD_FILENAME: &str = "2020-01-01-test.md";

fn make_post(dirs: &post::PathConfig) -> post::Metadata {
  let post = post::Metadata {
    name: "2020-01-01-test".to_string(),
    title: "I'm back! Recap of 2020 & next steps".to_string(),
    created: "2020-01-01".to_string(),
    updated: "2020-01-02".to_string(),
    tags: "game".to_string(),
    intro: "Some intro text\n".to_string(),
  };

  let mut file = File::create(dirs.posts.join(MD_FILENAME)).unwrap();
  file.write_all(r#"mmd header: {{../templates/header.html}}
mmd footer: {{../templates/footer.html}}
css: /css/main.css
updated: 2020-01-02
tags: game


# I'm back! Recap of 2020 & next steps

Some intro text
"#.as_bytes()).unwrap();


  post
}

#[test]
fn sets_up_metadata() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();
  let expected = vec![make_post(&paths)];

  let actual = post::build_all(&paths);

  assert_eq!(expected, actual);
}

#[test]
fn returns_correct_path() {
  let dirs = common::setup();
  let paths = dirs.as_path_config();
  File::create(paths.artifacts.join(MD_FILENAME)).unwrap();

  let posts = vec![make_post(&paths)];

  markdown::to_html_posts(&posts, &paths);
  assert!(paths.public_posts.join("2020-01-01-test.html").exists());
}

