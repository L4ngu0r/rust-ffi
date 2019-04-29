extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/double.c")
        .file("src/main.c")
        .compile("libsimple.a");
}