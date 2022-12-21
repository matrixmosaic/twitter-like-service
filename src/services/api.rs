
//mod crate::types;
use std::collections::HashMap;
use crate::models::models::{User, Tweet, Follow, Comment};

use crate::types;
use crate::{types::result::ResultType};
use types::end_points::Endpoint;

// Define a struct to hold the state of the API
pub struct TwitterLikeAPI {
   // A map of user IDs to user objects.
   pub users: HashMap<String, User>,
   // A map of tweet IDs to tweet objects.
   pub tweets: HashMap<String, Tweet>,
    // A map of (follower ID, followee ID) tuple to follow.
    pub follows: HashMap<(String, String), Follow>,
   // A map of comment IDs to comment objects.
   pub comments: HashMap<String, Comment>,
}

// Implement a function to handle each endpoint
impl TwitterLikeAPI {

    // Handles the given endpoint by interacting with the data model and returning the result.
    pub fn handle_endpoint(&mut self, endpoint: Endpoint) -> Result<ResultType, String> {
      
    }
}

