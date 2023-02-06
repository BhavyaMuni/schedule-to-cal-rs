extern crate reqwest;
extern crate google_calendar3 as calendar3;
use calendar3::{CalendarHub, hyper::{self, Client}, hyper_rustls};
// use calendar3::api::Channel;
// use calendar3::{Error};
// use reqwest::Response;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = reqwest::Client::builder().build()?;
    let token;
    match std::env::var("METRO_LOGIN"){
        Ok(val) => token = val,
        Err(_) => panic!("Set env variables")
    }
    client.get(format!("https://myschedule.metro.ca/api/Login/{}", token)).send()
        .await?
        .json::<Value>()
        .await?;
        
    let _result = client.get("https://myschedule.metro.ca/api/Employee/").send()
        .await?
        .json::<Value>()
        .await?;
    // match get_shifts(result){
    //     Ok(res) => println!("{:?}", res),
    //     Err(_) => panic!("hell")
    // }    
    authenticate().await;

    Ok(())
}

// fn get_shifts(response: Value) -> Result<Vec<String>, Box<dyn std::error::Error>>{
//     let times = (&response["WorkTime"]).as_array();
//     let objects;
//     let mut times_to_add = vec!();

//     match times{
//         Some(x) => objects = (x.as_slice()[x.len()-7..]).to_vec(),
//         None => panic!("No value")
//     }

//     for obj in objects{
//         times_to_add.push(obj["DailyShift"].to_string());
//     }
    
//     // println!("{:#?}", times_to_add);
//     return Ok(times_to_add);
// }

async fn authenticate(){
    let hub = CalendarHub::new(std::env::var("API_KEY").expect("test"), Client::new());
    let calendar_list = hub.calendar_list().list().doit().await;

    println!("Calendars: {:?}", calendar_list);

}