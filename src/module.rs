use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};
use std::io;

fn is_file(s: &str) -> bool {
    s.ends_with(".rs")
}

pub fn gen_module(mut name: String, private: bool, working_dir: &mut PathBuf) {
    // This makes sure that the name ends with .rs if not a directory
    if !name.ends_with('/') {
        name.push_str(".rs")
    }

    // Check if we are at project root
    working_dir.push("Cargo.toml");
    if working_dir.exists() {
        working_dir.pop();
        working_dir.push("src");
    } else {
        working_dir.pop();
    }

    for dir in name.split('/') {
        working_dir.push(dir);
        let res = if is_file(dir) {
            gen_file_module(working_dir.clone()).err()
        } else {
            gen_folder_module(working_dir.clone()).err()
        };

        if res.is_some() {
            let err = res.unwrap();
            if err.kind() == io::ErrorKind::AlreadyExists {
                println!("Skipping, folder already exists.");
                continue;
            }

            println!("Unexpected error: {}", err)
        }

        update_modrs(&mut working_dir.clone(),
                     generate_modstring(dir.to_string(), private)).
            expect("Unable to update the generated mod.rs");
    }
}

fn gen_file_module(target_path: PathBuf) -> Result<fs::File, io::Error> {
    println!("Creating empty file: {}", target_path.display());
    fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(target_path)
}

fn gen_folder_module(mut target_path: PathBuf) -> Result<fs::File, io::Error> {
    println!("Creating directory: {}", target_path.display());
    try!(fs::create_dir(&target_path));

    target_path.push("mod.rs");
    gen_file_module(target_path)
}

fn generate_modstring(name: String, private: bool) -> String {
    let mod_name = if name.ends_with(".rs") {
        name.replace(".rs", "")
    } else {
        name.clone()
    };

    if private {
        return format!("mod {};\n", &mod_name);
    }

    format!("pub mod {};\n", &mod_name)
}

// This function is definitely a feelsbadman.jpg
// There has got to be a better way to truncate the already open file...
// TODO: Investigate if this can be optimized to remove unnecessary disk IO
fn update_modrs(target: &mut PathBuf, mut modstring: String) -> Result<(), io::Error> {
    // We want to test at the parent level.
    target.pop();
    what_to_update(target);

    // Add this block so we destruct f when we are done with it
    {
        let mut f = try!(fs::File::open(&target));

        // Read all the contents of our target file
        let mut current_contents = String::new();
        try!(f.read_to_string(&mut current_contents));

        // Add our mod statement to top of the file
        modstring.push_str(&current_contents);
    }

    println!("Updating: {}", target.display());

    let mut new_file = try!(fs::File::create(target));
    try!(new_file.write_all(modstring.as_bytes()));
    Ok(())
}

fn what_to_update(target_path: &mut PathBuf) {
    let targets = ["mod.rs", "lib.rs", "main.rs"];
    for target in &targets {
        target_path.push(target);

        if target_path.exists() {
            break;
        }

        target_path.pop();
    }
}
