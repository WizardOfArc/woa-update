use std::cmp::{Ordering, PartialOrd};
use std::path::Path;

use csv;
use serde::{Deserialize, Serialize};

use crate::pagestructs::{FooterLink, Page};

type CountdownResult<T> = std::result::Result<T, CountdownError>;

#[derive(Debug)]
pub enum CountdownError {
    UnableToLoadCSV,
    UnableToLoadJSON,
    UnableToParseJSON,
    UnableToSaveJSON,
    UnableToWriteJSON,
}

// for reading the CSV
#[derive(Serialize, Deserialize, Debug)]
struct CountDownItem {
    label: String,
    gaeilge: String,
    start_time: String, //  (of form: "2027-02-02T00:00:00Z"),
    end_time: String,   // (of form: "2027-02-02T00:00:00Z")
}

impl Ord for CountDownItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time
            .cmp(&other.start_time)
            .then(self.gaeilge.cmp(&other.gaeilge))
    }
}

impl PartialEq for CountDownItem {
    fn eq(&self, other: &Self) -> bool {
        self.start_time == other.start_time && self.gaeilge == other.gaeilge
    }
}

impl PartialOrd for CountDownItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for CountDownItem {}

#[derive(Serialize, Deserialize, Debug)]
struct TimePageForJson {
    page: Page,
    footer_links: Vec<FooterLink>,
    countdowns: Vec<CountDownItem>,
}

impl TimePageForJson {
    fn update_countdowns(&mut self, countdowns: Vec<CountDownItem>) {
        println!("Updating countdown items with {:?}", countdowns);
        self.countdowns = countdowns;
    }

    fn update_from_csv(&mut self, countdown_csv: &Path) -> CountdownResult<()> {
        let mut records: Vec<CountDownItem> = vec![];
        let reader = csv::Reader::from_path(countdown_csv);
        for result in reader
            .map_err(|_| CountdownError::UnableToLoadCSV)?
            .deserialize()
        {
            match result {
                Ok(record) => {
                    println!("Pushing {:?}", record);
                    records.push(record);
                }
                Err(err) => {
                    println!("Error reading row: {:?}", err)
                }
            }
        }
        records.sort();
        self.update_countdowns(records);
        Ok(())
    }

    fn to_json_string(&self) -> CountdownResult<String> {
        serde_json::to_string(self).map_err(|_| CountdownError::UnableToWriteJSON)
    }

    fn from_json_string(json_string: &str) -> CountdownResult<TimePageForJson> {
        serde_json::from_str(json_string).map_err(|_| CountdownError::UnableToParseJSON)
    }

    fn save_to_file(&self, dest_file: &Path) -> CountdownResult<()> {
        let json_string = self.to_json_string()?;
        std::fs::write(dest_file, json_string).map_err(|_| CountdownError::UnableToSaveJSON)
    }

    fn from_file(source_file: &Path) -> CountdownResult<TimePageForJson> {
        let file_contents =
            std::fs::read_to_string(source_file).map_err(|_| CountdownError::UnableToLoadJSON)?;
        TimePageForJson::from_json_string(&file_contents)
    }
}

pub fn update() -> CountdownResult<()> {
    let data_dir_path_buff = Path::new("/")
        .join("Users")
        .join("wizar")
        .join("GitRepos")
        .join("TeraTemplateStuff")
        .join("data");

    let countdowns_csv = data_dir_path_buff.join("countdowns.csv");
    let time_json = data_dir_path_buff.join("time.json");
    let mut time_page = TimePageForJson::from_file(&time_json)?;
    time_page.update_from_csv(&countdowns_csv)?;
    time_page.save_to_file(&time_json)
}
