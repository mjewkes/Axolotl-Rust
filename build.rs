extern crate gcc;

fn main() {
    gcc::compile_library("libcurve25519-donna.a", &["libs/curve25519-donna-1.1/curve25519-donna.c"]);
}
