use actix_web :: {
    get, post, web, App, HttpResponse, HttpServer, Responder, Result
};

mod structs;
use structs::{TwitsModel, UserModel, StatusAns};

mod new_handler;





#[get("/check_user")]
async fn hello(data: web::Json<UserModel>) -> String {
    print!("Check_user");
    let login: String = data.login.clone();
    let password = data.password.clone();
    if (login.eq("admin") && password.eq("admin")) {
        let ans = StatusAns {status:200};
        let response = serde_json::to_string(&ans).unwrap();

        response
        //HttpResponse::Ok().body("Hello world")
    } else {
        let ans = StatusAns {status:400};
        let response = serde_json::to_string(&ans).unwrap();
        response
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
