use app::data::post::{Post, PostDate, PostDescription, Draft, Slug, PostTitle};
use app::App;
use bevy_ecs::system::EntityCommands;
use cinnog::loaders::markdown::{convert_markdown_to_html, read_markdown_from_directory};
use cinnog::{default_bundle_from_path, DataLayer, Ingest};
use leptos::serde;
use regex::Regex;
use std::io;
use std::path::Path;

#[tokio::main]
async fn main() -> io::Result<()> {
  let mut data = DataLayer::new();
  //data.insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()));

  data.run(read_markdown_from_directory::<PostFrontMatter>, "posts")?;
  data.run(convert_markdown_to_html, ());

  data.build(App).await
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct PostFrontMatter {
  pub title: String,
  pub date: Option<String>,
  pub description: String,
  pub slug: String,
  pub draft: bool,
}

impl Ingest for PostFrontMatter {
  fn ingest(self, commands: &mut EntityCommands) {
    commands.insert((
      Post,
      PostTitle(self.title),
      PostDescription(self.description),
      Slug(self.slug),
    ));
    if let Some(date) = self.date {
      commands.insert(PostDate(date));
    }
    if self.draft {
      commands.insert(Draft);
    }
  }
}
