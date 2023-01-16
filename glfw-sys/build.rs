use cmake;

fn main() {
    let dst = cmake::Config::new("glfw")
    .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=lib/glfw3");
}
