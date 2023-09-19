use actix_web::HttpResponse;

use crate::constants::APPLICATION_JSON;

/// list 50 last tweets `/tweets`
#[get("/items")]
pub async fn list() -> HttpResponse {
    // TODO find the last 50 tweets and return them

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json("")
}

#[get("/diesel")]
pub async fn diesel() -> HttpResponse {
    // TODO find the last 50 tweets and return them

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json("")
}

///// create a tweet `/tweets`
//#[post("/tweets")]
//pub async fn create(tweet_req: Json<TweetRequest>) -> HttpResponse {
//    HttpResponse::Created()
//        .content_type(APPLICATION_JSON)
//        .json(tweet_req.to_tweet())
//}

///// find a tweet by its id `/tweets/{id}`
//#[get("/tweets/{id}")]
//pub async fn get(path: Path<(String,)>) -> HttpResponse {
//    // TODO find tweet a tweet by ID and return it
//    let found_tweet: Option<Tweet> = None;
//
//    match found_tweet {
//        Some(tweet) => HttpResponse::Ok()
//            .content_type(APPLICATION_JSON)
//            .json(tweet),
//        None => HttpResponse::NoContent()
//            .content_type(APPLICATION_JSON)
//            .await
//            .unwrap(),
//    }
//}