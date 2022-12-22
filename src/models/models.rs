/* 
This file contains the struct for the Twitter like Api Models. The #[derive(...)] attribute on each model automatically implements certain traits for the struct, including Debug, Clone, PartialEq, Eq, Serialize, and Deserialize, 
which allows the struct to be printed, cloned, compared for equality, serialized (converted to a byte representation), and deserialized (converted from a byte representation) as needed.
*/
use chrono::{NaiveDateTime};
use serde::{Serialize,Deserialize};

// This struct represents a User in the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    // unique ID for the user
    pub uid: String, 
    // email address of the user
    //pub email :String, 
    // first name of the user
    //pub first_name :String, 
    // last name of the user
    //pub last_name :String, 
    // second name of the user 
    //pub second_name :String, 
    // username of the user
    //pub user_name :String, 
    // password of the user (hashed and salted for security)
   // pub password :String, 
    // phone number of the user
   // pub phone :String, 
    // gender of the user
   // pub gender :String 
   }


// This struct represents a Tweet in the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tweet {
    // a unique identifier for the tweet, represented as a String
     pub tweet_id: Option<String>,
    // the user's ID for the user who made the tweet
    pub user_id: String,
    // the tweet's body text
    pub body: String,
    // the date and time the tweet was created, represented as a NaiveDateTime
    pub created_at: Option<NaiveDateTime>,
}

//The Follow struct represents a following relationship between two users in a Twitter-like API.  
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Follow {
    // ID of the user who is following
    pub follower_id: String, 
    // ID of the user being followed
    pub followee_id: String, 
    // timestamp for when the follow relationship was created
    pub created_at: Option<NaiveDateTime>, 
}

// A struct representing a comment in a Twitter-like API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    // unique ID for the comment
    pub comment_id: String,
    // the contents of the comment 
    pub  body: String, 
    // timestamp for when the comment was created
    pub  created_at: NaiveDateTime, 
    // ID of the tweet that the comment belongs to
    pub tweet_id: String, 
    // ID of the user who made the comment
    pub user_id: String, 
}


