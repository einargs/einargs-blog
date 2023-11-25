pub mod app;
pub mod routes;
pub mod server;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      //leptos::leptos_dom::HydrationCtx::stop_hydrating();
      leptos::mount_to_body(App);
    }
}
}
