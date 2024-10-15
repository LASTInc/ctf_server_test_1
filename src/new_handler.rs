use std::error::Error;

use actix_web :: {
    get, post, web, App, HttpResponse, HttpServer, Responder, Result
};

use crate::structs::{TwitsModel, UserModel, StatusAns};


enum Status {
    Auth,
    MainAuth,

}


async fn check_reg() {

}


#[get("/get_data_for_ayth")]
pub async fn get_data_for_auth_user(data: web::Json<UserModel>) -> Option<String> {
    let vec_twits: Vec<TwitsModel> = vec ![
        TwitsModel{name: 
            "Марков Лаврентий".to_string(),
            image: "https://avatars.mds.yandex.net/get-entity_search/1554108/979273824/S600xU_2x".to_string(),
            someText: "Добрый день всем. Я новый президент США. Моя новая цель: Победа России в СВО и остановка украинсокй власти".to_string()
        },
        TwitsModel{name: 
            "Марков Лаврентий".to_string(),
            image: "https://avatars.mds.yandex.net/get-entity_search/1554108/979273824/S600xU_2x".to_string(),
            someText: "Добрый день всем. Я новый президент США. Моя новая цель: Победа России в СВО и остановка украинсокй власти".to_string()
        },
        TwitsModel{name: 
            "Марков Лаврентий".to_string(),
            image: "https://avatars.mds.yandex.net/get-entity_search/1554108/979273824/S600xU_2x".to_string(),
            someText: "Добрый день всем. Я новый президент США. Моя новая цель: Победа России в СВО и остановка украинсокй власти".to_string()
        },
    ];

    Some(serde_json::to_string(&vec_twits).unwrap())
}