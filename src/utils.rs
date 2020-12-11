use path_clean::PathClean;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::{collections::hash_map::DefaultHasher, env};

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s: DefaultHasher = Default::default();
    t.hash(&mut s);
    s.finish()
}

pub fn to_abs_path(
    target_path: impl AsRef<Path>,
    current_path: impl AsRef<Path>,
) -> Result<PathBuf, std::io::Error> {
    println!("ToAbsPATH= ===================");
    let target_path = target_path.as_ref();
    let current_path = current_path.as_ref();
    let abs_path = if target_path.is_absolute() {
        target_path.to_path_buf()
    } else {
        // let target_path_buf = target_path.to_path_buf();
        println!("CurrentPath in to abs path {:?}", current_path);
        let mut current_file_path_buf = current_path.to_path_buf();
        println!(
            "CurrentFilePathBuf in to abs path {:?}",
            current_file_path_buf
        );
        current_file_path_buf.pop();
        println!(
            "CurrentFilePathBuf After pop in to abs path {:?}",
            current_file_path_buf
        );
        let current_dir = current_file_path_buf;
        println!("CurrentDir in to abs path {:?}", current_dir);
        println!("Target Path in to abs path {:?}", target_path);
        let result = Path::new(&current_dir).join(target_path);
        println!("Joined Path {:?}", result);
        result
    }
    .clean();

    println!("Final Path {:?}", abs_path);
    println!("ToAbsPATH Finished ================================");

    Ok(abs_path)
}

// pub fn to_abs_path(target_path: impl AsRef<Path>) -> Result<PathBuf, std::io::Error> {
//     let target_path = target_path.as_ref();
//     println!("AbsPass Calling");
//     let abs_path = if target_path.is_absolute() {
//         target_path.to_path_buf()
//     } else {
//         println!("Current File! {}", file!());
//         // let root_dir = match env::current_dir() {
//         //     Ok(dir) => dir,
//         //     Err(err) => return Err(err),
//         // };
//         //
//         let target_path_buf = target_path.to_path_buf();
//         println!("Target PathBuf {:?}", target_path_buf);
//
//         let current_file_path = file!();
//         let mut current_file_path_buff: Vec<&str> = current_file_path.split("/").collect();
//         current_file_path_buff.pop();
//         let current_dir = current_file_path_buff.join("/");
//         Path::new(&current_dir).join(target_path)
//     }
//     .clean();
//
//     Ok(abs_path)
// }
