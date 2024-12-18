use std::{thread, time::Duration};
use chrono::{Datelike, Timelike, Local};
use reqwest::{Client, Error};
use tokio;

async fn check_in() -> Result<(), Error> {
    let url = "https://markatt-bdd53.web.app/kodex-staff-attendance/check-in";
    let full_name = "Prince Uchenna";
    let email = "Princeuchenna733@gmail.com";
    let phone_number = "09046002329";

    let client = Client::new();
    let res = client.post(url)
        .form(&[
            ("fullName", full_name),
            ("email", email),
            ("phoneNumber", phone_number),
        ])
        .send()
        .await?;

    println!("Response: {:?}", res.status());
    Ok(())
}

fn is_weekday() -> bool {
    let day = Local::now().weekday();
    day != chrono::Weekday::Sat && day != chrono::Weekday::Sun
}

#[tokio::main]
async fn main() {
    loop {
        let now = Local::now();
        if is_weekday() && now.hour() == 8 && now.minute() < 30 {
            println!("Running automation task...");
            if let Err(e) = check_in().await {
                eprintln!("Error during check-in: {:?}", e);
            }
        }
        thread::sleep(Duration::from_secs(60)); // Check every minute
    }
}
