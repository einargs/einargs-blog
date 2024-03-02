# Static Site Generation
Static Site Generation isn't yet a feature of Leptos. Instead, I'm going to use
the island features, and then basically run the serverside rendering for each
route.

We'll want `leptos_router::generate_route_list_inner` to get a list of routes.
It will return a list of `leptos_router::RouteListing`. We can then directly
call a function on that to render it statically.

That function ends up eventually calling stuff
(here)[https://github.com/leptos-rs/leptos/blob/061213ca783ed352370b28dfaddd30eb67bcea97/router/src/components/static_render.rs#L169].
Specifically `ResolvedStaticPath::write`.

Okay, there's a merged PR that implemented this stuff:
https://github.com/leptos-rs/leptos/pull/1649

To investigate:
- `leptos_router::StaticMode`
- `leptos_router::StaticRoute`

This is the code writing everything to the resolved path:
https://github.com/leptos-rs/leptos/blob/061213ca783ed352370b28dfaddd30eb67bcea97/router/src/components/static_render.rs#L197

The static rendering thing is working, and I can use a static file to load stuff
too. The exact details of whatever static site server I'll be using -- probably
github -- will determine how I change the output names and such. I can keep
using the `leptos_routes` stuff, since I want a nice server side router.

Sticking this in front of `leptos_routes` does override it and serve the static
route on it's own. It just doesn't do a redirect. Islands don't seem to work
with it.
```rust
.service(Files::new("", site_root)
  .use_hidden_files()
  .index_file(".static.html")
  .path_filter(|path, _head| path.ends_with("static.html")))
)
```

## Cinnog
Cinnog does SSG with the bevy ECS as a data layer. Very cool.

Remember to use `#[island]` if I want interactivity.

# TODO
- Check out daisyui's `prose` https://daisyui.com/docs/layout-and-typography/
