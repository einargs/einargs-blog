use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
  let link_class = "font-medium text-white hover:text-gray-200";

  view! {
    <header class="flex flex-wrap sm:justify-start z-40 \
                   sm:flex-nowrap z-50 w-full bg-dark-gray shadow-lg \
                   text-sm py-4 mb-4">
      <div class="xl:container">
      <nav class="px-4 sm:flex \
                  sm:items-center sm:justify-between"
          //aria_label="Global"
          >
          <A class="flex-none text-xl font-bold text-white hover:text-gray-200"
            href="/">"einargs"</A>
          <div class="flex flex-row items-center gap-5 mt-5 \
            sm:justify-end sm:mt-0 sm:ps-5">
            <A class=link_class href="/">"About"</A>
            <a class=link_class href="/resume.pdf" download>"Resume"</a>
            <A class=link_class href="/blog">"Blog"</A>
          </div>
        </nav>
      </div>
    </header>
  }
}
