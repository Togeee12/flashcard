use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Flashcard {
    pub id: i32,
    pub question: String,
    pub answer: String,
}
