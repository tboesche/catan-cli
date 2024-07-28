use std::{fs::File, path::Path, str::FromStr};

use csv::ReaderBuilder;
use plotters::style::RGBColor;
use serde::Deserialize;

use std::error::Error;

#[derive(Debug, Deserialize)]
struct RGBRow {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn read_colors_csv(file_path: &str) -> Result<Vec<RGBColor>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
                            .delimiter(b',')
                            .has_headers(true)
                            .from_reader(file);

    let mut rgb_values = Vec::new();
    for result in rdr.deserialize::<RGBRow>() {
        let record = result?;
        rgb_values.push(RGBColor(record.red, record.green, record.blue));
    }
    Ok(rgb_values)
}

pub fn read_csv_to_vector<T: 'static, P: AsRef<Path>>(path: P) -> Result<Vec<T>, Box<dyn Error>> 
where 
    T: Clone + FromStr,  < T as FromStr>::Err: std::error::Error ,
{
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut result = Vec::new();
    for record in rdr.records() {
        let record = record?;
        let value: T = record[0].parse()?;
        result.push(value);
    }

    Ok(result)
}