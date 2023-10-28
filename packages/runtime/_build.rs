use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use javy::Runtime;


pub fn main() {
    println!("cargo:rerun-if-changed=test.js");

    eprintln!("Compiling JS");
    // Compile the JS code

    // let file_path = std::env::var("JS_PATH").expect(
    //     "JS_PATH environmental variable not set. JS_PATH should be set to a compiled JS file that will be packaged at build-time."
    // );

    let mut file = File::open("test.js")
        .expect("Failed to open the file");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read the file");

    // Compile the QuickJS
    let runtime = Runtime::default();

    let context = runtime.context();

    let compiled_js = context.compile_global("index.js", &content)
        .expect("Failed to compile index.js");


    
    let dest_path = Path::new("generated.txt");
    let mut f = File::create(&dest_path)
        .expect("Failed to open file to write");

    f.write_all(&compiled_js)
        .expect("Failed to write compiled QuickJS into a binary file");


}