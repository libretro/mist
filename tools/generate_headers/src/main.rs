fn main() {
    let path = std::path::PathBuf::from(std::env::args().nth(1).expect("include path"));
    std::fs::write(path.join("mist_results.h"), mist::result::generate_header()).unwrap();
}
