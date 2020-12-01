#[macro_use]
extern crate glob;
extern crate serde;
extern crate serde_json;

// temporarily publish module
pub mod correlation_table;
pub mod parser;

use glob::glob;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// # Usage
// bundle("src/**/*.css", "dist/style.css")
pub fn bundle(target_path: &str, dist_path: &str) -> () {
    let stylesheets = search_stylesheets(target_path);
    let mut bundle_file = create_empty_bundle_stylesheet(dist_path);
    let mut correlation_table = correlation_table::CorrelationTable::new();

    for stylesheet in stylesheets {
        // generate hasher for class prefix from stylesheet path.
        let mut hasher = DefaultHasher::new();
        stylesheet.hash(&mut hasher);
        let prefix = &hasher.finish().to_string();
        // parse and compile style
        let style_string = std::fs::read_to_string(&stylesheet).unwrap();
        let complied_style = parser::compile(&style_string, prefix).expect("Compile Failed");

        // generate Table of stylesheetpath-classhash.
        correlation_table.append(
            // REFACTOR: Gross...
            stylesheet.into_os_string().into_string().unwrap().clone(),
            prefix.to_string(),
        );
        // output in bundled stylesheet
        bundle_file.write_all(complied_style.as_bytes()).unwrap();
    }

    let json_string = serde_json::to_string(&correlation_table);
    let json_file = create_correlation_table_json("css-modules-rs.table.json");
    serde_json::to_writer(json_file, &correlation_table).unwrap();
    println!("CorrelationTable: {:?}", &json_string);

    return;
}

fn search_stylesheets(target_path: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];
    for entry in glob(target_path).unwrap() {
        match entry {
            Ok(path_buf) => paths.push(path_buf),
            Err(e) => println!("Failed to fetch file: {}", e),
        }
    }
    return paths;
}

fn create_correlation_table_json(path: &str) -> File {
    let path = Path::new(path);
    let display = path.display();
    let file = match File::create(&path) {
        Err(err) => panic!("Couldn't create{}: {}", display, err.to_string()),
        Ok(file) => file,
    };

    return file;
}

fn create_empty_bundle_stylesheet(dist_path: &str) -> File {
    let path = Path::new(dist_path);
    let display = path.display();
    let directory = path.parent().unwrap();
    fs::create_dir_all(directory).unwrap();

    let file = match File::create(&path) {
        Err(err) => panic!("Couldn't create {}: {}", display, err.to_string()),
        Ok(file) => file,
    };

    return file;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::bundle("./src/**/*.css", "./dist/style.css");
    }
}
