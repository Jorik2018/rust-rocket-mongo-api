use std::env;
use std::collections::HashMap;
use serde_json::{ Value};

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{RawDocumentBuf,extjson::de::Error, oid::ObjectId, doc}, 
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client,Database, Collection,Cursor},
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
    db:Database
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo {col ,db}
    }


    
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn create(&self, new_user: HashMap<&str, Value>) -> Result<InsertOneResult, Error> {
        let disabled_quiz = self
            .db.collection("DisabledQuiz")
            .insert_one(new_user, None)
            .ok()
            .expect("Error creating DisabledQuiz");
        Ok(disabled_quiz)
    }

    

    pub fn get_list(&self) -> Result<Vec<RawDocumentBuf>, Error> {
        let cursor: Cursor<RawDocumentBuf> = self
            .db.collection("DisabledQuiz")
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
            //let docs: Vec<RawDocumentBuf> = coll
            //.find(None, None).try_collect();
           
           
            /*while cursor.advance().expect("Error getting list of users") {
                let dc=cursor.deserialize_current();
                match dc{
                    Ok(doc)=>{
                        
                        println!("{:?}", doc);
                        //bson::from_slice(doc.as_bytes()).unwrap()
                    },
                    Err(_) => {println!("err");}
                }
                
            }*/
            /*let books: Vec<Book> = docs
                .par_iter()
                .map(|raw| bson::from_slice(raw.as_bytes()).unwrap())
                .collect();*/

                /*cursor.map(|doc|{
                    bson::from_slice(doc.expect("REASON").as_bytes());
                    return HashMap::new();
                }).collect();*/


       // let users =Vec::new(); /**/
        let users = cursor.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

}