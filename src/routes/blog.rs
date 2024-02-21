use leptos::*;
use leptos_router::Params;
use leptos_router::*;

use post_lib::StaticPost;
use post_macros::posts_metadata;

posts_metadata!(POSTS, "./posts");

fn get_post_for(slug: String) -> Option<&'static StaticPost> {
  for post in POSTS.iter() {
    if post.link_slug == slug {
      return Some(post)
    }
  }
  return None
}

#[component]
fn Loading() -> impl IntoView {
  view! {
    <p>"Loading..."</p>
  }
}

#[component]
fn PostCard(post: &'static StaticPost) -> impl IntoView {
  let href = post.create_href();
  view! {
    <div class="flex flex-col bg-white border shadow-lg h-full rounded-xl">
    // <img class="w-full h-auto rounded-t-xl" src="https://images.unsplash.com/photo-1680868543815-b8666dba60f7?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2532&q=80" alt="Image Description">
      <a class="block p-4 md:p-5" href=href>
        <h3 class="text-lg font-bold text-black">
          {post.title}
        </h3>
        <p class="mt-1 text-black">
          {post.description}
            //"Some quick example text to build on the card title and make up the bulk of the card's content."
        </p>
        /* <a class="mt-2 py-2 px-3 inline-flex justify-center shadow-lg items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-primary text-white hover:bg-primary[1.5] disabled:opacity-50 disabled:pointer-events-none dark:focus:outline-none" href=href>
          "Post"
        </a>*/
      </a>
    </div>
  }
}

#[derive(PartialEq, Debug, Clone, Params)]
struct BlogPostParams {
  slug: String
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
  let params = use_params::<BlogPostParams>();
  // TODO add routing logic to redirect to 404 if this slug doesn't exist
  // TODO: fix all the expects in here, they're disgusting

  let post_opt = move || -> Option<&'static StaticPost> {
    let post_params = params.get().ok()?;
    let slug = post_params.slug;
    for post in POSTS.iter() {
      if post.link_slug == slug {
        return Some(post)
      }
    }
    None
  };

  let redirect = || view! {
    <Redirect path="/404" />
  };

  let show_post = |post: &'static StaticPost| {
    view! {
      <h1 class="text-2xl">{post.title}</h1>
      <div class="post-markdown" inner_html={post.content}></div>
    }
  };

  let render = move || match post_opt() {
    Some(post) => show_post(post).into_view(),
    None => redirect().into_view()
  };

  view! {
    <div>
      {render}
    </div>
  }
}

#[component]
pub fn BlogPostsPage() -> impl IntoView {
  view! {
    <div>
      <h1 class="text-2xl">"Projects"</h1>
      <div class="grow grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
        {POSTS.iter()
          .map(|post| view! { <PostCard post /> })
          .collect_view()
        }
      </div>
    </div>
  }
}
