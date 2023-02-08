extern crate reqwest;
extern crate google_calendar3 as calendar3;
extern crate dotenv;

use calendar3::{hyper_rustls::{HttpsConnector, self}, CalendarHub, hyper::{self, client::HttpConnector}, api::{Event, EventDateTime}};
use serde_json::Value;
// use reqwest::Response;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    // let client = reqwest::Client::builder().build()?;
    reqwest::get(format!("https://myschedule.metro.ca/api/Login/{}", std::env::var("METRO_LOGIN").expect("error in login token")))
        .await?;

    let mut _shifts = reqwest::get("https://myschedule.metro.ca/api/Employee/").await?.json::<serde_json::Value>().await?;
    if _shifts == Value::Null{
        _shifts = reqwest::get("https://myschedule.metro.ca/api/Employee/").await?.json::<serde_json::Value>().await?;
    }
    let shifts = get_shifts(_shifts).expect("msg");
    
    let hub  = authenticate().await.expect("Error");
    let events = create_events(shifts);
    for e in events{
        let status = hub.events().insert(e, &std::env::var("CALENDAR_ID").expect("set calendar id")).doit().await;
        match status{
            Ok((_, e)) => println!("Event created {}", e.id.expect("msg")),
            Err(e) => println!("Error creating event {}", e)
        }
    }
    Ok(())
}

fn create_events(shifts: Vec<Value>) -> Vec<Event>{
    let mut events = Vec::<Event>::new();
    for shift in &shifts{
        if shift.get("DailySeconds").unwrap().as_i64() > Some(0){
            let time:Vec<&str> = shift["DailyShift"][0].as_str().expect("").split("-").collect();
            let date = shift["StartDate"].as_str().expect("");
            let s_time= time[0];
            let e_time_vec: Vec<&str> = time[1].split(" ").collect();
            let e_time = e_time_vec[0];
            let start_time =  format!("{}{}:00", &date[..16-s_time.len()], &s_time);
            let end_time = format!("{}{}:00", &date[..16-e_time.len()], &e_time);
            let start = EventDateTime {
                date_time: Some(start_time.to_string()),
                time_zone: Some("America/New_York".to_string()),
                ..EventDateTime::default()
            };
            let end = EventDateTime {
                date_time: Some(end_time.to_string()),
                time_zone: Some("America/New_York".to_string()),
                ..EventDateTime::default()
            };
            
            let event = Event{
                summary: Some(String::from("Starbucks")),
                location: Some(String::from("444 Yonge St, Toronto ON M5B 2H3")),
                start: Some(start),
                end: Some(end),
                ..Event::default()
            };
            events.push(event);
        }
    }
    return events;
}

fn get_shifts(response: Value) -> Result<Vec<Value>, ()>{
    // println!("{:#?}", response);
    let times = (&response["WorkTime"]).as_array().expect("Error in response");
    let objects= (&times[times.len()-7..]).to_vec();
    Ok(objects)
}

async fn authenticate() -> Result<CalendarHub<HttpsConnector<HttpConnector>>, Box<dyn std::error::Error>>{
    let key = std::env::var("SERVICE_ACCOUNT").expect("Set service acc credentials");
    println!("{:#?}", key);
    let secret = calendar3::oauth2::parse_service_account_key(&key);
    match secret {
        Ok(s) => {
            let authenticator = calendar3::oauth2::ServiceAccountAuthenticator::builder(s)
            .build()
            .await;
            match authenticator{
                Ok(a) => Ok(CalendarHub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), a)),
                Err(e) =>  Err(Box::new(e))
            }
        },
        Err(e) => Err(Box::new(e))
    }
}