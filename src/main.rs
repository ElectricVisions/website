// metadata in md files overrides created date in filename
// First H1 becomes title or metadata title if no H1

use std::fs;
use std::io::Write;
use std::fs::File;
use regex::Regex;

struct Post {
  name: String,
  title: String,
  created: String,
  updated: String,
  tags: String,
  intro: String,
}

fn build_post(entry: fs::DirEntry) -> Post {
  let filename = entry.file_name().into_string().unwrap();
  let path = entry.path();
  let contents = fs::read_to_string(path).unwrap();
  println!("Reading from {}...", filename);

  let mut title = String::from("");
  let mut updated = String::from("");
  let mut tags = String::from("");
  let mut is_metadata = true;
  let mut intro = String::from("");
  let heading_re = Regex::new(r"^# ").unwrap();

  for line in contents.split("\n") {
    let line = line.trim();

    if is_metadata {
      if line.is_empty() {
        is_metadata = false;
        continue;
      }

      let (key, value) = line.split_once(": ").unwrap();
      match key {
        "title" => title = String::from(value),
        "created" => updated = String::from(value),
        "updated" => updated = String::from(value),
        "tags" => tags = String::from(value),
        _ => panic!("Unrecognised metadata: {}", value)
      }

      continue;
    }

    if heading_re.is_match(line) {
      title = line[2..].to_string();
    } else if !line.is_empty() {
      intro.push_str(line);
      intro.push('\n');
    } else if !intro.is_empty() {
      break;
    }
  }

  Post {
    name: String::from(&filename[0..filename.len() - 3]),
    title,
    created: String::from(&filename[0..10]),
    updated,
    tags,
    intro,
  }
}

fn main() {
  let mut posts: Vec<Post> =
    fs::read_dir("posts")
      .unwrap()
      .map(|entry| build_post(entry.unwrap()))
      .collect();

  posts.sort_by_key(|p| p.created.clone());

  let mut index = File::create("output/www/index.html").unwrap();

  println!("Writing to output/www/index.html");

  let posts = posts.iter().map(|p| {
    format!(
      r#"     <li><a href="posts/{}.html">{}</a></li>"#,
      p.name, p.title
    )
  }).collect::<Vec<String>>().join("\n");

  let html = format!(r#"<!DOCTYPE html>
<html lang="en">
  <ul>
{posts}
  </ul>
</html>
"#);

  index.write_all(html.as_bytes()).unwrap();
}

