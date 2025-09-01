rm -r ./docs
dx bundle
mv ./target/dx/number-facts/release/web/public ./docs
cp ./docs/index.html ./docs/404.html