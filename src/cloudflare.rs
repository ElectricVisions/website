use std::process::Command;

pub fn deploy() -> bool{
  let status = Command::new("wrangler")
    .args(["pages", "deploy", "public", "--project-name=electricvisions"])
    .status()
    .expect("Failed to execute wrangler");

  if !status.success() {
    eprintln!("Wrangler deploy failed with status: {status}");
  }

  status.success()
}
