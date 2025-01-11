
fn main() {
    println!("cargo:rerun-if-changed=src/boot.s");
    println!("cargo:rerun-if-changed=linker.ld");

    cc::Build::new()
        .file("src/boot.s")
        .flag("-march=armv8-a")
        .flag("-mgeneral-regs-only")
        .compile("boot");
}
