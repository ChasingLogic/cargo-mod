mod utils;

extern crate getopts;

use getopts::Options;

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::io;
use std::fs;
use std::env;

fn print_usage() {
    println!("Work in progress.")
}

fn update_librs(root: &PathBuf, modstring: String) {
    let mut lib_path = root.clone();
    lib_path.push("src");
    lib_path.push("lib.rs");

    let mut librs = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(lib_path.as_path())
                .unwrap(); 
                
    match librs.write_all(modstring.as_bytes()) {
        Ok(_) => println!("Updated lib.rs"),
        Err(e) => println!("Unable to update lib.rs: {}", e)
    }
}

fn update_mainrs(root: &PathBuf, modstring: String) {
    let mut bin_path = root.clone();
    bin_path.push("src");
    bin_path.push("main.rs");

    let mut mainrs = fs::OpenOptions::new()
                .write(true)
                .read(true)
                .open(bin_path.as_path())
                .unwrap();

    let mut current_contents = String::new();
    mainrs.read_to_string(&mut current_contents);

    current_contents.push_str(modstring.as_str());
    match mainrs.write_all(current_contents.as_bytes()) {
        Ok(_) => println!("Updated main.rs"),
        Err(e) => println!("Unable to update main.rs: {}", e)
    }
}

fn add_mod(root: &PathBuf, modstring: String) {
    match utils::project::kind_of_crate(&root) {
        utils::project::CrateType::Both => {
            update_librs(root, modstring.clone());
            update_mainrs(root, modstring)
        },
        utils::project::CrateType::Library => update_librs(root, modstring),
        utils::project::CrateType::Binary => update_mainrs(root, modstring),
    }
}

fn pretty_print_path(root: &PathBuf, target: &PathBuf) -> PathBuf {
    target.strip_prefix(root.parent().unwrap().parent().unwrap()).unwrap().to_path_buf()
}

fn gen_folder_module(name: String, private: bool) {
    let root_path = utils::project::find_project_root();
    let mut our_path = root_path.clone();
    our_path.push("src");
    our_path.push(&name);

    let res = fs::create_dir(our_path.as_path());
    if res.is_err() {
        panic!("Unable to create directory: {}", res.err().unwrap());
    } 
    println!("Created directory: {}", 
             pretty_print_path(&root_path, &our_path).display());

    our_path.push("mod.rs");
    let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(our_path.as_path())
            .unwrap();

    let mod_line = format!("pub mod {};\n", &name);
    let result = f.write_all(mod_line.as_bytes());
    if result.is_err() {
        panic!("Unable to write to file: {}", result.err().unwrap());
    }
    println!("Generated mod file: {}", 
             pretty_print_path(&root_path, &our_path).display()); 

    add_mod(&root_path, mod_line) 
}

// fn gen_module(name: String, private: bool) {
//     let path_string = format!("{}.rs", name);
//     let mut file = open_file_or_panic(&path_string);
// }

fn main() {
    let mut opts = Options::new();
    let args: Vec<String> = env::args().collect();

    opts.optflag("p", "private", "Make the generated module private.");
    opts.optflag("f", "folder", "Generate a folder module instead of a file.");

    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let private = matches.opt_present("p");
    let folder = matches.opt_present("f");
    let name = if !matches.free.is_empty() {
        matches.free[1].clone()
    } else {
        print_usage();
        return
    };

    // if folder {
        gen_folder_module(name, private);
        // return 
    // }

    // gen_module(name, private)
}
mod utils;

extern crate getopts;

use getopts::Options;

use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::io;
use std::fs;
use std::env;

fn print_usage() {
    println!("Work in progress.")
}

fn update_librs(root: &PathBuf, modstring: String) {
    let mut lib_path = root.clone();
    lib_path.push("src");
    lib_path.push("lib.rs");

    let mut librs = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(lib_path.as_path())
                .unwrap(); 
                
    match librs.write_all(modstring.as_bytes()) {
        Ok(_) => println!("Updated lib.rs"),
        Err(e) => println!("Unable to update lib.rs: {}", e)
    }
}

fn update_mainrs(root: &PathBuf, modstring: String) {
    let mut bin_path = root.clone();
    bin_path.push("src");
    bin_path.push("main.rs");

    let mut mainrs = fs::OpenOptions::new()
                .write(true)
                .read(true)
                .open(bin_path.as_path())
                .unwrap();

    let mut current_contents = String::new();
    mainrs.read_to_string(&mut current_contents);

    modstring.push_str(current_contents.as_str());
    match mainrs.write_all(current_contents.as_bytes()) {
        Ok(_) => println!("Updated main.rs"),
        Err(e) => println!("Unable to update main.rs: {}", e)
    }
}

fn add_mod(root: &PathBuf, modstring: String) {
    match utils::project::kind_of_crate(&root) {
        utils::project::CrateType::Both => {
            update_librs(root, modstring.clone());
            update_mainrs(root, modstring)
        },
        utils::project::CrateType::Library => update_librs(root, modstring),
        utils::project::CrateType::Binary => update_mainrs(root, modstring),
    }
}

fn pretty_print_path(root: &PathBuf, target: &PathBuf) -> PathBuf {
    target.strip_prefix(root.parent().unwrap().parent().unwrap()).unwrap().to_path_buf()
}

fn gen_folder_module(name: String, private: bool) {
    let root_path = utils::project::find_project_root();
    let mut our_path = root_path.clone();
    our_path.push("src");
    our_path.push(&name);

    let res = fs::create_dir(our_path.as_path());
    if res.is_err() {
        panic!("Unable to create directory: {}", res.err().unwrap());
    } 
    println!("Created directory: {}", 
             pretty_print_path(&root_path, &our_path).display());

    our_path.push("mod.rs");
    let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(our_path.as_path())
            .unwrap();

    let mod_line = format!("\npub mod {};\n", &name);
    let result = f.write_all(mod_line.as_bytes());
    if result.is_err() {
        panic!("Unable to write to file: {}", result.err().unwrap());
    }
    println!("Generated mod file: {}", 
             pretty_print_path(&root_path, &our_path).display()); 

    add_mod(&root_path, mod_line) 
}

// fn gen_module(name: String, private: bool) {
//     let path_string = format!("{}.rs", name);
//     let mut file = open_file_or_panic(&path_string);
// }

fn main() {
    let mut opts = Options::new();
    let args: Vec<String> = env::args().collect();

    opts.optflag("p", "private", "Make the generated module private.");
    opts.optflag("f", "folder", "Generate a folder module instead of a file.");

    let matches = match opts.parse(&args) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let private = matches.opt_present("p");
    let folder = matches.opt_present("f");
    let name = if !matches.free.is_empty() {
        matches.free[1].clone()
    } else {
        print_usage();
        return
    };

    // if folder {
        gen_folder_module(name, private);
        // return 
    // }

    // gen_module(name, private)
}
