use crate::consts;
use crate::correlation_table::CorrelationTable;
use crate::setting::Setting;
use crate::utils;
use lazy_static::lazy_static;
use path_clean::PathClean;
use serde_json;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref SETTING: Setting = {
        println!("FILEPATH: {}", consts::SETTING_FILE_PATH);
        let file = File::open(consts::SETTING_FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        let setting: Setting = serde_json::from_reader(reader).unwrap();
        setting
    };
    static ref TABLE: CorrelationTable = {
        let file = File::open(SETTING.table_path()).unwrap();
        let reader = BufReader::new(file);
        let correlation_table: CorrelationTable = serde_json::from_reader(reader).unwrap();
        correlation_table
    };
}

pub fn init() {
    println!("Module Path {}", module_path!());
    println!("Loading correlation table..");
    let _ = TABLE.clone();
    println!("Loaded!")
}

pub fn setting() -> Setting {
    SETTING.clone()
}

fn table() -> CorrelationTable {
    TABLE.clone()
}

// pub fn cssmod(filepath: String) -> Result<Style, std::io::Error> {
//     let abs_path_maybe = utils::to_abs_path(filepath);
//
//     match abs_path_maybe {
//         Ok(abs_path) => {
//             println!("AbsPath on cssmod: {:?}", &abs_path);
//             let mut hasher = DefaultHasher::new();
//             abs_path.hash(&mut hasher);
//             let prefix = &hasher.finish().to_string();
//
//             let style = Style {
//                 prefix: prefix.clone(),
//             };
//             return Ok(style);
//         }
//         Err(err) => Err(err),
//     }
// }
//

// #[macro_export]
// macro_rules! cssmod {
//     ($filepath: expr) => {{
//         use std::collections::hash_map::DefaultHasher;
//         use std::hash::{Hash, Hasher};
//         let result = css_modules_rs::loader::to_abs_path($filepath, file!());
//         match result {
//             Err(err) => Err(err),
//             Ok(abs_path) => {
//                 let mut hasher = DefaultHasher::new();
//                 abs_path.hash(&mut hasher);
//                 let prefix = &hasher.finish().to_string();
//                 let style = css_modules_rs::loader::Style {
//                     prefix: prefix.clone(),
//                 };
//                 Ok(style)
//             }
//         }
//     }};
// }

#[test]
fn cssmod_macro_test() {
    // println!("CSSMOD! Macro Test");
    // let style = cssmod!("hello.css".to_string());
    // println!("StyleStruct!!!!!!!!!!!!: {:?}", style);
}

// macroじゃないとダメ
// コンパイル時にファイルを探して、クラス名に変換する
// pub fn css(filepath: &str) -> Style {
//     // ファイルを読み込む
//     // hash化する
//     //
// }

// interface
// let style = css!(./signup.css);
// // style.hello => .1290icopojo3jklca-hello
// <div class={style.hello} />

// pub fn css()
