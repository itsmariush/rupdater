#![allow(dead_code)]
#![allow(unused_variables)]

extern crate confy;
extern crate reqwest;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io;
use serde::{Serialize, Deserialize};


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

fn download_file(url : &str, dest : &str) {
    let mut response = reqwest::blocking::get(url).expect("Failed connect to server");
    let filename = response
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");
    let out_path = Path::new(dest).join(filename);
    println!("{}", out_path.to_str().unwrap());
    let mut out_file = File::create(out_path).expect("Failed to create output file");
    io::copy(&mut response, &mut out_file).expect("failed to copy content of downloaded file");
}

fn main() -> Result<(), confy::ConfyError> {
    println!("Starting rUpdater...");
    let args : Vec<String> = env::args().collect();

    if args.len() == 0 {

    }
    
    let conf : RuConfig = confy::load("rupdater")?;
    dbg!(&conf);
    println!("Download zip file from {}", &conf.zipurl);
    download_file(&conf.zipurl, &conf.tmppath);
    Ok(())
}
