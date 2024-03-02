use leptos::*;
use leptos_router::*;

#[component]
pub fn SocialLinks() -> impl IntoView {
  view! {
    <div class="pt-4 w-full mx-auto flex justify-center">
      <a href="https://github.com/einargs">
        <img
          class="w-10 h-10"
          src="/github-mark.svg"/>
      </a>
      // github
      // linked in
      // TODO: make sure my resume has my email as something I can filter on
      // TODO: linked in, projects
    </div>
  }
}
