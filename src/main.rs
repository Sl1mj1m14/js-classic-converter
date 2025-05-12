use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::exit;

use serde::Deserialize;
use thiserror::Error;
use rusqlite::Error;

use mc_classic;
use mc_classic_js::{self, deserialize_data, JSLevel, Settings, Data};

mod convert;

const INPUT_MODE: u8 = 0;
const INPUT_FOLDER: &str = "input";
const INPUT_FILE: &str = "data.sqlite";
const OUTPUT_FOLDER: &str = "output";
const OUTPUT_FILE: &str = "level.dat";

#[derive(Deserialize, Debug)]
struct Config {
    input_settings: Input,
    output_settings: Output
}

#[derive(Deserialize, Debug)]
struct Input {
    input_mode: u8,
    input_folder: String,
    input_file: String
}

#[derive(Deserialize, Debug)]
struct Output {
    output_folder: String,
    output_file: String,
}

#[derive(Error, Debug)]
pub enum GeneralError {
    #[error("Error Parsing Config")]
    TOMLError(#[from] toml::de::Error),

    #[error("File Error")]
    FileError(#[from] std::io::Error),

    #[error("Invalid Json")]
    DeserializeError(#[from] serde_json::Error),

    #[error("Classic Error")]
    ClassicError(#[from] mc_classic::ClassicError),

    #[error("Conversion Error")]
    ConversionError(#[from] convert::ConversionError),

    #[error("Read Error")]
    ReadError(#[from] rusqlite::Error),

    #[error("Could not find {0}")]
    MissingFile(String),

    #[error("Input mode invalid, expected 0 or 1 but found {0}")]   
    InvalidMode(u8)    
}

pub fn main () {

    if !fs::exists("config.toml").unwrap() {
        if let Err(e) = build_settings() {throw(e)}
    }

    let conf = fs::read_to_string("config.toml").unwrap().replace("-", "_");
    let config: Config = match toml::from_str(&conf) {
        Ok(c) => c,
        Err(e) => {
            throw(GeneralError::TOMLError(e));
            exit(1)
        }
    };

    if !fs::exists(&config.input_settings.input_folder).unwrap() {
        if let Err(e) = fs::create_dir(&config.input_settings.input_folder) {throw(GeneralError::FileError(e))}
    }
    if !fs::exists(&config.output_settings.output_folder).unwrap() {
        if let Err(e) = fs::create_dir(&config.output_settings.output_folder) {throw(GeneralError::FileError(e))}
    }

    println!("Loading level");
    if !fs::exists(config.input_settings.input_folder.clone() + "/" + &config.input_settings.input_file).unwrap() {
        throw(GeneralError::MissingFile(config.input_settings.input_folder.clone() + "/" + &config.input_settings.input_file));
    }

    let mut data: Data = Data::new(JSLevel::default(), Settings::default());

    println!("Reading level");
    match config.input_settings.input_mode {
        0 => {
            let saved_game = match mc_classic_js::read_saved_game(config.input_settings.input_folder.clone() + "/" + &config.input_settings.input_file) {
                Ok(c) => c,
                Err(e) => { throw(GeneralError::ReadError(e)); exit(1)}
            };
            let settings = match mc_classic_js::read_settings(config.input_settings.input_folder.clone() + "/" + &config.input_settings.input_file) {
                Ok(c) => c,
                Err(e) => { throw(GeneralError::ReadError(e)); exit(1)}
            };
            data = deserialize_data(saved_game, settings);
        },

        1 => {
            let str: String = std::fs::read_to_string(
                &(config.input_settings.input_folder.clone() + "/" + &config.input_settings.input_file))
                .unwrap()
                .replace("savedGame", "js_level");
            data = match
            serde_json::from_str(&str) {
                Ok(c) => c,
                Err(e) => { throw(GeneralError::DeserializeError(e)); exit(1)}
            };
        }
        _ => throw(GeneralError::InvalidMode(config.input_settings.input_mode))
    }

    println!("Converting level");
    let level: mc_classic::Level = match convert::js_to_classic(data) {
        Ok(c) => c,
        Err(e) => { throw(GeneralError::ConversionError(e)); exit(1)}
    };

    println!("Writing level");
    if let Err(e) = mc_classic::write_level(
        level, config.output_settings.output_folder + "/" + &config.output_settings.output_file, 1
    ) {throw(GeneralError::ClassicError(e))};

    println!("Conversion is complete!");

    println!("Press Enter to Exit");
    let mut s: String = String::from("");
    std::io::stdin().read_line(&mut s).expect("");
    return;

}

fn build_settings () -> Result<(),GeneralError>{
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("config.toml").unwrap();

    file.write("[input-settings]\n".as_bytes())?;
    file.write(format!(r#"input-mode = {INPUT_MODE}"#).as_bytes())?;
    file.write("\n".as_bytes())?;
    file.write(format!(r#"input-folder = "{INPUT_FOLDER}""#).as_bytes())?;
    file.write("\n".as_bytes())?;
    file.write(format!(r#"input-file = "{INPUT_FILE}""#).as_bytes())?;
    file.write("\n\n".as_bytes())?;
    file.write("[output-settings]\n".as_bytes())?;
    file.write(format!(r#"output-folder = "{OUTPUT_FOLDER}""#).as_bytes())?;
    file.write("\n".as_bytes())?;
    file.write(format!(r#"output-file = "{OUTPUT_FILE}""#).as_bytes())?;
    file.write("\n".as_bytes())?;
    return Ok(())
}

fn throw (e: GeneralError) {
    eprintln!("Error: {:#?}", e);
    println!("Press Enter to Exit");
    let mut s: String = String::from("");
    std::io::stdin().read_line(&mut s).expect("");
    std::process::exit(1)
}