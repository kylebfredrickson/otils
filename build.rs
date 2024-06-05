fn main() {
    cc::Build::new()
        .file("src/ops/select.c")
        .file("src/ops/swap.c")
        // .file("src/ops/compare.c")
        .compile("ops");
}
