use csv::ReaderBuilder;
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io};

fn main() {
    // get file names
    let mut paths: Vec<PathBuf> = Vec::new();
    let _f = get_files(&mut paths);

    for i in paths {
        // println!("{:?}", i.into_os_string().into_string().as_str());
        csv_process(i.into_os_string().into_string().unwrap().as_str());
    }
    // if let Err(err) = example() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }
}

fn check_path(p: PathBuf) -> bool {
    if let None = p
        .into_os_string()
        .into_string()
        .unwrap()
        .as_str()
        .strip_suffix(".txt")
    {
        return false;
    }
    true
}

fn get_files(v: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        let path = dir.path();
        if !path.is_dir() {
            if check_path(path.clone()) {
                v.push(path);
            }
        }
    }
    Ok(())
}

fn csv_process(s: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().delimiter(b'|').from_path(s)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record.get(4));
    }
    Ok(())
}
