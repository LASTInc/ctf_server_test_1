use actix_web :: {
    get, post, web, App, HttpResponse, HttpServer, Responder, Result
};

use std::{env, vec};



mod structs;
use mongodb::bson::{doc, Document};
use structs::{TwitsModel, UserModel, StatusAns, MyResponse};


mod db;
use db::MongoClient;

mod new_handler;





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




async fn gen_users() {
    let mongo_client = MongoClient::new("mongodb://localhost:27017".to_string()).await;
    let users_collection = mongo_client.create_new_collection("Users").await;
    let admin_twits = mongo_client.create_new_collection("Admins_twits").await;
    let temps_twits = mongo_client.create_new_collection("Temps_twits").await;
    let users = vec![
        doc! {"login":"12345", "password":"34234", "dir_data":"Temps_twits"},
        doc! {"login":"sdfsdf", "password":"342dfgdfgdfg34", "dir_data":"Admins_twits"},
        ];
    mongo_client.add_document_to_collection("Users", users).await;
}



async fn add_tweets() {
    let mongo_client = MongoClient::new("mongodb://localhost:27017".to_string()).await;
    let vec_twits: Vec<TwitsModel> = vec ![
        TwitsModel{name: 
            "Марков Лаврентий".to_string(),
            image: "https://avatars.mds.yandex.net/get-entity_search/1554108/979273824/S600xU_2x".to_string(),
            someText: "This Quick Start post will help you connect your Rust application to a MongoDB cluster. It will then show you how to do Create, Read, Update, and Delete (CRUD) operations on a collection. ".to_string()
        },
        TwitsModel{name: 
            "Марков Лаврентий".to_string(),
            image: "https://avatars.mds.yandex.net/get-entity_search/1554108/979273824/S600xU_2x".to_string(),
            someText: "This series assumes that you have a recent version of the Rust toolchain installed (v1.57+), and that you're comfortable with Rust syntax. It also assumes that you're reasonably comfortable using the command-line and your favourite code editor.
".to_string()
        },
        TwitsModel{name: 
            "Марков Лаврентий".to_string(),
            image: "https://avatars.mds.yandex.net/get-entity_search/1554108/979273824/S600xU_2x".to_string(),
            someText: "I'm going to assume you have a working knowledge of Rust. I won't use any complex Rust code - this is a MongoDB tutorial, not a Rust tutorial - but you'll want to know the basics of error-handling and borrowing in Rust, at least! You may want to run rustup update if you haven't since January 2022 because I'll be working with a recent release.".to_string()
        },
    ];

    let mut all_doc: Vec<Document> = vec! [];
    
    for item in vec_twits.iter() {
        let get_doc = doc! {
            "name":item.name.to_string(),
            "image":item.image.to_string(),
            "someText":item.someText.to_string()
        };

        all_doc.push(get_doc);
    }

    mongo_client.add_document_to_collection("Temps_twits", all_doc).await;

}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.contains(&"add_users".to_string()) {
        gen_users().await;
    }
    if args.contains(&"add_data".to_string()) {
        add_tweets().await;
    }
    HttpServer::new(|| {
        App::new()
        .service(hello)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}






