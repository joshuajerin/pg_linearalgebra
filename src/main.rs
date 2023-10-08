fn main() {
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/openblas/0.3.24/lib");
    println!("cargo:rustc-link-lib=dylib=openblas");
}
