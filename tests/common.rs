use tempfile::TempDir;
use website::post::PathConfig;

pub struct TempPathConfig {
  posts: TempDir,
  artifacts: TempDir,
  public: TempDir,
  public_posts: TempDir,
}

impl TempPathConfig {
  pub fn as_path_config(&self) -> PathConfig {
    PathConfig {
      posts: self.posts.path().to_path_buf(),
      artifacts: self.artifacts.path().to_path_buf(),
      public: self.public.path().to_path_buf(),
      public_posts: self.public_posts.path().to_path_buf(),
    }
  }
}

#[cfg(test)]
pub fn setup() -> TempPathConfig {
  let posts = TempDir::with_prefix("posts").unwrap();
  let public = TempDir::with_prefix("public").unwrap();
  let public_posts = TempDir::with_prefix_in("public_posts", public.path()).unwrap();
  let artifacts = TempDir::with_prefix("artifacts").unwrap();

  TempPathConfig {
    posts,
    artifacts,
    public,
    public_posts,
  }
}

