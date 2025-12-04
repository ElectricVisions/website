use std::fs::File;
use std::fs;

use crate::common::*;
use website::post;

#[test]
fn build_all_posts_metadata() {
  let dirs = setup();
  let paths = dirs.as_path_config();
  File::create(paths.artifacts.join("non-md-file")).unwrap();
  let expected = vec![make_artifact(&paths)];

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

  let highlightjs = fs::read_to_string("templates/highlightjs.html").unwrap();
  let html = fs::read_to_string(html_path).unwrap();
  assert!(html.contains(&highlightjs));
  assert!(html.contains("<div class=\"tags\">game</div>"));
  assert!(html.contains("<span class=\"created\">Published: 2020-01-01</span>"));
}

#[test]
fn post_process_again_does_not_add_highlightjs_tags() {
  let dirs = setup();
  let paths = dirs.as_path_config();
  let posts = vec![make_post(&paths)];
  let html_path = make_html(&paths);

  post::post_process(&posts, &paths);
  post::post_process(&posts, &paths);

  let highlightjs = fs::read_to_string("templates/highlightjs.html").unwrap();
  let html = fs::read_to_string(html_path).unwrap();

  assert_eq!(html.match_indices(&highlightjs).count(), 1);
}

#[test]
fn generate_index() {
  let dirs = setup();
  let paths = dirs.as_path_config();

  make_page(&paths);
  let posts = vec![make_post(&paths)];
  post::generate_index(posts, &paths);
  let html = fs::read_to_string(paths.public.join("index.html")).unwrap();

  assert!(html.contains("<nav>"));
  assert!(html.contains("<article class=\"card\">"));
  assert!(html.contains("<a href=\"/posts/2020-01-01-test.html\">"));
  assert!(html.contains("<h3 class=\"title\">A Title</h3>"));
  assert!(html.contains("<p class=\"created\">Published: 2020-01-01</p>"));
  assert!(html.contains("<p class=\"intro\">Some intro text\n</p>"));
  assert!(html.contains("<p>\n      Some stuff about me"));
  assert!(html.contains("<a href=\"/about.html\">more...</a>"));
}
