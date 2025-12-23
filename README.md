# My Blog/Website

A Rust-based static site generator that converts markdown and Rust files into HTML.

## Commands

### Development
* `cargo run` - Build the site (generates HTML in `public/` & `public/posts/`)
* `./ws` - Start web server on http://localhost:1337
* `./watch` - Watch source and tests, run tests on changes
* `./watch-posts` - Watch posts directory, rebuild site on changes

### Testing & Quality
* `cargo test` - Run all tests
* `cargo clippy` - Run linter
* `cargo fmt` - Format code

### Deployment
* `cargo run -- --deploy` - Deploy to [Cloudflare Pages](https://pages.cloudflare.com/)

## Project Structure

* `posts/` - Blog posts (`.md` or `.rs` files with embedded markdown)
* `pages/` - Static pages (`.md` files)
* `artifacts/` - Generated markdown from `.rs` files
* `public/` - Generated HTML output (served by web server)
* `templates/` - HTML templates

## How It Works

1. Generates markdown from `.rs` files in `posts/` → `artifacts/`
2. Converts markdown in `artifacts/` to HTML → `public/posts/`
3. Converts markdown in `pages/` to HTML → `public/`
4. Removes stale HTML posts
5. Post-processes HTML (inserts dates, etc.)
6. Generates index page

## References

* [MultiMarkdown](https://fletcher.github.io/MultiMarkdown-6/MMD_Users_Guide.html)
