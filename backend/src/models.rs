use serde_derive::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct FlashCard {
    id: i32,
    question: String,
    answer: String,
}

impl FlashCard {
    pub fn new(id: i32, question: String, answer: String) -> Self {
        FlashCard{ id, question, answer }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_question(&self) -> String {
        String::from(&self.question)
    }

    pub fn get_answer(&self) -> String {
        String::from(&self.answer)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}