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
  let metadata_re = Regex::new(r": ").unwrap();
  let heading_re = Regex::new(r"^# ").unwrap();

  for line in contents.split("\n") {
    if is_metadata {
      if line.trim().is_empty() {
        is_metadata = false;
        continue;
      }

      let key_value: Vec<&str> = metadata_re.splitn(line, 2).collect();
      match key_value[0] {
        "updated" => updated = String::from(key_value[1]),
        "tags" => tags = String::from(key_value[1]),
        _ => panic!("Unrecognised metadata: {}", key_value[0])
      }
    }

    if heading_re.is_match(line) {
      let heading: Vec<&str> = heading_re.splitn(line, 2).collect();
      title = String::from(heading[1]);
    }

    if !title.is_empty() { break; }
  }

  Post {
    name: String::from(&filename[0..filename.len() - 3]),
    title,
    created: String::from(&filename[0..10]),
    updated,
    tags,
  }
}

fn main() {
  // Iterate over the posts/ folder
  let mut posts: Vec<Post> =
    fs::read_dir("posts")
      .unwrap()
      .map(|entry| build_post(entry.unwrap()))
      .collect();

  posts.sort_by_key(|p| p.created.clone());

  let mut index = File::create("output/index.md").unwrap();

  println!("Writing to output/index.md");
  for p in posts {
    write!(index, "* [{}](posts/{}.html)\n", p.title, p.name).unwrap();
  }
}

