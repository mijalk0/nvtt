fn main() {
    std::env::set_var("RUSTDOCFLAGS", "\"--cfg docsrs\" cargo +nightly doc --all-features");
}
