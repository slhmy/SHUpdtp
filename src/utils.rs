use chrono::*;

pub fn get_cur_naive_date_time() -> NaiveDateTime {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = local.month();
    let day = local.day();
    let hour = local.hour();
    let minute = local.minute();
    let second = local.second();
    NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, second)
}