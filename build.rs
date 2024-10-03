#[path = "src/utils.rs"]
mod utils;
use utils::list_mods;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn generate_bin() -> io::Result<()> {
    let bin_dir = utils::get_project_root().unwrap().join("src/bin");
    let src_dir = utils::get_project_root().unwrap().join("src");

    let list_of_mods = list_mods(src_dir.as_path(), None,None, vec!["utils"]).unwrap();

    // collect module according to ancestors (joined by |)
    let mut module_map = HashMap::new();

    for module in list_of_mods.iter() {
        let key = module.ancestors.join("|");

        if module_map.contains_key(&key) {
            let value: &mut Vec<_> = module_map.get_mut(&key).unwrap();
            value.push(module);
        } else {
            module_map.insert(key, vec![module]);
        }
    }

    for (key, module) in module_map.iter() {
        let splits = key.split("|").collect::<Vec<_>>();

        let root_package = *splits.first().unwrap();
        let mod_name = splits.last().unwrap().split('_').next().unwrap();

        // split root_package name with _ and get the first letter of each word
        // use mod_name as the module name
        // create a file with the module name
        let fname = root_package
            .split('_')
            .map(|s| s.chars().next().unwrap())
            .collect::<String>()
            .to_lowercase() + "_" + mod_name + ".rs";

        let mut generated_code = String::new();

        // push line that warns the file is auto-generated
        generated_code.push_str("// This file is auto-generated. Do not edit!\n");

        generated_code.push_str("#[path = \"../rust_with_examples_activities/mod.rs\"]\nmod rust_with_examples_activities;\n");
        generated_code.push_str("#[path = \"../utils.rs\"]\nmod utils;\n");

        generated_code.push_str("fn main() {\n");

        for m in module.iter() {
            let run_fn_str = format!("    {}::run();\n", m.to_string());
            generated_code.push_str(&run_fn_str);
        }

        generated_code.push_str("}");

        let dest_path = Path::new(&bin_dir).join(fname);

        match File::create(&dest_path) {
            Ok(mut f) => {
                match f.write_all(generated_code.as_bytes()) {
                    Ok(_) => {
                        println!("File created: {:?}", dest_path);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

fn generate_run_all_rs() -> io::Result<()> {
    // Get the directory path where the generated code will be placed
    let out_dir = env::var("OUT_DIR").unwrap();
    // Create a string to hold the generated code
    let mut generated_code = String::new();

    generated_code.push_str("pub fn run_all() {\n");

    // list directoreis in src folder
    let src_dir = utils::get_project_root().unwrap().join("src");

    let list_of_mods = list_mods(src_dir.as_path(), None, None, vec!["utils"]).unwrap();

    for module in list_of_mods.iter() {
        generated_code.push_str(&format!("    println!(\"{:-^1$}\");\n", "", 80));
        generated_code.push_str(&format!("    println!(\"Running: {}\n\");\n", module));
        generated_code.push_str(&format!("    {}::run();\n", module));
    }

    generated_code.push_str("}\n");

    // Write the generated code to a file in the output directory
    let dest_path = Path::new(&out_dir).join("run_all.rs");
    let mut f = File::create(&dest_path)?;
    f.write_all(generated_code.as_bytes())?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    Ok(())
}

fn main() -> io::Result<()> {
    match generate_bin() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    match generate_run_all_rs() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
