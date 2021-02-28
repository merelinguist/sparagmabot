use egg_mode::tweet::DraftTweet;
use rand::seq::SliceRandom;
use std::env;

// TODO: read the quotes from a text file
fn get_line() -> &'static str {
    let quotes = [
        "You arrive on the scene, like a message from God\nListen to the people applaud\nThis is what you were born to do",
        "If I cannot take you for a liar or a lover\nI’ll take you for my brother in arms",
        "My nose isn’t bleeding!",
        "Come on in\nWe haven’t slept for weeks\nDrink some of this\nIt’ll put color in your cheeks",
        "Summer went the way of spring\nWinter’s waiting in the wings\nAnd we haven’t saved anything\nBut that’s alright",
        "I watch you through a smoky haze\nA secret smile on your face\nI’m sorry if I stare",
        "Clearly I remember when\nI used to scratch my poems\nOn the backs of other lovers in\nThe darkness of my mind",
        "Lord have mercy on my mind\nMercy on my memory\nI’m lying ’neath the same Virginia sky\nWhere she lay beside me, biding time\nTrying to abide me",
        "When he said\nWhen he said that my body he’d not miss\nI became a sculptress",
        "I’ve seen Venus, Venus coming\nShe come down in a cloud machine\nAnd I believe\nI believe that she is the only woman I ever seen",
        "In my life\nI hope I lie, and tell everyone you were a good wife\nAnd I hope you die\nI hope we both die",
        "I will keep bringing the ball to you. Just trust me and jump.",
        "It wasn’t that he was the first friend I’d made. He was a partner.",
        "I wish I could receive, toss, and spike, all by myself.",
        "I love you, Hinata.",
        "The Saint of Duty exploded outward as your construct emerged from his abdomen. Your soup was watery and mediocre, as soup went, but as a delivery method for gelid explosives—marrow rendered through so much water as to not pass comment—it was perfect.",
        "Despite the fact that you had said ‘not being cocked at all,’ Ianthe only slurped angrily at her soup, making a sound like custard going down a flute.",
        "It was in this abandoned state that the cavalier of the Ninth House ate two breakfasts, starved of both protein and attention, dark glasses slipping on her nose as she drank another bowl of soup.",
        "You were very aware of her nonetheless: of her skimmed-milk hair and discontented mouth, and of the amber satin she wore that made her arm so gold and her veins so green.",
        "She looked at you, quiet, and perhaps even a little lost; and she said: ‘I can’t tell if you’re a once-in-a-lifetime genius, an insane imbecile, or both.’",
        "If Ianthe Tridentarius knelt beside you then, no matter with what sugary contempt or filigreed Third condescension, you would press your diminished bloody terror into her; you would creep naked into her lap, shamelessly, and weep.",
        "Raskolnikov felt sick, but he couldn’t say why\nWhen he saw his face reflected in his victim’s twinkling eye",
        "Snakes in the grass beneath our feet, rain in the clouds above\nSome moments last forever, but some flare out with love, love, love",
        "The most remarkable thing about coming home to you\nIs the feeling of being in motion again",
        "The most remarkable thing about you\nStanding in the doorway is that it's you\nAnd that you are standing in the doorway",
        "And I am frozen with joy right where I stand\nThe world throws its light underneath your hair\n40 miles from Atlanta\nThis is nowhere",
        "Lean in close to my little record player on the floor\nSo this is what the volume knob’s for",
        "And I won't get better\nBut someday I’ll be free\n’Cause I am not this body\nThat imprisons me",
        "Hail Satan!",
        "Eros is an issue of boundaries. He exists because certain boundaries do. In the interval between reach and grasp, between glance and counterglance, between ‘I love you’ and ‘I love you too,’ the absent presence of desire comes alive.",
        "The poets record this struggle from within a consciousness—perhaps new in the world—of the body as a unity of limbs, senses and self, amazed at its own vulnerability."
    ];

    for (_index, quote) in quotes.iter().enumerate() {
        if quote.len() > 280 {
            panic!("{} is too long", quote);
        }
    }

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
