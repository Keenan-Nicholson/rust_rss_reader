use rss::Channel;
use std::error::Error;

async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www.cbc.ca/cmlink/rss-canada-newfoundland")
        .await?
        .bytes()
        .await?;

    let channel: Channel = Channel::read_from(&content[..])?;

    channel.write_to(::std::io::sink()).unwrap();
    let string = channel.to_string();

    println!("{}", string);

    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    example_feed().await?;
    Ok(())
}
