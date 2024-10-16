use std::error::Error;

use actix_web :: {
    get, post, web, App, HttpResponse, HttpServer, Responder, Result
};

use crate::structs::{TwitsModel, UserModel, StatusAns};


enum Status {
    Auth,
    MainAuth,

}
