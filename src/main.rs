use std::env;

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
        RuConfig {
            zipurl : "http://localhost:8000/update.zip".to_string(),
            sigurl : "http://localhost:8000/update.zip.sig".to_string(),
            tmppath: "/tmp/rupdater".to_string(),
            destpath: "./dest".to_string(),
            user: "sparer".to_string(),
            chmod: "0755".to_string(),
            pre_cmd: "None".to_string(),
            post_cmd: "None".to_string(),
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    println!("Starting rUpdater...");
    let args : Vec<String> = env::args().collect();

    if args.len() == 0 {

    }
    
    let conf : RuConfig = confy::load("rupdater")?;
    dbg!(conf);
    Ok(())
}
