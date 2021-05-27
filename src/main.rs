use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::string::String;
use std::{fs, io};

fn main() {
    // get file names
    let mut paths: Vec<PathBuf> = Vec::new();
    let _f = get_files(&mut paths);
    for i in paths {
        let mut tophash: HashMap<String, u128> = HashMap::new();
        let mut toplist = process(
            i.into_os_string().into_string().unwrap().as_str(),
            &mut tophash,
        );
        println!("top:{:?}", toplist);
    }
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

fn process(
    s: &str,
    map: &mut HashMap<String, u128>,
) -> Result<Vec<(String, u128)>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().delimiter(b'|').from_path(s)?;
    for result in rdr.records() {
        let record = result?;
        let number = String::from(record.get(4).unwrap());
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    let mut v: Vec<_> = map.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1));
    v.truncate(50);
    Ok(v)
}
