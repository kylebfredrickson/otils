fn main() {
    cc::Build::new().file("src/ops/select.c").compile("ops");
}
