use rss::Channel;
use std::error::Error;

async fn get_rss_feed(feed_url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(feed_url).await?.bytes().await?;
    let channel: Channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn make_db() -> Result<sqlite::Connection, Box<dyn Error>> {
    let connection: sqlite::Connection = sqlite::open("./rss_feeds.db")?;

    let query: &str = "
    CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);

    ";
    connection.execute(query)?;

    Ok(connection)
}

//TODO insert channels into db
fn insert_into_db() {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let channel = get_rss_feed("https://www.cbc.ca/cmlink/rss-canada-newfoundland").await?;
    make_db()?;
    Ok(())
}
