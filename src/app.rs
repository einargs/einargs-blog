use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn Header() -> impl IntoView {
  let link_class = "font-medium text-gray-900 hover:text-gray-600";

  view! {
    <header class="flex flex-wrap sm:justify-start z-40 \
      sm:flex-nowrap z-50 w-full bg-white shadow-lg \
      text-sm py-4 mb-4">
      <div class="xl:container">
        <nav class="px-4 sm:flex \
            sm:items-center sm:justify-between"
          //aria_label="Global"
          >
          <A class="flex-none text-xl font-semibold"
            href="/">"einargs"</A>
          <div class="flex flex-row items-center gap-5 mt-5 \
            sm:justify-end sm:mt-0 sm:ps-5">
            <A class=link_class href="/">"About"</A>
            <A class=link_class href="#">"Resume"</A>
            <A class=link_class href="/projects">"Projects"</A>
            <A class=link_class href="#">"Blog"</A>
          </div>
        </nav>
      </div>
    </header>
  }
}

#[component]
fn Footer() -> impl IntoView {
  view! {
    <footer class="py-10">
      <div class="w-full mx-auto text-center">
        "Made with ❤️ using Rust, Leptos, and TailwindCSS."
      </div>
    </footer>
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
    <Router>
      <div class="min-h-screen flex flex-col">
        <Header />
        <div class="grow flex flex-col bg-paper xl:container">
          <main class="p-5 grow flex flex-col">
            <BlogRouter />
          </main>
          <Footer />
        </div>
      </div>
    </Router>
  }
}

#[component(transparent)]
fn BlogRouter() -> impl IntoView {
  use crate::routes::projects::ProjectsPage;

  view! {
    <Routes>
      <StaticRoute
        mode=StaticMode::Upfront
        path="/"
        view=AboutPage
        static_params=|| Box::pin(async { StaticParamsMap::default() })
      />
      <StaticRoute
        mode=StaticMode::Upfront
        path="/projects"
        view=ProjectsPage
        static_params=|| Box::pin(async { StaticParamsMap::default() })
        />
      <StaticRoute
        mode=StaticMode::Upfront
        path="/other"
        view=OtherPage
        static_params=|| Box::pin(async { StaticParamsMap::default() })
        />
      <StaticRoute
        mode=StaticMode::Upfront
        path="/404"
        view=NotFoundPage
        static_params=|| Box::pin(async { StaticParamsMap::default() })
        />
    // TODO: make this a static route with an any parameter that is
    // "404". That way we'll auto build that for the SSG.
      <Route path="/*any" view=NotFoundRes />
    </Routes>
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

/// Counter.
#[component]
fn Counter() -> impl IntoView {
  // Creates a reactive value to update the button
  let (count, set_count) = create_signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);
  view! {
    <button on:click=on_click>"Click Me: " {count}</button>
  }
}

#[component]
fn SocialLinks() -> impl IntoView {
  view! {
    <div>
      // github
      // linked in
      // TODO: make sure my resume has my email as something I can filter on
      // TODO: linked in, projects
    </div>
  }
}

/// Renders the home page of your application.
#[component]
fn AboutPage() -> impl IntoView {
  view! {
    <div class="grow flex flex-row justify-center items-center">
      <div class="">
        <h1 class="text-6xl text-center">"einargs"</h1>
        <div class="text-2xl">"Fullstack Web Developer | Functional Programmer | Type Enthusiast"</div>
        <A href="/other">"Link to other page"</A>
        <A href="/wrong">"Bad link"</A>
        <Counter />
      </div>
    </div>
  }
}

#[component]
fn NotFoundPage() -> impl IntoView {
  view! {
      <h1>"Custom Not Found"</h1>
  }
}

/// 404 - Not Found
#[component]
fn NotFoundRes() -> impl IntoView {
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

  NotFoundPage()
}
