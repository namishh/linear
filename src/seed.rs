use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::bson::Document;
use pwhash::bcrypt;

fn get_hashed_var(key: &str) -> String {
    dotenv().ok();
    bcrypt::hash(std::env::var(key).expect("No_Key_As_Well")).expect("No_Key")
}

pub async fn seed(mongo: &mongodb::Client) -> Result<(), mongodb::error::Error> {
    let db = mongo.database("linear");
    let collection = db.collection::<Document>("Questions");

    let data: Vec<Document> = vec![
        doc! {
            "question": "What is the meaning of life?",
            "level": 0,
            "answer": get_hashed_var("ANSWER_1"),
        },
        doc! {
           "question": "What is not the meaning of life?",
            "level": 1,
            "answer": get_hashed_var("ANSWER_2"),
        },
        doc! {
           "question": "Who is the best programmer?",
            "level": 2,
            "answer": get_hashed_var("ANSWER_3"),
        },
        doc! {
           "question": "Who is the worst programmer?",
            "level": 3,
            "answer": get_hashed_var("ANSWER_4"),
        },
        doc! {
           "question": "Who is the best at being offended?",
            "level": 4,
            "answer": get_hashed_var("ANSWER_5"),
        },
    ];

    // now insert each document from vector into the collection
    for doc in data {
        collection.insert_one(doc, None).await?;
    }

    Ok(())
}
