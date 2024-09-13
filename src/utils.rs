use std::path::{Path, PathBuf};
use std::{env, fmt};
use std::{fmt::Write, fs, io::Error};

// define a type called Mod

#[derive(Debug)]
#[allow(dead_code)]
pub struct Mod {
    pub name: String,
    pub ancestors: Vec<String>,
}

impl fmt::Display for Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use "::" to join the ancestors
        let ancestors = self.ancestors.join("::");
        write!(f, "{}::{}", ancestors, self.name)
    }
}

pub fn get_project_root() -> Result<PathBuf, Error> {
    let mut current_dir = env::current_dir()?;
    loop {
        if current_dir.join("Cargo.toml").exists() {
            return Ok(current_dir);
        }

        current_dir = current_dir.parent().unwrap().to_path_buf();
    }
}

// self learning note: 's is a lifetime parameter
// NOTE: this is extremely ugly, don't do that just to remove "Some" while calling the function
//       I am just learning how to use lifetimes
pub fn list_mods<'s>(
    path: &Path,
    root_package: impl Into<Option<&'s str>>,
    filter_list: impl Into<Option<Vec<&'s str>>>,
    exclude_list: impl Into<Option<Vec<&'s str>>>,
) -> Result<Vec<String>, Error> {
    // filter_list: list of modules to include
    // exclude_list: list of modules to exclude

    let root_package = root_package.into().unwrap_or("src") as &str;

    let filter_list = filter_list.into().unwrap_or_default() as Vec<&str>;
    let exclude_list = exclude_list.into().unwrap_or_default() as Vec<&str>;

    // TODO: make it an iterator
    let mut list_of_mods = Vec::new();

    // bfs the src directory
    let mut dirs_to_visit = vec![path.to_path_buf()];

    while !dirs_to_visit.is_empty() {
        let dir = dirs_to_visit.pop().unwrap();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // if file name is in exclude_list, skip
                if let Some(file_name) = path.file_stem() {
                    if !exclude_list.is_empty()
                        && exclude_list.contains(&file_name.to_str().unwrap())
                    {
                        continue;
                    }
                }

                // add to dirs_to_visit if the directory is a module
                if path.join("mod.rs").exists() {
                    dirs_to_visit.push(path.clone());
                }
            } else {
                // file
                if path.ends_with("main.rs") || path.ends_with("lib.rs") || path.ends_with("mod.rs")
                {
                    continue;
                }

                // if file name is in exclude_list, skip
                if let Some(file_name) = path.file_stem() {
                    if !exclude_list.is_empty()
                        && exclude_list.contains(&file_name.to_str().unwrap())
                    {
                        continue;
                    }
                }

                // get all the directories leading to the file, skip the first one (it's usually src)
                let parent_folders = path
                    .ancestors()
                    .skip(1)
                    .map(|f| f.file_name().unwrap_or_default())
                    .collect::<Vec<_>>();

                // build the chain of module calling
                let mut parent_folders_str = String::new();

                let mut is_valid = false;
                for parent_folder in parent_folders
                    .into_iter()
                    .rev()
                    .skip_while(|x| root_package != *x)
                {
                    if filter_list.is_empty()
                        || filter_list.contains(&parent_folder.to_str().unwrap())
                    {
                        is_valid = true;
                    }
                    if parent_folder == "src" {
                        continue;
                    }
                    write!(parent_folders_str, "{}::", parent_folder.to_str().unwrap()).unwrap();
                }

                if !is_valid {
                    continue;
                }

                let file_name = path.file_stem().unwrap().to_str().unwrap();
                list_of_mods.push(parent_folders_str + &file_name);
            }
        }
    }

    Ok(list_of_mods)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_root() {
        let project_root = get_project_root().unwrap();
        assert_eq!(project_root.ends_with("learn-rust"), true);
    }

    #[test]
    fn test_list_all_mods() {
        let project_root = get_project_root().unwrap();
        let src_dir = project_root.join("src");
        let list_of_mods = list_mods(src_dir.as_path(), None, None, vec!["utils"]).unwrap();
        assert_eq!(list_of_mods.len(), 5, "list_of_mods: {:?}", list_of_mods);
    }

    #[test]
    fn test_list_certain_mod() {
        let project_root = get_project_root().unwrap();
        let src_dir1 = project_root.join("src/rust_with_examples_activities/ch01_hello_world");
        let list_of_mods1 = list_mods(src_dir1.as_path(), None, None, vec!["utils"]).unwrap();
        assert_eq!(list_of_mods1.len(), 4, "list_of_mods1: {:?}", list_of_mods1);

        let src_dir2 = project_root.join("src");
        let list_of_mods2 = list_mods(
            src_dir2.as_path(),
            None,
            vec!["ch01_hello_world"],
            vec!["utils"],
        )
        .unwrap();

        assert_eq!(list_of_mods2.len(), 4, "list_of_mods2: {:?}", list_of_mods2);
        assert_eq!(list_of_mods1, list_of_mods2);
    }
}
