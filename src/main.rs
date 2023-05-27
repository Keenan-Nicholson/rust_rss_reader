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
    CREATE TABLE IF NOT EXISTS item (item TEXT);
    ";
    connection.execute(query)?;

    Ok(connection)
}

fn insert_into_db(connection: sqlite::Connection, channel: Channel) {
    // SQLite hates single quotes
    let escaped_channel: String = channel.to_string().replace("'", "''");

    let query: String = format!(
        "
        INSERT INTO item
        VALUES ('{:#?}');
    ",
        escaped_channel
    );

    connection.execute(query).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let channel: Channel =
        get_rss_feed("https://www.cbc.ca/cmlink/rss-canada-newfoundland").await?;

    let connection: sqlite::Connection = make_db()?;

    insert_into_db(connection, channel);

    Ok(())
}
