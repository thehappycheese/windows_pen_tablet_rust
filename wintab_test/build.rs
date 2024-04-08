use std::io::Write;

fn main(){
    // println!("cargo:rustc-link-search=native=C:/Windows/System32/wintab32");
    // println!("cargo:rustc-link-lib=dylib=wintab32");

    return;

    let bindings = bindgen::Builder::default()
        .header("./src/wintab32_headers/wrapper.h")
        .layout_tests(false)
        // .blocklist_file(r"^(.*Windows\.h)$")
        // .blocklist_file(r"^(.*winapifamily\.h)$")
        // .blocklist_file(r"^(.*sdkddkver\.h)$")
        // .blocklist_file(r"^(.*excpt\.h)$")
        // .blocklist_file(r"^(.*stdarg\.h)$")
        // .blocklist_file(r"^(.*windef\.h)$")
        // .blocklist_file(r"^(.*winbase\.h)$")
        // .blocklist_file(r"^(.*wingdi\.h)$")
        // .blocklist_file(r"^(.*winuser\.h)$")
        // .blocklist_file(r"^(.*winnls\.h)$")
        // .blocklist_file(r"^(.*wincon\.h)$")
        // .blocklist_file(r"^(.*winver\.h)$")
        // .blocklist_file(r"^(.*winreg\.h)$")
        // .blocklist_file(r"^(.*winnetwk\.h)$")
        // .blocklist_file(r"^(.*cderr\.h)$")
        // .blocklist_file(r"^(.*dde\.h)$")
        // .blocklist_file(r"^(.*ddeml\.h)$")
        // .blocklist_file(r"^(.*dlgs\.h)$")
        // .blocklist_file(r"^(.*lzexpand\.h)$")
        // .blocklist_file(r"^(.*mmsystem\.h)$")
        // .blocklist_file(r"^(.*nb30\.h)$")
        // .blocklist_file(r"^(.*rpc\.h)$")
        // .blocklist_file(r"^(.*shellapi\.h)$")
        // .blocklist_file(r"^(.*winperf\.h)$")
        // .blocklist_file(r"^(.*winsock\.h)$")
        // .blocklist_file(r"^(.*wincrypt\.h)$")
        // .blocklist_file(r"^(.*winefs\.h)$")
        // .blocklist_file(r"^(.*winscard\.h)$")
        // .blocklist_file(r"^(.*winspool\.h)$")
        // .blocklist_file(r"^(.*ole\.h)$")
        // .blocklist_file(r"^(.*ole2\.h)$")
        // .blocklist_file(r"^(.*commdlg\.h)$")
        .generate()
        .expect("Unable to generate bindings");

    // bindings
    //     .write_to_file("./src/wintab32_bindings.rs")
    //     .expect("Couldn't write bindings!");

    let mut file = std::fs::File::create("./src/wintab32_bindings.rs").expect("Couldn't create bindings file");

    // Write the `allow` attribute to the file first.
    writeln!(file, "#![allow(warnings)]").expect("Couldn't write warnings attribute");
    
    // Write the generated bindings.
    writeln!(file, "{}", bindings.to_string()).expect("Couldn't write bindings");
}






























