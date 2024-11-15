/*
    TODO: wild card support
    : convert string arguments to vector
*/
use chrono::Local;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
#[derive(Debug)]
struct Hist {
    id: String,
    item: Vec<HistoryItems>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct HistoryItems {
    //  id: u64,
    from: String,
    to: String,
}

pub fn write_history_as_json() -> Result<(), Box<dyn Error>> {
    let c_time = Local::now();
    let _formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
    Ok(())
}

#[allow(dead_code)]
fn test_function_01() {
    let c_time = Local::now();
    let formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
    println!("{}", formatted_time);
}

// https://thelinuxcode.com/rust-json-example/
pub fn write_history() -> Result<(), Box<dyn Error>> {
    // let hist_file = "roxide_history.json".to_owned();
    // let contents = fs::read_to_string(hist_file)?;
    let file = File::open("roxide_history.json").expect("where is the roxide_history.json file?");
    let _reader = BufReader::new(file);
    // let history_from_json: HistoryItems =
    // serde_json::from_reader(reader).expect("reader failed to read the json data");

    // let json: serde_json::Value = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    // println!("got this:\n{}", json);
    // println!("got this:\n{:?}", history_from_json);
    // println!("from: {}", history_from_json.from);
    // println!("to: {}", history_from_json.to);

    /*
    let hist = HistoryItems {
    from: from.to_str().unwrap().to_string(),
    to: to.to_str().unwrap().to_string()
    };
    let filename = "roxide_history.json";
    let history_file = Path::new("roxide_history.json");

    let mut history_data: Vec<HistoryItems> = Vec::new();
    if history_file.exists() {
    let file = File::open(history_file)?;
    let reader = BufReader::new(file);

    history_data = match serde_json::from_reader(reader) {
    Ok(data) => data,
    Err(_) => Vec::new(),
    }
    */
    Ok(())
}
