use egg_mode::tweet::DraftTweet;
use rand::seq::SliceRandom;
use std::env;

// TODO: read the quotes from a text file
fn get_line() -> &'static str {
    let quotes = [
        "You arrive on the scene, like a message from God\nListen to the people applaud\nThis is what you were born to do",
        "If I cannot take you for a liar or a lover\nI’ll take you for my brother in arms",
        "My nose isn’t bleeding!",
        "Come on in\nWe haven't slept for weeks\nDrink some of this\nIt’ll put color in your cheeks"
    ];

    return quotes.choose(&mut rand::thread_rng()).unwrap();
}

#[tokio::main]
async fn main() {
    // TODO: allow the environment variables to error properly
    let con_token = egg_mode::KeyPair::new(
        env::var("TWITTER_CONSUMER_API_KEY").unwrap(),
        env::var("TWITTER_CONSUMER_API_SECRET").unwrap(),
    );

    let access_token = egg_mode::KeyPair::new(
        env::var("TWITTER_ACCESS_TOKEN").unwrap(),
        env::var("TWITTER_ACCESS_TOKEN_SECRET").unwrap(),
    );

    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let line = get_line();
    let post = DraftTweet::new(line).send(&token).await.unwrap();

    println!("Tweeted: {}", post.response.text);
}
