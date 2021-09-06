use chrono::{Date, Datelike, LocalResult, Month, TimeZone, Utc, Weekday};
use num_traits::FromPrimitive;
use wasm_bindgen::prelude::*;

type Year = i32;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub struct Calendar {
    start_date: (Month, Year),
    end_date: (Month, Year),
}

const WEEKYDAY_ORDER: [Weekday; 7] = [
    Weekday::Sun,
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
];

impl Calendar {
    pub fn new(start_month: Month, start_year: Year, end_month: Month, end_year: Year) -> Self {
        if start_year > end_year {
            panic!("the start year was greater than the end year");
        }

        if start_year == end_year
            && start_month.number_from_month() >= end_month.number_from_month()
        {
            panic!("the start and end date are the same");
        }

        Self {
            start_date: (start_month, start_year),
            end_date: (end_month, end_year),
        }
    }
}

#[wasm_bindgen]
impl Calendar {
    pub fn try_new(start_month: u32, start_year: i32, end_month: u32, end_year: i32) -> Self {
        Calendar::new(
            Month::from_u32(start_month).unwrap(),
            start_year,
            Month::from_u32(end_month).unwrap(),
            end_year,
        )
    }

    pub fn render(&self) -> String {
        let (start_m, start_y) = self.start_date;
        let (end_m, end_y) = self.end_date;

        let mut result = String::from(r#"<div class="calendar">"#);
        let mut curr_month = start_m;
        let mut curr_year = start_y;

        loop {
            result.push_str("<div class=\"month\">");
            let month_calendar = build_month(curr_month, curr_year as i32);
            result.push_str(&month_calendar);
            result.push_str("</div>");

            // we are at the end of the cycle! Time to break out
            if curr_year == end_y && curr_month == end_m {
                break;
            }

            // a new year! update the year we are working on
            if curr_month.succ() == Month::January {
                curr_year += 1;
            }

            curr_month = curr_month.succ();
        }
        result.push_str("</div>");
        result
    }
}

#[wasm_bindgen]
pub fn try_build_month(month: u32, year: i32) -> String {
    let m: Month = Month::from_u32(month).unwrap();

    build_month(m, year)
}

pub fn build_month(month: Month, year: i32) -> String {
    let mut result = format!("<table><caption>{}</caption><thead><tr><th>Sunday</th><th>Monday</th><th>Tuesday</th><th>Wednesday</th><th>Thursday</th><th>Friday</th><th>Saturday</th></tr></thead>", month.name());

    let mut current_day = 1;
    let start_date = Utc.ymd(year, month.number_from_month(), current_day);
    let mut end_of_month = false;

    result.push_str("<tbody>");

    loop {
        result = result + "<tr>";
        WEEKYDAY_ORDER.iter().for_each(|d| {
            if current_day == 1 && start_date.weekday() != *d {
                // not yet at current day
                result.push_str("<td class=\"previous-month\"></td>");
                return;
            }

            // check to see if the current day number is valid in the month
            let maybe_date = Utc.ymd_opt(year, month.number_from_month(), current_day);
            if maybe_date == LocalResult::None {
                // invalid date, we are at the end of the month
                result.push_str("<td class=\"next-month\"></td>");
                end_of_month = true;
                return;
            }

            result.push_str(&format!(
                r#"<td class="current-month">{}</td>"#,
                current_day
            ));

            current_day += 1;
        });
        result = result + "</tr>";

        if end_of_month {
            break;
        }
    }

    result.push_str("</tbody></table>");
    result
}
