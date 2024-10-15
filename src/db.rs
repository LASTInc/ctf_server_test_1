use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    rc::Rc,
    str::FromStr,
    sync::Arc,
    thread::{self, Thread},
    vec,
};

use mongodb::{
    bson::{doc, oid::ObjectId, raw::Error, Document, Serializer},
    options::FindOptions,
    results::InsertOneResult,
    Client, Collection, Database,
};

use crate::structs::{TwitsModel, UserModel};

pub struct MongoClient {
    uri: String,
    client: Client,
    database: Database,
}

impl MongoClient {
    pub async fn new(new_uri: String) -> Self {
        let client = Client::with_uri_str(&new_uri).await.unwrap();
        let database = client.database("ctf_test_1");
        MongoClient {
            uri: new_uri,
            client: client,
            database: database,
        }
    }

    pub async fn new_connection(&self) -> Self {
        let my_uri = self.uri.clone();
        MongoClient::new(my_uri).await
    }

    pub async fn get_data_for_user(&self, user_data: &UserModel) -> Option<Vec<TwitsModel>> {
        let result: Collection<Document> = self.database.collection("Users");
        let ans = result
            .find_one(doc! {
                "login":&user_data.login,
                "password":&user_data.password
            })
            .await;
        if let Ok(my_doc) = ans {
            if let Some(doc_ans) = my_doc {
                let directory_name = doc_ans.get_str("dir_data").unwrap();
                let my_dir: Collection<Document> = self.database.collection(directory_name);
                let mut all_ans = my_dir.find(doc! {}).await.unwrap();
                let mut ans_for_request: Vec<TwitsModel> = vec![];
                while all_ans.advance().await.unwrap() {
                    let name_twits = all_ans.current().get_str("name").unwrap();
                    let image = all_ans.current().get_str("image").unwrap();
                    let some_text = all_ans.current().get_str("someText").unwrap();
                    ans_for_request.push(TwitsModel::new(name_twits, image, some_text));
                }
                Some(ans_for_request)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Drop for MongoClient {
    fn drop(&mut self) {
        let result = Client::shutdown(self.client.clone());
    }
}
