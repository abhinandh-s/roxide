use chrono::{DateTime, Utc};
use dirs::data_dir;
use std::error::Error;
use std::fs::remove_file;
use std::time::{Duration, UNIX_EPOCH};
use std::vec;

#[allow(dead_code)]
#[derive(Debug)]
struct Trash {
    file: String,
    date: i64,
}

pub trait Metadata {
    fn get_file_name(&self) -> &str;
    fn get_deleted_date(&self) -> i64;
}

impl Metadata for Trash {
    fn get_file_name(&self) -> &str {
        &self.file
    }
    fn get_deleted_date(&self) -> i64 {
        self.date
    }
}

pub fn gc(_date: &i8) -> Result<(), Box<dyn Error>> {
    remove_file(data_dir().unwrap().join("roxide/roxide_history.log"))?;
    Ok(())
}

#[allow(dead_code)]
fn get_items_for_gc(period: Option<u8>) -> Vec<u64> {
    let b = period.is_none();
    if b {
        println!("no time period specifed:\nconsidering 30 as default\ncleaning all files and dirs in trash for longer than 30 days");
    } else {
        println!("considering 30 as default\ncleaning all files and dirs in trash for longer than 30 days");
    }

    let mut access_time_vec: Vec<u64> = vec![20241101064906, 20241101064907, 20241101064905];
    access_time_vec.sort();
    println!("sorted from vec: {:#?}", access_time_vec);
    access_time_vec
}

#[allow(dead_code)]
fn sort_by_date() {
    let mut access_time: [u64; 3] = [20241101064906, 20241101064907, 20241101064905];
    access_time.sort();
    println!("sorted from slice: {:#?}", access_time);
}

#[allow(dead_code)]
fn convert_system_time(tv_sec: u64, tv_nsec: u32) -> String {
    let duration = Duration::new(tv_sec, tv_nsec);
    let _datetime = UNIX_EPOCH + duration;
    let naive_datetime = DateTime::from_timestamp(tv_sec as i64, tv_nsec).unwrap();
    let datetime: DateTime<Utc> =
        DateTime::from_naive_utc_and_offset(naive_datetime.naive_utc(), Utc);
    datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
}

#[allow(dead_code)]
pub fn dry_run() {}
