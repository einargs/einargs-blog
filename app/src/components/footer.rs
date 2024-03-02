use leptos::*;
use leptos_router::*;

use crate::components::SocialLinks;

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <footer class="py-10">
      <div class="w-full mx-auto text-center">
      "Under construction."
      </div>
      <div class="w-full mx-auto text-center">
      "Made with ❤️ using Rust, Leptos, Cinnog, and TailwindCSS."
      </div>
      <SocialLinks />
    </footer>
  }
}
