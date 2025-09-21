fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rustc-env=TS_RS_EXPORT_DIR=../web/types/server");
}
