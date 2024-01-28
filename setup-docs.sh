# This is just a temporary outline script; need to test with build
echo "make sure to run cargo build and then visit each page"
SOURCE=./target/site
rm -r docs
cp -ra $SOURCE docs
mv docs/blog/test.static.html docs/blog/test.html
mv docs/404.static.html docs/404.html
mv docs/other.static.html docs/other.html
mv docs/404.static.html docs/404.html
mv docs/blog.static.html docs/blog.html
mv docs/.static.html docs/index.html
