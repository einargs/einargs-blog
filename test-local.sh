# This is just a temporary outline script; need to test with build
cargo leptos build
cp -ra ./target/site local-site
# mv local-site/.static.html local-site/index.static.html
miniserve local-site
