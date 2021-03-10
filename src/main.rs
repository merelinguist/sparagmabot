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
        "The poets record this struggle from within a consciousness—perhaps new in the world—of the body as a unity of limbs, senses and self, amazed at its own vulnerability.",
        "It was another small triumph, to think that you had wiped away her paint.",
        "You could take the Reverend Daughter out of Drearburh, but in a pinch she could still be counted on to scope out every angle of unfair advantage.",
        "Mama, mama\nBe with me\nWith the music in your breast\nIn your glittering evening dress\nAnd the white flag in your fist trembling",
        "I’m a short-wave radio\nDo you copy?\nI’m a flash of light\nFrom the radar tower to the runway\nIf I love you\nI’m gonna do it semi-automatically\nDo you blame me?",
        "You were dead when they found you\nOn the floor of your apartment\nYou were dead, and your drum set stood there",
        "This is just to tell you\nThat I wear your dress sometimes\nThe one you made with the gold brocade\nAnd the empire waist line\nYou fit it to your figure",
        "This is just to tell you\nThat I wear your dress sometimes\nWear it down to the bar in town\nAnd I dance around all night\nTalking and joking\nSwearing and smoking\nLike any stranger in the crowd\nAnd nobody stares\nAnd nobody cares to tell me I’m not allowed\nI am allowed",
        "And my body by the letter of the law is still my own\nWhen I lay down in the darkness\nUnburdened and alone",
        "Lord have mercy on my mind\nMercy on my memory",
        "I’m lying ’neath the same Virginia sky\nWhere she lay beside me, biding time\nTrying to abide me\nEvery night when the night was long\nShe was clinging to me",
        "Told me twice that her love was strong\nStronger than the love in old love songs\nShe was singing to me",
        "I brought in the winter squash\nI brought in the melon\nCortland, empire, macintosh\nOn the afternoon before the frost\nI could feel it coming",
        "I can see her now in her flowery clothes\nAll of those things I bought her\nTrailing her perfume wherever she goes\nCross the rolling water",
        "Wore we then our warmest capes\nWore we then our walking shoes\nOpen wide the city gates\nAnd let us through!",
        "I have loved you for so long\nEven when I could only do you wrong\nGo see if they have our song\nOn the jukebox over there",
        "The shepherd rode the yellow rows\nThe clouds above and the fields below\nUntil the bales had all been tied\nThen homeward turned to find his wife",
        "How many times her name he called\nAnd no replying would she make\nHer breath it cameth not at all\nShe would not rise from where she lay",
        "He turned the seed into the ground\nHe brought the flock to feed thereon\nHe held the cleaver and the plow\nAnd the shepherd's work was never done",
        "Antigone: we begin in the dark and birth is the death of us",
        "Eros, no one can fight you\nEros, you clamp down on every living thing\non girls’ cheeks on oceans on wild fields\nnot even an immortal can evade you\ncertainly not a creature of the day",
        "you change the levels of a person’s mind\nthis Haimon crisis is all your doing\nyou shook his blood\nyou glow on girls’ eyelids\nwho cares about the laws of the land\nAphrodite, you play with us you",
        "I like a good argument\nmarrow versus marrow\nyou two could learn from each other",
        "many terribly quiet customers exist but none more\nterribly quiet than Man\nhis footsteps pass so perilously soft across the sea\nin marble winter",
        "Whether apprehended as a dilemma of sensation, action or value, eros prints as the same contradictory fact: love and hate converge within erotic desire.",
        "Triangulation makes both present at once by a shift of distance, replacing erotic action with a ruse of heart and language. For in this dance the people do not move. Desire moves. Eros is a verb.",
        "From her inchoate little poem we learn several things about eros. The reach of desire is defined in action: beautiful (in its object), foiled (in its attempt), endless (in time).",
        "The curve of her cheek—the thick, black lashes that fringed her golden eyes—the thumbprint divot that lay pressed like a kiss within the bow of her lip—",
        "Everything felt dark and strange and incorrect, right down to the still-drying paint her adept had applied to her face. Gideon had not even murmured dissent at this incursion, just got on with spooning porridge into her mouth.",
        "Time to us is sarcasm, a slick treacherous monster with a jaw like a furnace incinerating every moment of our lives.",
        "Eternity utters a day.",
        "Even when the soul is seared, even when no prayer can come out of our tightened throats, the clean, silent rest of the Sabbath leads us to a realm of endless peace, or to the beginning of an awareness of what eternity means."
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
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if &args[1] == "dry" {
            let _line = get_line();

            println!("All quotes okay!");

            return;
        }
    }

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
