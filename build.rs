fn main() {
    cc::Build::new()
        .file("src/ops/select.c")
        // .file("src/ops/swap.c")
        .compile("ops");
}
