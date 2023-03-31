d=./pinset_sync

rm -rf $d
mkdir $d

cargo build --release
cp target/release/pinset_sync_rust $d
cp start.sh stop.sh restart.sh $d

cp -r conf scripts $d
