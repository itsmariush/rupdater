extern crate confy;
extern crate reqwest;

use std::{env, fs};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io;

use serde::{Serialize, Deserialize};
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize)]
struct RuConfig {
    zipurl : String,
    sigurl : String,
    tmppath: String,
    destpath: String,
    user: String,
    chmod: String,
    pre_cmd: String,
    post_cmd: String,
}

impl Default for RuConfig {
    fn default() -> Self {
        Self {
            zipurl : "http://localhost:8000/update.zip".to_string(),
            sigurl : "http://localhost:8000/update.zip.sig".to_string(),
            tmppath: "/tmp/rupdater".to_string(),
            destpath: "./dest".to_string(),
            user: "sparer".to_string(),
            chmod: "0755".to_string(),
            pre_cmd: "".to_string(),
            post_cmd: "".to_string(),
        }
    }
}

fn unzip_file(file : File, dest : &str) {
    let dest_path = Path::new(dest);
    let mut archive = ZipArchive::new(file).expect("Failed to open Zip file");
    println!("Unzip files to {}", dest_path.to_str().unwrap());
    for i in 0..archive.len() {
        let zfile = archive.by_index(i).expect("Failed get compressed file");
        println!("File {} Name: {}, Last_Modified: {:?}", i, zfile.name(), zfile.last_modified());
        let outpath = match zfile.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        println!("Zipped File name: {}", zfile.name());
        if zfile.name().ends_with('/') {
            println!("Is directory {:?}", dest_path.join(outpath));
            //fs::create_dir_all(&outpath).expect("Could not create directories to unzip");
        }
    }
}

fn download_file(url : &str, dest : &str) -> File {
    let mut response = reqwest::blocking::get(url).expect("Failed connect to server");
    let filename = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");
    let out_path = Path::new(dest);
    // TODO: Find better way to check if diretories exist (maybe a more rustian)
    let file_path: PathBuf = match out_path.try_exists() {
        Ok(k) => match k {
            true => {out_path.join(filename)},
            false => {
                fs::create_dir_all(out_path).expect("Failed to create tmp path");
                out_path.join(filename)
            }
        },
        Err(e) => panic!("{}", e),
    };
    println!("{}", file_path.to_str().unwrap());
    // Open File in read-write-create mode
    let mut out_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(file_path).expect("Failed to open output file");

    io::copy(&mut response, &mut out_file)
            .expect("failed to write content of downloaded file to disk");
    return out_file;
}

fn main() -> Result<(), confy::ConfyError> {
    println!("Starting rUpdater...");
    let args : Vec<String> = env::args().collect();
    if args.len() == 0 {

    }
    
    let conf : RuConfig = confy::load("rupdater")?;
    dbg!(&conf);
    println!("Download zip file from {}", &conf.zipurl);
    let zipfile = download_file(&conf.zipurl, &conf.tmppath);
    unzip_file(zipfile, &conf.tmppath);
    Ok(())
}
