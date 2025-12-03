use std::fs::File;
use std::io::Write;

use tempfile::TempDir;

use website::markdown::*;

#[test]
fn mmd_runs() {
  let temp_dir = TempDir::with_prefix("posts").unwrap();
  let inputfile = temp_dir.path().join("test.md");
  let outputfile = temp_dir.path().join("test.html");

  let mut buffer = File::create(&inputfile).unwrap();
  buffer.write_all("# Test".as_bytes()).unwrap();

  let response = run_mmd(inputfile.to_str().unwrap(), outputfile.to_str().unwrap());
  assert!(response);
  println!("{:?}", response);
  assert!(outputfile.exists());
}

