use std::fs;

use crate::common::*;
use website::post;

#[test]
fn build_all_posts_metadata() {
  let dirs = setup();
  let paths = dirs.as_path_config();
  let expected = vec![make_post(&paths)];

  let actual = post::build_all(&paths);

  assert_eq!(expected, actual);
}

#[test]
fn post_process_adds_highlightjs_tags_created_and_updated_dates() {
  let dirs = setup();
  let paths = dirs.as_path_config();
  let posts = vec![make_post(&paths)];
  let html_path = make_html(&paths);

  post::post_process(&posts, &paths);

  let expected = r#"<link rel="stylesheet" href="/highlightjs/styles/onedark.min.css">
  <script src="/highlightjs/highlight.min.js"></script>
  <script>hljs.highlightAll();</script>
</head>
"#;

  let actual = fs::read_to_string(html_path).unwrap();
  assert!(actual.contains(expected));
  assert!(actual.contains("<div class=\"tags\">game</div>"));
  assert!(actual.contains("<span class=\"created\">Published: 2020-01-01</span>"));
  assert!(actual.contains("<span class=\"updated\">Updated: 2020-01-02</span>"));
}

#[test]
fn generate_index() {

}
