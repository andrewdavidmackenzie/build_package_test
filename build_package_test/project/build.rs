use std::{env, io};
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use glob::glob;

fn main() -> io::Result<()> {
    let lib_root_dir = env!("CARGO_MANIFEST_DIR");
    let out_filename = PathBuf::from(&env::var("OUT_DIR").unwrap()).join("generated.rs");

    println!("cargo:warning=Source dir is '{}'", lib_root_dir);
    println!("cargo:warning=out_file is '{}'", out_filename.display());

    // Tell Cargo that if any file changes it should rerun this build script
    println!("cargo:rerun-if-changed={}", lib_root_dir);

    let search_pattern = format!("{}/**/*_module.rs", lib_root_dir);

    let mut manifest = vec!();

    let mut out_file = File::create(out_filename).unwrap();

    for mut file in (glob(&search_pattern).unwrap()).flatten() {
        let _ = file.pop();
        let _ = file.pop();
        let module_name = file.file_name().unwrap().to_string_lossy().to_string();
        if !manifest.contains(&module_name) {
            println!("cargo:warning=module found called '{}'", module_name);

            // Add a reference to the module so that it is included in the library when built
            let mod_reference = format!("#[path=\"{}/{}/mod.rs\"]\npub mod {};\n",
                                        lib_root_dir, module_name, module_name);
            out_file.write_all(mod_reference.as_bytes()).unwrap();

            // Add the module name to the list of modules we will include
            manifest.push(module_name);
        }
    }

    // Write out the header of the manifest list function
    out_file.write_all(format!("\npub fn list_modules() {{\n").as_bytes()).unwrap();

    for module in manifest {
        // print out the module in the list by calling it's run function
        out_file.write_all(format!("    {}::optional::Optional::run();\n", module).as_bytes()).unwrap();
    }

    // close the manifest list function
    out_file.write_all("}".as_bytes()).unwrap();

    Ok(())
}