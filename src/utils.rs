use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io::Error};

pub fn get_project_root() -> Result<PathBuf, Error> {
    let mut current_dir = env::current_dir()?;
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return Ok(current_dir);
        }

        current_dir = current_dir.parent().unwrap().to_path_buf();
    }
}

pub fn list_mods(path: &Path, root_pakcage: Option<&str>) -> Result<Vec<String>, Error> {
    let mut list_of_mods = Vec::new();

    let mut dirs_to_visit = vec![path.to_path_buf()];

    while !dirs_to_visit.is_empty() {
        let dir = dirs_to_visit.pop().unwrap();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if path.join("mod.rs").exists() {
                    dirs_to_visit.push(path.clone());
                } else {
                    if path.ends_with("main.rs")
                        || path.ends_with("lib.rs")
                        || path.ends_with("mod.rs")
                    {
                        continue;
                    }

                    // get all the directories leading to the file, skip the first one
                    let parent_folders = path.ancestors().skip(1).collect::<Vec<_>>();

                    // build the chain of module calling
                    let mut parent_folders_str = String::new();
                    let mut flag = false;
                    for parent_folder in parent_folders.iter().rev() {
                        let parent_folder_name = parent_folder.file_name().unwrap_or_default();

                        if !root_pakcage.is_none() && parent_folder_name == root_pakcage.unwrap() {
                            flag = true;
                            parent_folders_str.push_str(&format!("{}::", root_pakcage.unwrap()));
                        }

                        if flag {
                            parent_folders_str
                                .push_str(&format!("{}::", parent_folder_name.to_str().unwrap()));
                        }
                    }

                    let file_name = path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".rs", "");

                    list_of_mods.push(parent_folders_str + &file_name);
                }
            }
        }
    }

    Ok(list_of_mods)
}
