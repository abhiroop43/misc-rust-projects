use std::collections::HashMap;
use std::env;

use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let user = User {
        name: "John Doe".to_string(),
        age: 90,
        email: "john.doe@example.com".to_string(),
    };

    let firebase_url = env::var("FIREBASE_URL").expect("$FIREBASE_URL is not set");
    let auth_key = env::var("AUTH_KEY").expect("$AUTH_KEY is not set");

    let firebase = Firebase::auth(&firebase_url, &auth_key).unwrap();

    let response = set_user(&firebase, &user).await;

    let mut user = get_user(&firebase, &response.name).await;

    println!("{:?}", user);

    let users = get_users(&firebase).await;

    println!("{:?}", users);

    user.email = "john.doe2@example.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;

    println!("{:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User deleted.");
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;

    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// convert a string to a Response
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// convert a string to a User
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
