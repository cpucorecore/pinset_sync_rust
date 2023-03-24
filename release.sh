d=./pinset_sync

rm -rf $d
mkdir $d

cargo build --release
cp target/release/pinset_sync_rust $d

cp -r conf scripts $d
