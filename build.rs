use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=VST3_C_API_PATH");
    let path =
        std::env::var("VST3_C_API_PATH").expect("please provide a value for VST3_C_API_PATH");

    //print!("{}\n", &path);

    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <vst3_c_api.h>\n")
        .clang_args(["-I", &path])
        .blocklist_item("*_iid") // ignore the static const iid's because bindgen can't reason about them
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    //print!("{}", &out_path.display());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings");

    //----------------------------------------------------------
    // grab extra static const's that bindgen can't reason about
    //----------------------------------------------------------
    
    // todo: go over header again, match all static const's with IId's, output to another rust file, or a data file. 

}
