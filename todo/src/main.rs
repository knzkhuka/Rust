use actix_web::HttpResponse;


#[get("/")]
async fn index() -> Result<HttpResponse,actix_web::Error>{
    let response_body = "Hlloe World";
    Ok(HttpResponse::Ok().body(response_body))
}


fn main() {
    
}
