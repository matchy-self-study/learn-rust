use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn generate_mod_rs() -> io::Result<()> {
    Ok(())
}

fn generate_run_all_rs() -> io::Result<()> {
    // Get the directory path where the generated code will be placed
    let out_dir = env::var("OUT_DIR").unwrap();
    // Create a string to hold the generated code
    let mut generated_code = String::new();

    generated_code.push_str("pub fn run_all() {\n");

    // list directoreis in src folder
    let src_dir = Path::new("src");

    let mut dirs_to_visit = vec![src_dir.to_path_buf()];

    // while there are directories to visit
    while !dirs_to_visit.is_empty() {
        // get the next directory to visit
        let dir = dirs_to_visit.pop().unwrap();

        // read the directory and itereate over the entries
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // if path contains a mod.rs file then add it to the list of directories to visit
                if path.join("mod.rs").exists() {
                    dirs_to_visit.push(path.clone());
                }
            } else {
                // skip main.rs and lib.rs
                if path.ends_with("main.rs") || path.ends_with("lib.rs") || path.ends_with("mod.rs")
                {
                    continue;
                }

                // get all the directories leading to the file
                let parent_folders = path.ancestors().skip(1).collect::<Vec<_>>();

                let mut parent_folders_str = String::new();
                for parent_folder in parent_folders.iter().rev().skip(2)
                {
                    let parent_folder_name = parent_folder.file_name().unwrap().to_str().unwrap();
                    parent_folders_str.push_str(&format!("{}::", parent_folder_name));
                }

                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(".rs") {
                    let file_name_without_extension = file_name.split('.').next().unwrap();


                    // push a - with 80 characters
                    generated_code.push_str(
                        "   println!(\"--------------------------------------------------------------------------------\");\n",
                    );
                    generated_code.push_str(&format!(
                        "    println!(\"Running {}{}...\n\");\n",
                        parent_folders_str, file_name_without_extension
                    ));

                    generated_code.push_str(&format!(
                        "    {}{}::run();\n",
                        parent_folders_str, file_name_without_extension
                    ));
                }
            }
        }
    }
    generated_code.push_str("}\n");

    // Write the generated code to a file in the output directory
    let dest_path = Path::new(&out_dir).join("run_all.rs");
    let mut f = File::create(&dest_path)?;
    f.write_all(generated_code.as_bytes())?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn main() -> io::Result<()> {
    generate_mod_rs()?;
    generate_run_all_rs()?;
    Ok(())
}
