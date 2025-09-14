use std::fs;

fn main() {

    let paths = fs::read_dir("./public").unwrap(); //only files in public folder can be renamed.

    for path in paths {
        let path = path.unwrap().path();
        let old_name = path.to_string_lossy();
        let new_name = old_name.to_lowercase();

        fs::rename(&*old_name, &new_name).unwrap();

        println!("renamed file: {} -> {}", old_name, new_name);
    }
}
