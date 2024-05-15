fn main() {
    cc::Build::new()
        .file("src/ops/compare.c")
        .file("src/ops/equal.c")
        .file("src/ops/select.c")
        .compile("ops");
}
