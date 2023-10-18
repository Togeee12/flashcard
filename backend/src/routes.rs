use actix_web::{web, HttpResponse};

// use serde_derive::{Serialize, Deserialize};
use crate::{db, models};
use diesel::{prelude::*, sql_query};





// TODO
// TODO
// TODO
// TODO





pub fn api(cfg: &mut web::ServiceConfig) {
    // Flashcard conf for routes (Please make it idk how xDDD)
    cfg
        .route("", web::get().to(get_flashcards))
        .route("/", web::get().to(get_flashcards));
        
        // .route("", web::post().to(anwser_flashcard))
        // .route("/", web::post().to(anwser_flashcard));



    
    async fn get_flashcards(pool: web::Data<db::Pool>) -> HttpResponse {
        // let conn = pool.get().expect("Couldn't get DB connection from pool");
        
        // let raw_query = "SELECT * FROM cards;";  // Your raw SQL query

        // let results = sql_query(raw_query)
        //     .load::<models::Card>(&conn)
        //     .expect("Error executing raw SQL query");


        HttpResponse::Ok().body("")
    }
    
    // async fn anwser_flashcard(body_content: web::Json<FormData>) -> HttpResponse {
    //     println!("Received form data: {:?}", body_content.a);
    //     HttpResponse::Ok().body("Form POST request received")
    // }
}



/*
pub async fn get_users(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let conn = pool.get().expect("Couldn't get DB connection from pool");
    let users = web::block(move || {
        use crate::schema::users::dsl::*;
        users.load::<User>(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("Failed to execute DB query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    pool: web::Data<Pool>,
    user: web::Json<NewUser>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("Couldn't get DB connection from pool");
    let inserted_user = web::block(move || {
        diesel::insert_into(crate::schema::users::table)
            .values(&user.into_inner())
            .execute(&conn)
    })
    .await
    .map_err(|e| {
        eprintln!("Failed to execute DB query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(inserted_user))
}
*/