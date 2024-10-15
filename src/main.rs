use actix_web :: {
    get, post, web, App, HttpResponse, HttpServer, Responder, Result
};

mod structs;
use structs::{TwitsModel, UserModel, StatusAns, MyResponse};


mod db;
use db::MongoClient;

mod new_handler;
use new_handler::{get_data_for_auth_user, };





#[get("/check_user")]
async fn hello(data: web::Json<UserModel>) -> HttpResponse {
    let mongo_client = MongoClient::new("mongodb://localhost:27017".to_string()).await;
    let ans = mongo_client.get_data_for_user(&data).await;
    if let Some(ans) = ans {
        let status = StatusAns {status: 200};
        let response: MyResponse<Vec<TwitsModel>> = MyResponse::new(ans, status);
        let temp_ans = serde_json::to_string(&response).unwrap();
        HttpResponse::Ok().body(temp_ans)
    } else {
        let status = StatusAns {status: 404};
        let ans = serde_json::to_string(&status).unwrap();
        HttpResponse::NotFound().body(ans)
    }
    
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(hello)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}
