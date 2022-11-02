use std::{path::PathBuf, env, fs::File};

use gl_generator::{Registry, StructGenerator};

fn main() {
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
    Registry::new(gl_generator::Api::Gles2, (2, 1), gl_generator::Profile::Core, gl_generator::Fallbacks::All, [])
    .write_bindings(StructGenerator, &mut file)
        .unwrap();
}
