use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::bson::Document;
use pwhash::bcrypt;

fn get_hashed_var(key: &str) -> String {
    dotenv().ok();
    bcrypt::hash(std::env::var(key).expect("No_Key_As_Well")).expect("No_Key")
}

fn get_var(key: &str) -> String {
    std::env::var(key).expect("NO KEY LIKE THAT LIL BRO")
}

pub async fn seed(mongo: &mongodb::Client) -> Result<(), mongodb::error::Error> {
    let db = mongo.database("linear");
    let collection = db.collection::<Document>("Questions");

    let data: Vec<Document> = vec![
        doc! {
            "question": "What is the meaning of life?",
            "level": 0,
            "answer": get_hashed_var("ANSWER_1"),
            "hint": get_var("HINT_1"),
            "giveaway": get_var("GIVEAWAY_1"),
        },
        doc! {
           "question": "What is not the meaning of life?",
            "level": 1,
            "answer": get_hashed_var("ANSWER_2"),
            "hint": get_var("HINT_2"),
            "giveaway": get_var("GIVEAWAY_2"),
        },
        doc! {
           "question": "Who is the best programmer?",
            "level": 2,
            "answer": get_hashed_var("ANSWER_3"),
            "hint": get_var("HINT_3"),
            "giveaway": get_var("GIVEAWAY_3"),
        },
        doc! {
           "question": "Who is the worst programmer?",
            "level": 3,
            "answer": get_hashed_var("ANSWER_4"),
            "hint": get_var("HINT_4"),
            "giveaway": get_var("GIVEAWAY_4"),
        },
        doc! {
           "question": "Who is the best at being offended?",
            "level": 4,
            "answer": get_hashed_var("ANSWER_5"),
            "hint": get_var("HINT_5"),
            "giveaway": get_var("GIVEAWAY_5"),
        },
    ];

    // now insert each document from vector into the collection
    for doc in data {
        collection.insert_one(doc, None).await?;
    }

    Ok(())
}
