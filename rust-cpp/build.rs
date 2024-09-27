fn main() {
    cxx_build::bridge("src/main.rs")
        .file("include/my_cpp.cc")
        .compile("rust-cpp-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
}
