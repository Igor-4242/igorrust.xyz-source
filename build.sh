dx build --release &&

rm -rf ./igorrust.xyz-source/* &&
cp -r ./target/dx/igorrust/release/web/public/* ../igorrust.xyz-source/ &&

cd ../igorrust.xyz-source/ &&

git add . &&
git commit -m "update" &&
git push