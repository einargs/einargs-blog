use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn AboutPage() -> impl IntoView {
  view! {
    <div class="grow flex flex-row justify-center items-center">
      <div class="">
        <div class="text-center mb-5">
          <div class="before:block before:absolute before:-inset-1 \
                      before:-skew-y-3 before:bg-primary before:outline-black before:outline-2 before:drop-shadow-xl relative inline-block">
            <h1 class="relative text-white text-6xl text-center p-4">"einargs"</h1>
          </div>
        </div>
      <div class="text-2xl space-3 justify-items-center items-center flex flex-col md:flex-row">
      <span>"Fullstack Web Developer"</span>
      <span class="hidden md:inline">"|"</span>
      <span>"Functional Programmer"</span>
      <span class="hidden md:inline">"|"</span>
          <span>"Type Enthusiast"</span>
        </div>
      </div>
    </div>
  }
}
