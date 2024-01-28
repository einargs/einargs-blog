use leptos::*;
use leptos_router::Params;
use leptos_router::*;

use crate::server::posts::*;

#[component]
fn Loading() -> impl IntoView {
  view! {
    <p>"Loading..."</p>
  }
}

#[component]
fn PostCard(post: Post) -> impl IntoView {
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
  let post = create_resource(
    move || params.get()
              .map(move |params| params.slug)
              .ok_or(ServerFnError::ServerError("could not parse params".to_owned())),
    |slug_opt| async move {
      slug_opt.map(|slug| get_post(slug))?.await.expect()
    }
  );

  let post_view = |result| match result {
    Err
    Ok(Post{title, content, ..}) => view! {
      <h1 class="text-2xl">{title}</h1>
      <div class="post-markdown" inner_html=content.inner_html()>

      </div>
    }
  };

  view! {
    <div>
      <Suspense
      fallback=Loading
      >
    {post.get().map(|post_result| post_view(post_result.expect("error loading post on server")))}
      </Suspense>
    </div>
  }
}

#[component]
pub fn BlogPostsPage() -> impl IntoView {
  let posts = create_resource(
    || (),
    |_| async move {
      get_posts()
        .await
        .expect("error getting posts while generating")
    },
  );

  view! {
    <div>
      <h1 class="text-2xl">"Projects"</h1>
      <div class="grow grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
      <Suspense
      fallback=Loading
      >
    {posts.get().map(|posts| posts.into_iter()
                     .map(|post| view! { <PostCard post /> })
                     .collect_view()
    )}
    </Suspense>
      </div>
    </div>
  }
}
