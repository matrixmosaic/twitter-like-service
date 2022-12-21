
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
        match endpoint {

          

            // Returns the tweet with the given ID, if it exists.
            Endpoint::GetTweet(tid) => {
                if let Some(tweet) = self.tweets.get(&tid) {
                    Ok(ResultType::Tweet(tweet.clone()))
                } else {
                    Err(format!("Tweet with ID {} not found", tid))
                }
            },

            // Returns all follows that involve the given user ID.
            Endpoint::GetFollows(uid) => {
                let follows = self.follows
                    .values()
                    .filter(|follow| follow.follower_id == uid || follow.followee_id == uid)
                    .cloned()
                    .collect();
                Ok(ResultType::Follows(follows))
            },

            // Returns all comments on the tweet with the given ID.
            Endpoint::GetComments(tid) => {
                let comments = self.comments
                    .values()
                    .filter(|comment| comment.tweet_id == tid)
                    .cloned()
                    .collect();
                Ok(ResultType::Comments(comments))
            },
               // Returns the user with the given ID, if it exists.
               Endpoint::GetUser(uid) => {
                if let Some(user) = self.users.get(&uid) {
                    Ok(ResultType::User(user.clone()))
                } else {
                    Err(format!("User with ID {} not found", uid))
                }
            },

            // Creates a new user.
            Endpoint::CreateUser(user) => {
                self.users.insert(user.clone().uid, user.clone());
                Ok(ResultType::Success)
            }
            Endpoint::CreateTweet { user_id, body } => todo!(),
            Endpoint::FollowUser { follower_id, followee_id } => todo!(),
            Endpoint::UnfollowUser { follower_id, followee_id } => todo!(),
            Endpoint::CreateComment { tweet_id, user_id, body } => todo!(),
            Endpoint::DeleteComment(_) => todo!(),
            Endpoint::UpdateComment { id, body } => todo!(),
            Endpoint::DeleteTweet(_) => todo!(),
            Endpoint::UpdateTweet { id, body } => todo!(),
        }
    }
}

