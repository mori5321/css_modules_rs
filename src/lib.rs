#[macro_use]
extern crate glob;
extern crate lazy_static;
extern crate serde;
extern crate serde_json;

// temporarily publish module
mod consts;
pub mod correlation_table;
pub mod loader;
pub mod parser;
mod setting;
pub mod utils;

use glob::glob;
use path_clean::PathClean;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// globとったパス
// ["src/hello.css"]

// abs_path
// 相対パスで書いたら現在のディレクトリ情報から絶対パスに変換してくれる
// ["src/hello.css"] -> ["src/src/hello.css"]

// # Usage
// bundle("src/**/*.css", "dist/style.css")
pub fn bundle(target_path: &str, dist_path: &str) -> () {
    let stylesheets = search_stylesheets(target_path);
    let mut bundle_file = create_empty_bundle_stylesheet(dist_path);
    let mut correlation_table = correlation_table::CorrelationTable::new();

    for stylesheet in stylesheets {
        println!(
            "Bundle CSS PathBuf {}",
            stylesheet.to_str().unwrap().to_string()
        );

        println!("StyleSheet {}", &stylesheet.to_str().unwrap().to_string());

        // build.rsから呼ぶという前提でcurrent_pathに何も食わせない。
        let abs_path = utils::to_abs_path(&stylesheet, "").unwrap();
        println!("ABSPATH in bundle: {:?}", abs_path);
        // generate hasher for class prefix from stylesheet path.
        // let mut hasher = DefaultHasher::new();
        // abs_path.hash(&mut hasher);
        // let prefix = hasher.finish().to_string();
        // println!("Prefix in bundle {}", prefix);
        let abs_path_string = abs_path.to_str().unwrap().to_string();
        let suffix = utils::hash(&abs_path_string).to_string();
        println!("Sufffix in bundle {}", suffix);
        // parse and compile style
        let style_string = std::fs::read_to_string(&stylesheet).unwrap();
        let complied_style = parser::compile(&style_string, &suffix).expect("Compile Failed");

        // generate Table of stylesheetpath-classhash.
        correlation_table.append(
            // REFACTOR: Gross...
            abs_path.to_str().unwrap().to_string(),
            suffix,
        );
        // output in bundled stylesheet
        bundle_file.write_all(complied_style.as_bytes()).unwrap();
    }

    let json_file = create_correlation_table_json(&loader::setting().table_path());
    serde_json::to_writer(json_file, &correlation_table).unwrap();

    return;
}

#[macro_export]
macro_rules! bundle {
    ($target_path: expr, $dist_path: expr) => {
        bundle($target_path, $dist_path);
    };
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

#[derive(std::fmt::Debug)]
pub struct Style {
    pub suffix: String,
    pub abs_path: String,
}

impl Style {
    pub fn class(&self, class_name: &str) -> String {
        format!("cssmod-{}-{}", self.suffix, class_name)
    }
}

#[macro_export]
macro_rules! cssmod {
    ($filepath: expr) => {{
        let result = $crate::utils::to_abs_path($filepath, file!());
        match result {
            Err(err) => Err(err),
            Ok(abs_path) => {
                println!("AbsPath in cssmod {:?}", abs_path);

                let abs_path_string = match abs_path.to_str() {
                    Some(str) => str.to_string(),
                    None => panic!("Failed to parse string"),
                };
                let suffix = $crate::utils::hash(&abs_path_string).to_string();
                println!("Suffix in cssmod! {}", suffix);
                // let prefix = abs_path_string;
                // let prefix = table.search_hash(abs_path_string.clone());

                let style = $crate::Style {
                    suffix,
                    abs_path: abs_path_string.clone(),
                };
                Ok(style)
            }
        }
    }};
}

#[test]
fn it_works() {
    // loader::init();
    //
    // println!("Table importing ==================");
    // let table = loader::table();
    // println!("Table from macro {:?}", table);
    // //
    // let table2 = loader::table();
    // println!("Table2 from macro {:?}", table2);
    // let table3 = loader::table();
    // println!("Table3 from macro {:?}", table3);
    bundle!("./src/**/*.css", "./dist/style.css");
    let style = cssmod!("./style/sample.css");
    let styl2 = cssmod!("./header.css");
    // loader::init();
}
