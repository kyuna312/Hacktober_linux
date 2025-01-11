fn main() {
    println!("cargo:rerun-if-changed=src/boot.s");

    cc::Build::new()
        .file("src/boot.s")
        .flag("-march=armv8-a")
        .compile("boot");
}
