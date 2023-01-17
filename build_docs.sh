RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=nvtt_rs/index.html\">" > target/doc/index.html
cp -r target/doc ./docs
