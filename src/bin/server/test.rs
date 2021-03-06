/* good_stv - a good single transferable vote utility.
 * Copyright (C) 2019 good_stv authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use super::poll::Poll;
use super::rocket;

use rocket::http::{ContentType, Status};
use rocket::local::Client;
use serde_json::json;

#[test]
fn create_poll() {
    let req = json!({
        "name": "Test poll.",
        "candidates": ["a", "b", "c"]
    });
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client
        .post("/polls")
        .header(ContentType::JSON)
        .body(req.to_string())
        .dispatch();
    println!("{:#?}", response.headers());
    println!("{:#?}", response.body_string().unwrap());
    assert_eq!(Status::Created, response.status());
}

#[test]
fn create_and_get_poll() {
    let req = json!({
        "name": "Test poll.",
        "candidates": ["a", "b", "c"]
    });
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client
        .post("/polls")
        .header(ContentType::JSON)
        .body(req.to_string())
        .dispatch();
    println!("{:#?}", response.headers());
    assert_eq!(Status::Created, response.status());
    let mut response_two = client
        .get(response.headers().get_one("Location").unwrap())
        .dispatch();
    assert_eq!(Status::Ok, response_two.status());
    let poll: Poll = serde_json::from_str(&response_two.body_string().unwrap()).unwrap();
    assert_eq!("Test poll.", poll.name);
}

#[test]
fn get_nonexistent_poll_should_fail() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.get("/polls/foobar").dispatch();
    assert_eq!(Status::NotFound, response.status());
}
