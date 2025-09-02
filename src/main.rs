use std::{fs::File};
use std::io::Read;
use std::path::Path;
use chrono::{DateTime, Utc};

#[derive(Debug)]
struct Output {
    cve_number: String,
    software_name: String,
    date: DateTime<Utc>,
    versions: Vec<String>,
    json_file: Path
}


struct User {
    id: i8,
    name: String,

}

#[derive(Debug)]
struct Config {
    start_time: DateTime<Utc>,
    csv_file: String,
    cve_directory: String
}


impl Config {
        pub fn new() -> Config {
            Config { 
                start_time: chrono::offset::Utc::now(),
                csv_file: String::new(),
                cve_directory: String::new()
            }
        }
}


fn main() {
    let file_path = Path::new("./test_data/customer.json");

    let configuration = match check_args() {
        Ok(args) => args,
        Err(message) => 
            {
                std::println!("{}", message);
                return;
            }
    };
    std::println!("start_time: {}", configuration.start_time);
    std::println!("csv_file: {}", configuration.csv_file);
    std::println!("cve_directory: {}", configuration.cve_directory);

    match  read_json_file(file_path){
        Ok(data) => std::println!("{}", data),
        Err(message) => std::println!("{}",message)
    }
    
}

fn check_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let mut arg : Config = Config::new();

    std::println!("args: {}", args.len());
    if(args.len() < 4) {
        return Err("please add 3 args: <start_time> <csv_path> <cve_directory>".to_string());
    }

    // read start_time and parse to datetime utc
    if !args[1].is_empty() {
        let date_string: String = format!("{} 00:00:00", &args[1]);
        // std::de!("{}",date_string);
        arg.start_time  = match chrono::NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S") {
            Ok(datetime) => datetime.and_utc(),
            Err(message) => return Err(String::from(format!("failed to parse string to datetime: {}", message)))
        };

    }
    // std::println!("{}", arg.start_time.to_string());

    // read csv_path
    if !args[2].is_empty() {
        
    }

    // read cve_directory
    if !args[3].is_empty() {
        
    }   
    return Ok(arg);
}


fn read_json_file(file_path: &Path) -> Result<serde_json::Value, String> {
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(message) => return Err(String::from(format!("failed to open file: {}", message)))
    };
    
    let mut data = String::new();

    match file.read_to_string(&mut data)   {
        Ok(_) => match serde_json::from_str(&data) {
                    Ok(json_data) => return json_data,
                    Err(message) => return Err(String::from(format!("failed to convert content to str: {}", message)))
                },
        Err(message) => return Err(String::from(format!("failed to read file: {}", message)))
    };
}
