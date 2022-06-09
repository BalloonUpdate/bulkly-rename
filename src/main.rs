use std::fs;
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from("rename");
    let format_file = PathBuf::from("format.txt");

    let format_contents = fs::read_to_string(format_file).unwrap()
        .replace("/", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("*", "")
        .replace("?", "")
        .replace("\"", "")
        .replace("<", "")
        .replace(">", "")
        .replace("|", "");
    
    println!("文件命名格式: {}", format_contents);
        
    let files = fs::read_dir(dir).unwrap();

    let mut processed = 0;
    for e in files {
        let e = e.unwrap().path();
        if e.is_file() {
            let filename = e.file_name().unwrap().to_str().unwrap().to_owned();
            
            let result = filename.rfind(".");
            if result == None {
                continue;
            }

            let basename = &filename[..result.unwrap()];
            let suffix = &filename[result.unwrap() + 1..];
            let replaced = format_contents
                .replace("{NAME}", basename)
                .replace("{SUFFIX}", suffix);
            
            let s: PathBuf = e.clone();
            let mut d: PathBuf = e.parent().unwrap().to_path_buf();
            d.push(replaced);

            fs::rename(s, d.clone()).unwrap();

            let sname = e.file_name().unwrap().to_str().unwrap();
            let dname = d.file_name().unwrap().to_str().unwrap();
            println!("{}: {} => {}", processed + 1, sname, dname);

        }
        processed += 1;
    }

}
