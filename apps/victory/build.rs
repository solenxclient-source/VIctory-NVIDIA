fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rustc-link-arg-bins=/EXPORT:D3D12SDKVersion,data");
        println!("cargo:rustc-link-arg-bins=/EXPORT:D3D12SDKPath,data");
    }
}
