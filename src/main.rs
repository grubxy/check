use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::string::String;
use std::{fs, io};

fn main() {
    // get file names
    let mut paths: Vec<PathBuf> = Vec::new();
    let _f = get_files(&mut paths);
    let mut tophash: HashMap<String, u128> = HashMap::new();

    for i in paths {
        println!("pocess {:?}...", i);
        let _result = process(
            i.into_os_string().into_string().unwrap().as_str(),
            &mut tophash,
        );
    }

    // sort and truncate
    let mut v: Vec<_> = tophash.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1));
    let totallines = v.iter().fold(0, |acc, x| acc + x.1);
    println!("total {}", totallines);
    v.truncate(50);
    // wirte to result file
    let mut file = File::create("result.txt").expect("file create fail");
    for (num, count) in v.iter() {
        let percent = (*count * 100) as f64 / totallines as f64;
        let line = format!(
            "number: {}, count:{}, percent:{:.2}% \r\n",
            num, count, percent
        );
        println!("{}", line);
        file.write_all(line.as_str().as_bytes())
            .expect("wite result failed");
        file.sync_all().expect("sync result failed");
    }
}

// check path is txt?
fn check_path(p: PathBuf) -> bool {
    let name = p.into_os_string().into_string().unwrap();
    match name.as_str().strip_suffix(".txt") {
        Some(x) => {
            if x.contains("result") {
                return false;
            } else {
                return true;
            }
        }
        None => false,
    }
}

// get files from this dir
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

// process counts
fn process(s: &str, map: &mut HashMap<String, u128>) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .from_path(s)?;
    for result in rdr.records() {
        let record = result?;
        let number = String::from(record.get(4).unwrap());
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    Ok(())
}
