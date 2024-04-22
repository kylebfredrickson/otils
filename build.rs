fn main() {
    cc::Build::new().file("src/ops/ops.c").compile("ops");
}
