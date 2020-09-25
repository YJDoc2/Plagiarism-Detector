fn main() {
    let mut dir = std::env::current_dir().unwrap();
    dir.push("lexer");
    println!("cargo:rustc-link-search={}", dir.to_str().unwrap()); // the "-L" flag
    println!("cargo:rustc-link-lib=lexer"); // the "-l" flag
}
