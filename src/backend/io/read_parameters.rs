
use std::error::Error;

use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;

use plotters::prelude::*;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer};


#[derive(Debug, Deserialize)]
struct RGBRow {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Deserialize)]
struct HarborRow {
    first_node: u32,
    second_node: u32,
    harbor_type: u32,
}


pub fn read_csv_to_option<T: 'static, P: AsRef<Path>>(path: P) -> Result<Vec<Option<T>>, Box<dyn Error>> 
where 
    T: Clone + FromStr,  < T as FromStr>::Err: std::error::Error ,
{
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut result = Vec::new();
    for record in rdr.records() {
        let record = record?;
        let value: Option<T> = record[0].parse().ok();
        result.push(value);
    }

    Ok(result)
}

pub fn read_matrix_csv(path: &str) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).delimiter(b',').from_reader(file);
    let mut result = Vec::new();

    for (i, result_record) in rdr.records().enumerate() {
        let record = result_record?;
        if i == 0 { // Skip the header
            continue;
        }
        let row = record.iter()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
        result.push(row);
    }

    Ok(result)
}

pub fn read_harbors_csv(file_path: &str) -> Result<Vec<(u32, u32, u32)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
                            .delimiter(b',')
                            .has_headers(true)
                            .from_reader(file);

    let mut harbor_values = Vec::new();
    for result in rdr.deserialize::<HarborRow>() {
        let record = result?;
        harbor_values.push((record.first_node, record.second_node, record.harbor_type));
    }
    Ok(harbor_values)
}

pub fn read_deserialized_csv<T: 'static>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where 
    T: for<'a> Deserialize<'a>,
{
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
                            .delimiter(b',')
                            .has_headers(true)
                            .from_reader(file);

    let mut ts = Vec::new();
    for result in rdr.deserialize::<T>() {
        ts.push(result?);
    }
    Ok(ts)
}

pub fn deserialize_tuple<'de, D>(deserializer: D) -> Result<(u32, u32), D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.trim_matches(|c| c == '(' || c == ')');
    let mut parts = s.split(',');
    let lat = parts.next().ok_or_else(|| de::Error::custom("missing latitude"))?;
    let lon = parts.next().ok_or_else(|| de::Error::custom("missing longitude"))?;
    let lat = u32::from_str(lat).map_err(de::Error::custom)?;
    let lon = u32::from_str(lon).map_err(de::Error::custom)?;
    Ok((lat, lon))
}