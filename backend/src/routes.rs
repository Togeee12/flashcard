use actix_web::{web, HttpResponse};

use serde_derive::{Serialize, Deserialize};
use crate::{models, db};

pub fn api(cfg: &mut web::ServiceConfig) {
    // Flashcard conf for routes (Please make it idk how xDDD)
    cfg
        .route("", web::get().to(get_flashcards))
        .route("/", web::get().to(get_flashcards))
        
        .route("", web::post().to(anwser_flashcard))
        .route("/", web::post().to(anwser_flashcard));



    #[derive(Serialize, Deserialize, Debug)]
    struct FormData {
        a: String,
    }

    let dupa = models::FlashCard::new(
        2137,
        "O której godzinie umarł papaj".to_owned(),
        "21:37".to_owned(),
    );


    
    async fn get_flashcards() -> HttpResponse {
        HttpResponse::Ok().body(db::pseudo_db_call().await.get_question())
    }
    
    async fn anwser_flashcard(body_content: web::Json<FormData>) -> HttpResponse {
        println!("Received form data: {:?}", body_content.a);
        HttpResponse::Ok().body("Form POST request received")
    }
}
