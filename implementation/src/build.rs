
fn main() {
    println!("cargo:rustc-link-lib=static=pqm4");
    println!("cargo:rustc-link-search=native=../pqm4");
}
