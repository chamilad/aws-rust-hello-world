use aws_config::SdkConfig;
use aws_sdk_cloudtrail::operation::list_trails::ListTrailsOutput;
use aws_sdk_cloudtrail::operation::lookup_events::LookupEventsOutput;
use aws_sdk_cloudtrail::primitives::DateTimeFormat;
use aws_sdk_cloudtrail::{
    types::{LookupAttribute, LookupAttributeKey},
    Client, Error,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("initializing client with environment variables");
    let shared_config: SdkConfig = aws_config::load_from_env().await;
    let client: Client = Client::new(&shared_config);

    let region = shared_config.region().unwrap();
    println!("listing existing trails on the region: {}", region);
    let req = client.list_trails();
    let resp: ListTrailsOutput= req.send().await?;

    for trail in resp.trails() {
        println!("{}", trail.name().unwrap());
    }

    print!("press ENTER to proceed");
    let _ = io::stdout().flush();
    let mut proceed_input = String::new();
    io::stdin().read_line(&mut proceed_input).unwrap();
    println!();

    println!("looking up management events with read_only=false");
    let read_only_attrib: LookupAttribute = LookupAttribute::builder()
        .attribute_key(LookupAttributeKey::ReadOnly)
        .attribute_value("false")
        .build()
        .unwrap();

    let lookup_events_req = client.lookup_events().lookup_attributes(read_only_attrib);
    let lookup_events_resp: LookupEventsOutput = lookup_events_req.send().await?;

    println!("-----------------------------------------------------------------------------------------");
    println!("|{:^30}|{:^35}|{:^20}|", "event_id", "username", "event_time");
    println!("-----------------------------------------------------------------------------------------");
    for event in lookup_events_resp.events() {
        let event_name = event.event_name().unwrap();

        // some events do not contain a username
        let username = if event.username() != None {
            event.username().unwrap()
        } else {
            "-"
        };

        let event_time = event
            .event_time()
            .unwrap()
            .fmt(DateTimeFormat::DateTime)
            .unwrap();
        let event_time_formatted = event_time.as_str();

        println!("|{:<30}|{:<35}|{:<20}|", event_name, username, event_time_formatted,);
    }
    println!("-----------------------------------------------------------------------------------------");

    Ok(())
}
