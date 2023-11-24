use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// TODO: I need an Island with my router functionality inside it.
/// Island router
#[island]
fn BlogRouter() -> impl IntoView {
  view! {
    <Router>
      <main>
        <Routes>
          <Route
            //mode=StaticMode::Upfront
            path="/"
            view=HomePage
            //static_params=|| Box::pin(async { StaticParamsMap::default() })
            />
          <Route
            //mode=StaticMode::Upfront
            path="/other"
            view=OtherPage
            //static_params=|| Box::pin(async { StaticParamsMap::default() })
            />
          // TODO: make this a static route with an any parameter that is
          // "404". That way we'll auto build that for the SSG.
          <Route path="/*any" view=NotFound />
        </Routes>
      </main>
    </Router>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

    // sets the document title
    <Title text="Welcome to Leptos"/>

    // content for this welcome page
    <BlogRouter/>
  }
}

/// Other page.
#[component]
fn OtherPage() -> impl IntoView {
  view! {
    <A href="/">"Back"</A>
    <h1>"Hello!"</h1>
  }
}

/// Island Counter.
#[island]
fn Counter() -> impl IntoView {
  // Creates a reactive value to update the button
  let (count, set_count) = create_signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);
  view! {
    <button on:click=on_click>"Click Me: " {count}</button>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

  view! {
      <h1>"Welcome to Leptos!"</h1>
      <A href="/other">"Link to other page"</A>
      <A href="/wrong">"Bad link"</A>
      <Counter />
  }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
  // set an HTTP status code 404
  // this is feature gated because it can only be done during
  // initial server-side rendering
  // if you navigate to the 404 page subsequently, the status
  // code will not be set because there is not a new HTTP request
  // to the server
  #[cfg(feature = "ssr")]
  {
    // this can be done inline because it's synchronous
    // if it were async, we'd use a server function
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }

  view! {
      <h1>"Not Found"</h1>
  }
}
