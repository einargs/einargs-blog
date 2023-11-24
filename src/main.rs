#[cfg(any(feature = "ssr", feature = "rust-analyzer"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use actix_files::Files;
  use actix_web::*;
  use einargs_blog::app::*;
  use futures::future::try_join_all;
  use leptos::*;
  use leptos_actix::{generate_route_list_with_ssg, LeptosRoutes};
  use leptos_router::*;

  let conf = get_configuration(None).await.unwrap();
  let addr = conf.leptos_options.site_addr;
  // Generate the list of routes in your Leptos App
  let (routes, _static_data_map) = generate_route_list_with_ssg(App);
  let params = StaticParamsMap::default();
  try_join_all(
    routes
      .iter()
      .map(|route| route.build_static(&conf.leptos_options, App, || {}, &params)),
  )
  .await?;
  println!("listening on http://{}", &addr);

  HttpServer::new(move || {
    let leptos_options = &conf.leptos_options;
    let site_root = &leptos_options.site_root;

    App::new()
      .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
      // serve JS/WASM/CSS from `pkg`
      .service(Files::new("/pkg", format!("{site_root}/pkg")))
      // serve other assets from the `assets` directory
      .service(Files::new("/assets", site_root))
      // serve the favicon from /favicon.ico
      .service(favicon)
      // Sticking this 
      /*.service(Files::new("", site_root)
        .use_hidden_files()
        .show_files_listing()
        .index_file(".static.html")
        //.path_filter(|path, _head| path.ends_with("static.html")))
      )*/
      .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
      .app_data(web::Data::new(leptos_options.to_owned()))
    //.wrap(middleware::Compress::default())
  })
  .bind(&addr)?
  .run()
  .await
}

#[cfg(any(feature = "ssr", feature = "rust-analyzer"))]
#[actix_web::get("favicon.ico")]
async fn favicon(
  leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
  let leptos_options = leptos_options.into_inner();
  let site_root = &leptos_options.site_root;
  Ok(actix_files::NamedFile::open(format!(
    "{site_root}/favicon.ico"
  ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr", feature = "rust-analyzer")))]
pub fn main() {
  // no client-side main function
  // unless we want this to work with e.g., Trunk for pure client-side testing
  // see lib.rs for hydration function instead
  // see optional feature `csr` instead
}

#[cfg(any(all(not(feature = "ssr"), feature = "csr"), feature = "rust-analyzer"))]
pub fn main() {
  // a client-side main function is required for using `trunk serve`
  // prefer using `cargo leptos serve` instead
  // to run: `trunk serve --open --features csr`
  use einargs_blog::app::*;
  use leptos::*;
  use wasm_bindgen::prelude::wasm_bindgen;

  console_error_panic_hook::set_once();

  leptos::mount_to_body(App);
}
