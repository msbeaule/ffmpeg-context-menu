fn main() {
    // adds windows UAC / elevated permission to the install_win binary
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-arg-bin=install_win=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bin=install_win=/MANIFESTUAC:level=\'requireAdministrator\'");
    }
}
