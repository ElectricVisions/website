use std::fs::File;
use std::fs;
use std::io::Write;

use crate::io;
use crate::post::{Metadata, PathConfig};
use crate::post;
use crate::string::format_or_empty;

// Generates the index.html page from metadata
pub fn generate(posts: &[Metadata], paths: &PathConfig) {
  let about = post::build(&paths.pages.join("about.md"));
  let mut index = File::create(paths.public.join("index.html")).unwrap();

  let nav = io::load_template("nav");
  let card = io::load_template("card");
  let posts_html = posts.iter().map(|p| {
    let created = format_or_empty("Published: ", &p.created);
    let updated = format_or_empty("Updated: ", &p.updated);
    let draft = if p.name.starts_with("draft-") { " draft" } else { "" };

    card
      .replace("{additional_classes}", draft)
      .replace("{name}", &p.name)
      .replace("{tags}", &p.tags)
      .replace("{created}", &created)
      .replace("{updated}", &updated)
      .replace("{title}", &p.title)
      .replace("{intro}", &p.intro)
  }).collect::<Vec<String>>().join("\n");

  let more_html =
    format!("{} <div><a href=\"/about.html\">more...</a></div>", &about.intro);
  let home =
    fs::read_to_string("templates/home.html")
    .unwrap()
    .replace("{nav}", &nav)
    .replace("{intro}", &more_html)
    .replace("{posts}", &posts_html);

  index.write_all(home.as_bytes()).unwrap();
}

pub fn remove_drafts(paths: &PathConfig) {
  let html = fs::read_to_string(paths.public.join("index.html")).unwrap();
  let mut new_html = String::new();
  let mut in_draft = false;

  for line in html.lines() {
    if line.contains("<article class=\"card draft\">") {
      in_draft = true;
    }

    if !in_draft {
      new_html.push_str(line);
      new_html.push('\n');
    }

    if line.contains("</article>") {
      in_draft = false;
    }
  }

  let mut index = File::create(paths.public.join("index.html")).unwrap();
  index.write_all(new_html.as_bytes()).unwrap();
}
