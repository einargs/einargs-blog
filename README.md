# Run CSR
Use `trunk serve --open --features csr`

# Build
Build this using `trunk build --features csr`

# Deploy
Deploy with `npx firebase-tools deploy`

# Note about cargo-leptos
This version is built to be a single page entirely client side web app. I want
to adapt tools already existing inside leptos to let me easily generate a static
multipage site ala various existing tools. I haven't had the time to work on
that though.

The main branch is more targeted around working on that.

It turns out there's a new static site generator called cinnog that I plan to
check out first. https://github.com/NiklasEi/cinnog
