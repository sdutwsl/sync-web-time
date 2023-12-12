use std::error::Error;
use std::process;
use std::env;

use chrono::Datelike;
use windows::Win32::System::SystemInformation::*;
use serde::Deserialize;
use chrono::{ NaiveDateTime, Timelike };

#[derive(Deserialize)]
struct WebTime {
    #[serde(rename = "sysTime1")]
    _sys_time1: String,
    #[serde(rename = "sysTime2")]
    sys_time2: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result;
    if args.len() == 1 {
        result = set_web_time();
    } else if args.len() == 2 {
        result = set_argument_time();
    } else {
        println!("To many arguments.");
        result = Ok(());
    }

    match result {
        Ok(_) => { println!("Time set successful.") }
        Err(e) => {
            error_handler(e);
        }
    }
}

fn set_argument_time() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    set_windows_system_time(NaiveDateTime::parse_from_str(&args[1], "%Y-%m-%d %H:%M:%S")?)?;
    Ok(())
}

fn error_handler(err: Box<dyn Error>) {
    println!("Some error occured: {:#?}", err);
    process::exit(0);
}

fn set_web_time() -> Result<(), Box<dyn Error>> {
    set_windows_system_time(fetch_web_time()?)?;
    Ok(())
}

fn set_windows_system_time(date_time: NaiveDateTime) -> Result<(), Box<dyn Error>> {
    unsafe {
        let mut lt = GetLocalTime();
        lt.wDay = date_time.day() as u16;
        lt.wMonth = date_time.month() as u16;
        lt.wYear = date_time.year() as u16;
        lt.wHour = date_time.hour() as u16;
        lt.wMinute = date_time.minute() as u16;
        lt.wSecond = date_time.second() as u16;
        SetLocalTime(&lt)?;
    }
    Ok(())
}

fn fetch_web_time() -> Result<NaiveDateTime, Box<dyn Error>> {
    let body = reqwest::blocking::get("https://quan.suning.com/getSysTime.do")?;
    let web_time: WebTime = body.json()?;
    let date_time: NaiveDateTime = NaiveDateTime::parse_from_str(
        &web_time.sys_time2,
        "%Y-%m-%d %H:%M:%S"
    )?;
    Ok(date_time)
}
