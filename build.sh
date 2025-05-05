cargo check &&
dx build --release &&

rm -rf ../Igor-4242.github.io/* &&
cp -r ./target/dx/igorrust/release/web/public/* ../Igor-4242.github.io/ &&

cd ../Igor-4242.github.io/ &&

git add . &&
git commit -m "update" &&
git push