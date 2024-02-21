#![feature(try_blocks)]
#![feature(iterator_try_collect)]
extern crate proc_macro;
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};
use syn::parse::{Parse, ParseStream};
use std::path::Path;
use std::fs;

use post_lib::*;

enum PostMacroError {
  CantParseSlug(String),
  CantFindFile(String),
  CantParsePost(String),
  CantFindPostDir(String),
  CantReadPostDir
}

use PostMacroError::*;

fn path_to_string(path: &Path) -> String {
  path.to_str()
    .unwrap_or("path cannot be turned into unicode")
    .to_string()
}

type MResult<T> = Result<T, PostMacroError>;

fn parse_post_from(path: &Path) -> MResult<Post> {
  use fs::read_to_string;
  let content = read_to_string(path)
    .map_err(|_| CantFindFile(path_to_string(path)))?;
  let slug: String = path.file_stem()
    .and_then(|s| s.to_str())
    .ok_or(CantParseSlug(path_to_string(path)))?
    .to_owned();
  Post::parse(slug.clone(), &content).ok_or(CantParsePost(
    slug
  ))
}

fn parse_posts_in_dir(path: &Path) -> MResult<Vec<Post>> {
  use fs::read_dir;
  let dir_iter = read_dir(path)
    .map_err(|_| CantFindPostDir(path_to_string(&path)))?;
  dir_iter.map(|entry| -> MResult<Post> {
    let entry = entry.map_err(|_| CantReadPostDir)?;
    parse_post_from(&entry.path())
  }).collect()
}

fn to_static_post(post: Post) -> TokenStream {
  let Post {
    title, link_slug, description, content
  } = post;
  let inner_content = content.inner_html();
  quote! {
    StaticPost {
      title: #title,
      link_slug: #link_slug,
      description: #description,
      content: #inner_content
    },
  }
}

struct MacroArgs {
  posts_dir: String,
  ident: Ident,
}

impl Parse for MacroArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    use syn::{LitStr, Token};
    let ident: Ident = input.parse()?;
    let _ = input.parse::<Token![,]>()?;
    let posts_dir: LitStr = input.parse()?;
    Ok(MacroArgs {
      ident,
      posts_dir: posts_dir.value(),
    })
  }
}

fn macro_impl(args: MacroArgs) -> MResult<TokenStream> {
  let posts_dir = Path::new(&args.posts_dir);
  let posts = parse_posts_in_dir(posts_dir)?;
  let size = posts.len();
  let ident = args.ident;

  let posts_stream: TokenStream = posts.into_iter()
    .map(to_static_post)
    .collect();

  let output = quote! {
    const #ident: [StaticPost; #size] = [#posts_stream];
  };

  Ok(output.into())
}

#[proc_macro]
pub fn posts_metadata(arg_stream: pm::TokenStream) -> pm::TokenStream {
  let args = parse_macro_input!(arg_stream as MacroArgs);
  match macro_impl(args) {
    Ok(stream) => stream.into(),
    Err(err) => {
      let err_str = match err {
        CantParseSlug(slug) => format!("Can't parse slug {}", slug),
        CantFindFile(path) => format!("Can't find file {}", path),
        CantParsePost(path) => format!("Can't parse post {}", path),
        CantFindPostDir(path) => format!("Can't find post dir {}", path),
        CantReadPostDir => format!("file io error reading post directory"),
      };
      let stream = quote! {
        compile_error!(#err_str)
      };
      stream.into()
    }
  }
}
