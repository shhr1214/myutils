use chrono::prelude::*;
use std::fmt::{Display, Formatter, Result};

fn main() {
    let local = Local::now();
    let dt = local.date();

    // head(dt.month());

    let days = vec![' '; dt.weekday().number_from_sunday() as usize - 1];

    println!("{:?}", days);

    // for i in 0..5 {
    //     println!(
    //         "| {:2} | {:2} | {:2} | {:2} | {:2} | {:2} | {:2} |",
    //         i * 7 + 1,
    //         i * 7 + 2,
    //         i * 7 + 3,
    //         i * 7 + 4,
    //         i * 7 + 5,
    //         i * 7 + 6,
    //         i * 7 + 7
    //     );
    //     println!("|----------------------------------|",);
    // }
}

fn head(month: u32) {
    let month_name = MonthName::from(month);

    println!("|----------------------------------|",);
    println!("|{:^34}|", month_name.to_string());
    println!("|----------------------------------|",);
}

#[derive(Debug)]
enum MonthName {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    UNKNOWN(u32),
}

impl Display for MonthName {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            MonthName::January => write!(f, "January"),
            MonthName::February => write!(f, "February"),
            MonthName::March => write!(f, "March"),
            MonthName::April => write!(f, "April"),
            MonthName::May => write!(f, "May"),
            MonthName::June => write!(f, "June"),
            MonthName::July => write!(f, "July"),
            MonthName::August => write!(f, "August"),
            MonthName::September => write!(f, "September"),
            MonthName::October => write!(f, "October"),
            MonthName::November => write!(f, "November"),
            MonthName::December => write!(f, "December"),
            MonthName::UNKNOWN(n) => write!(f, "Unknown Month: {}", n),
        }
    }
}

impl From<u32> for MonthName {
    fn from(n: u32) -> Self {
        match n {
            1 => MonthName::January,
            2 => MonthName::February,
            3 => MonthName::March,
            4 => MonthName::April,
            5 => MonthName::May,
            6 => MonthName::June,
            7 => MonthName::July,
            8 => MonthName::August,
            9 => MonthName::September,
            10 => MonthName::October,
            11 => MonthName::November,
            12 => MonthName::December,
            n => MonthName::UNKNOWN(n),
        }
    }
}
