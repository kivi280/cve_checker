mod cve;

use std::{fs::File};
use std::io::Read;
use std::path::{Path};
use chrono::Datelike;
use cve::{Cve};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///start year
    #[arg(short, long)]
    start_year: i32,

    ///end year
    #[arg(short, long, default_value_t=chrono::Utc::now().year())]
    end_year: i32,

    /// csv_file with softwarenames and version
    #[arg(short='c', long)]
    csv_file: Option<std::path::PathBuf>,

    /// directory of cve files
    #[arg(long, short='C', default_value = ".")]
    cve_dir: Option<std::path::PathBuf>,
}

impl Args {
    fn check_path(&self) -> bool {
        let is_file = match &self.csv_file {
            Some(path) => {
                if !path.is_file(){
                    std::println!("csv_file not found: {}",&path.to_string_lossy());
                }
                return path.is_file();
            }
            None => false
        };

        let is_dir = match &self.cve_dir {
            Some(path) => {
                if !path.is_dir(){
                    std::println!("cve_dir not found: {}", &path.to_string_lossy());
                }
                return path.is_dir();                
            },
            None => false,
        };
        return is_file & is_dir;
    }
}

fn main() { 
    let args = Args::parse();
    if !args.check_path() {
        return;
    }
    let file_path = Path::new("./test_data/cve_test.json");
    
    std::println!("start_year: {}", args.start_year);
    std::println!("csv_file: {:?}", args.csv_file);
    std::println!("cve_directory: {:?}", args.cve_dir);

    for n in args.start_year..=args.end_year {
        std::println!("{}", n);
    }

    match read_json_file(file_path) {
        Ok(data) => std::println!("{}", data),
        Err(message) => std::println!("{}", message)
    }
}

fn read_json_file(file_path: &Path) -> Result<Cve, String> {
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(message) => return Err(String::from(format!("failed to open file: {}", message)))
    };
    
    let mut data: String = String::new();

    match file.read_to_string(&mut data)   {
        Ok(_) => match serde_json::from_str(&data) {
                    Ok(json_data) => return Ok(json_data),
                    Err(message) => return Err(String::from(format!("failed to convert content to str: {}", message)))
                },
        Err(message) => return Err(String::from(format!("failed to read file: {}", message)))
    }
}