// Based on tutorials and documentation on https://cxx.rs/index.html
use ffi::new_hello_sayer;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust-cpp/include/my_cpp.h");

        type HelloSayer;

        fn new_hello_sayer() -> UniquePtr<HelloSayer>;
        fn sayHello(&self);
    }
}

fn main() {
    let hello = new_hello_sayer();
    hello.sayHello();
}
