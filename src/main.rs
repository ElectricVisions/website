// This program:
// 1. Generates markdown from any .rs files in posts/ and writes to artifacts/
// 2. Generates HTML from markdown in artifacts/ and writes to public/posts/
// 3. Generates HTML from markdown in pages/ and writes to public/
// 4. Removes stale HTML posts from public/posts/
// 5. Post-processes HTML posts by inserting created & updated dates
// 6. Generates HTML index from posts
// metadata in md files overrides created date in filename
// First H1 becomes title or metadata title if no H1

use website::run;

fn main() {
  run();
}
