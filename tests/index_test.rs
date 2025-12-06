use std::fs;

use crate::common::*;
use crate::refute;
use website::index;

#[test]
fn generates_index_page() {
  let dirs = setup();
  let paths = dirs.as_path_config();

  make_page(&paths);
  let posts = [make_post(&paths), make_draft(&paths)];
  index::generate(&posts, &paths);
  let html = fs::read_to_string(paths.public.join("index.html")).unwrap();

  assert!(html.contains("<nav>"));
  assert!(html.contains("<article class=\"card\">"));
  assert!(html.contains("<a href=\"/posts/2020-01-01-test.html\">"));
  assert!(html.contains("<h3 class=\"title\">A Title</h3>"));
  assert!(html.contains("<p class=\"created\">Published: 2020-01-01</p>"));
  assert!(html.contains("<p class=\"intro\">Some intro text\n</p>"));
  assert!(html.contains("<p>\n      Some stuff about me"));
  assert!(html.contains("<a href=\"/about.html\">more...</a>"));

  assert!(html.contains("<article class=\"card draft\">"));
  assert!(html.contains("<a href=\"/posts/draft-test.html\">"));
}

#[test]
fn removes_draft_articles_from_index_page() {
  let dirs = setup();
  let paths = dirs.as_path_config();

  make_index(&paths);

  index::remove_drafts(&paths);

  let html = fs::read_to_string(paths.public.join("index.html")).unwrap();
  refute!(html.contains("<article class=\"card draft\">"));
  refute!(html.contains("Lorem ipsum dolor sit amet"));
  assert!(html.contains("<article class=\"card\">"));
  assert!(html.contains("consectetur adipiscing elit sed do"));
  assert!(html.contains("Duis aute irure dolor in reprehenderit"));
}
