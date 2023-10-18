use serde::{Serialize, Deserialize};

use diesel;
use diesel::prelude::*;
use diesel::query_dsl::*;
use diesel::deserialize::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::pg::PgConnection;
use super::schema::users;
use super::schema::users::dsl::*;

#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
pub struct UserDTO {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl User {

    //  pub fn read(connection: &PgConnection) -> Vec<User> {
    //     users::table.order(users::id.asc()).load::<User>(connection).unwrap()
    // }

    pub fn teste() -> i32 {
        0
    }

    pub fn create(user: UserDTO, connection: &PgConnection) -> UserDTO {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");
        //users::table.order(users::id.desc()).first(connection).unwrap()
        user
    }

    pub fn update(_id: i32, user: User, connection: &PgConnection) -> bool {
        diesel::update(users::table.find(_id)).set(&user).execute(connection).is_ok()
        //false
    }

    pub fn delete(_id: i32, connection: &PgConnection) -> bool {
        diesel::delete(users::table.find(_id)).execute(connection).is_ok()
    }

    pub fn select(_id: i32, connection: &PgConnection) -> User {
        //diesel::select(users:table.find(_id)).execute(connection).is_ok()

        let user = users::table.find(_id).first::<User>(connection).expect("Error loading user");

        user
    }

    // pub fn findById(_id: i32, connection: &PgConnection) -> User {
    //     users::table.find(_id)
    // }

    pub fn read(connection: &PgConnection) -> Vec<User> {

        // let data = users::table.select(users::id);
        // let datax = users::table.find(1);
        // let data = users.select(name).load::<String>(connection);

        let datay = users.load::<User>(connection).unwrap_or_default();


        // vec![
        //     User {id:(1), name: String::from("a"),age:1, hometown:String::from("a"), identity:String::from("a")},
        //     User {id:(2), name: String::from("b"),age:1, hometown:String::from("b"), identity:String::from("b")},
            
        //     ]

            datay
    }
   
}