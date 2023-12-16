use windows::Win32::System::SystemInformation::*;
use serde::Deserialize;
use chrono::{ NaiveDateTime, Timelike };

#[derive(Deserialize, Debug)]
struct WebTime {
    #[serde(rename = "sysTime1")]
    sys_time1: String,
    #[serde(rename = "sysTime2")]
    sys_time2: String,
}

fn main() {
    let body = reqwest::blocking
        ::get("https://quan.suning.com/getSysTime.do")
        .expect("Failed to get internet time.");
    println!("Internet time acquired, raw response:\n {:#?}", body);

    let web_time: WebTime = body.json().expect("Failed to parse json.");
    println!("JSON time parsed, timestamp: {} ,time: {}", web_time.sys_time1, web_time.sys_time2);

    let data_time: NaiveDateTime = NaiveDateTime::parse_from_str(
        &web_time.sys_time2,
        "%Y-%m-%d %H:%M:%S"
    ).expect("Failed to parse time.");
    println!("Time object generated: {:#?}", data_time);

    unsafe {
        let mut lt = GetLocalTime();
        lt.wHour = data_time.hour() as u16;
        lt.wMinute = data_time.minute() as u16;
        lt.wSecond = data_time.second() as u16;
        SetLocalTime(&lt).expect("Feailed to set time.");
    }

    println!("System time sync!")
}
