use clap::{Command, Arg};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Write;
use colored::*;


fn display_logo() {
    println!(
        "{}",
        r#"
 __      _________        _         
 \ \    / |__   __|      | |        
  \ \  / /   | |___ _   _| |__  ___ 
   \ \/ /    | / __| | | | '_ \/ __|
    \  /     | \__ | |_| | |_) \__ \
     \/      |_|___/\__,_|_.__/|___/
                                  
	Author:  @ahmedhamdy0x
        YouTube: Gentil Security
        Version: 0.1.0
                                  
                                    
        "#.truecolor(169, 169, 169) 
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    display_logo();

    let matches = Command::new("VTsubs")
        .version("1.0")
        .author("Ahmed Hamdy")
        .about("VirusTotal subscription checker")
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .help("The domain to check")
                .value_parser(clap::value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("api")
                .short('a')
                .long("api")
                .value_name("API_KEY")
                .help("Your VirusTotal API key")
                .value_parser(clap::value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("File to write the results to")
                .value_parser(clap::value_parser!(String))
                .default_value("output.txt"), 
        )
        .get_matches();


    let target_domain = matches.get_one::<String>("domain").unwrap();
    let api_key = matches.get_one::<String>("api").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();


    println!("Domain: {}", target_domain);
    println!("API Key: {}", api_key);
    println!("Output File: {}
    ", output_file);


    let client = Client::new();
    let url = format!(
        "https://www.virustotal.com/vtapi/v2/domain/report?apikey={}&domain={}",
        api_key, target_domain
    );

    let response = client
        .get(&url)
        .send()?
        .json::<serde_json::Value>()?;


    if let Some(subdomains) = response.get("subdomains") {
        for subdomain in subdomains.as_array().unwrap() {
            if let Some(subdomain_str) = subdomain.as_str() {
                
                println!(
                    "{} Found Subdomain: {}{}",
                    "[+]".color("magenta"), 
                    "https://".green(), 
                    subdomain_str.green() 
                );
            }
        }
    } else {
        println!("No subdomains information found in the API response.");
    }

    let mut file = File::create(output_file)?;
    if let Some(subdomains) = response.get("subdomains") {
        for subdomain in subdomains.as_array().unwrap() {
            if let Some(subdomain_str) = subdomain.as_str() {
                writeln!(file, "https://{}", subdomain_str)?;
            }
        }
    } else {
        writeln!(file, "No subdomains information found.")?;
    }

    Ok(())
}
