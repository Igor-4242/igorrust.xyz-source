cargo check &&
dx build --release &&

rm -rf ../Igor-4242.github.io/* &&
cp -r ./target/dx/igorrust/release/web/public/* ../Igor-4242.github.io/ &&

cd ../Igor-4242.github.io/ &&

echo igorrust.xyz > CNAME
echo "Hosted on [igorrust.xyz](https://igorrust.xyz)" >> README.md

git add . &&
git commit -m "update" &&
git push