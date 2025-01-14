use std::{thread, time::Duration};
use chrono::{Datelike, Timelike, Local, NaiveDateTime};
use reqwest::{Client, Error};
use tokio;
use rand::Rng;

struct CheckInStatus {
    last_check_in: Option<NaiveDateTime>,
    retry_count: u32,
}

async fn check_in(status: &mut CheckInStatus) -> Result<bool, Error> {
    // Don't check in if we already did today
    let now = Local::now().naive_local();
    if let Some(last) = status.last_check_in {
        if last.date() == now.date() {
            println!("Already checked in today at {}", last.time());
            return Ok(false);
        }
    }

    let url = "https://markatt-bdd53.web.app/kodex-staff-attendance/check-in";
    let client = Client::new();
    let res = client.post(url)
        .form(&[
            ("Full Name", "Prince Uchenna"),
            ("Email Address", "princeuchenna733@gmail.com"),
            ("Phone Number", "09046002329"),
        ])
        .send()
        .await?;

    if res.status().is_success() {
        status.last_check_in = Some(now);
        status.retry_count = 0;
        println!("Successfully checked in at {}", now.time());
        Ok(true)
    } else {
        println!("Check-in failed with status: {}", res.status());
        Ok(false)
    }
}

fn is_weekday() -> bool {
    let day = Local::now().weekday();
    day != chrono::Weekday::Sat && day != chrono::Weekday::Sun
}

#[tokio::main]
async fn main() {
    println!("Automation started. Will check in on weekdays at 8:00-8:30 AM");
    
    let mut status = CheckInStatus {
        last_check_in: None,
        retry_count: 0,
    };

    loop {
        let now = Local::now().naive_local();
        if is_weekday() && now.hour() == 8 && now.minute() < 30 {
            if status.last_check_in.is_none() || status.last_check_in.unwrap().date() != now.date() {
                let random_minutes = rand::thread_rng().gen_range(0..30);
                println!("Will check in after {} minutes", random_minutes);
                thread::sleep(Duration::from_secs(random_minutes * 60));
            }
            
            match check_in(&mut status).await {
                Ok(true) => {
                    println!("Check-in successful, sleeping for 30 minutes");
                    thread::sleep(Duration::from_secs(1800)); // Sleep 30 mins
                },
                Ok(false) => {
                    println!("Check-in skipped, sleeping for 5 minutes");
                    thread::sleep(Duration::from_secs(300)); // Sleep 5 mins
                },
                Err(e) => {
                    status.retry_count += 1;
                    let retry_delay = std::cmp::min(status.retry_count * 60, 600); // Max 10 minutes
                    eprintln!("Error during check-in: {:?}, retrying in {} seconds", e, retry_delay);
                    thread::sleep(Duration::from_secs(retry_delay as u64));
                }
            }
        } else {
            thread::sleep(Duration::from_secs(60)); // Check every minute
        }
    }
}
