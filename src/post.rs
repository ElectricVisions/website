use regex::Regex;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::io;
use crate::string::format_or_empty;

#[derive(Debug, PartialEq)]
pub struct Metadata {
  pub name: String,
  pub title: String,
  pub created: String,
  pub updated: String,
  pub tags: String,
  pub intro: String,
}

pub struct PathConfig {
  pub posts: PathBuf,
  pub pages: PathBuf,
  pub artifacts: PathBuf,
  pub public: PathBuf,
  pub public_posts: PathBuf,
}

// Builds the metadata struct for all posts
pub fn build_all(paths: &PathConfig) -> Vec<Metadata> {
  io::paths_in_dir(&paths.artifacts, &["md"])
    .iter()
    .map(build)
    .collect()
}

// Reload the generated HTML posts and insert tags, created & updated dates
// And insert highlightjs.html into <head>
pub fn post_process(posts: &[Metadata], paths: &PathConfig) {
  let highlightjs = io::load_template("highlightjs");

  for p in posts {
    let created = format_or_empty("Published: ", &p.created);
    let updated = format_or_empty("Updated: ", &p.updated);
    let path = paths.public_posts.join(format!("{}.html", &p.name));
    let original_html =
      fs::read_to_string(&path)
      .unwrap_or_else(|e| panic!("Could not open: {}.\n{}", path.to_str().unwrap(), e));

    let html =
      original_html
      .replace(highlightjs.as_str(), "")
      .replace("</head>", format!("{highlightjs}</head>").as_str())
      .replace("{tags}", &p.tags)
      .replace("{created}", &created)
      .replace("{updated}", &updated);

    if html != original_html {
      let mut post = File::create(path).unwrap();
      post.write_all(html.as_bytes()).unwrap();
      println!("  Processed {}", p.name);
    }
  }
}

// Populates the metadata struct which allows the Home page to be generated
pub fn build(path: &PathBuf) -> Metadata {
  let contents = fs::read_to_string(path).unwrap();

  let mut title = String::new();
  let mut created =  String::new();
  let mut updated = String::new();
  let mut tags = String::new();
  let mut is_metadata = true;
  let mut intro = String::new();
  let heading_re = Regex::new(r"^# ").unwrap();
  let transclusion_re = Regex::new(r"\{\{.*\}\}").unwrap();

  for line in contents.split('\n') {
    let line = line.trim_end();

    if is_metadata {
      if line.is_empty() {
        is_metadata = false;
        continue;
      } else if line.starts_with(' ') { // Don't try to parse multiline metadata
        continue;
      }

      let (key, value) = line.split_once(": ").unwrap();
      match key {
        "title" => title = unescape(value),
        "created" => created = String::from(value),
        "updated" => updated = String::from(value),
        "tags" => tags = String::from(value),
        _ => (), // Ignore unrecognised metadata
      }

      continue;
    }

    if heading_re.is_match(line) {
      title = unescape(&line[2..]);
    } else if !line.is_empty() && !transclusion_re.is_match(line) {
      intro.push_str(line);
      intro.push('\n');
    } else if !intro.is_empty() {
      break;
    }
  }

  let filename = path.file_name().unwrap().to_str().unwrap();
  Metadata {
    name: String::from(&filename[0..filename.len() - 3]),
    title,
    created: if created.is_empty() { get_created(filename) } else { created },
    updated,
    tags,
    intro,
  }
}

fn unescape(s: &str) -> String {
  s.replace("\\#", "#")
}

fn get_created(filename: &str) -> String {
  if filename.starts_with("draft-") { return "draft".to_string() }
  if !filename.starts_with("20") { return filename.to_string() }

  filename[0..10].to_string()
}

