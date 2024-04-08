use std::io::Write;

fn main(){

    let bindings = bindgen::Builder::default()
        .header("./src/wintab32_headers/wrapper.h")
        .layout_tests(false) // makes the output a lot smaller. still pretty big tho
        .generate()
        .expect("Unable to generate bindings");

    let mut file = std::fs::File::create("./src/wintab32_bindings.rs").expect("Couldn't create bindings file");

    // Silence the billions of annoying warnings about variable naming
    writeln!(file, "#![allow(warnings)]").expect("Couldn't write warnings attribute");
    
    // Write the generated bindings.
    writeln!(file, "{}", bindings.to_string()).expect("Couldn't write bindings");
}






























