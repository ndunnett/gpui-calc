fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15.7");
    }

    // if cfg!(target_os = "windows") {
    //     println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
    // }
}
