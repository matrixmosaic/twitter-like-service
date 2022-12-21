
//mod crate::types;
use std::collections::HashMap;
use crate::models::models::{User, Tweet, Follow, Comment};
use uuid::Uuid;
use crate::types;
use crate::{types::result::ResultType};
use types::end_points::Endpoint;
use chrono::{Utc, NaiveDateTime};


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

       // This match arm handles the "GetTweets" variant of the "Endpoint" enum.
       // The `uid` parameter represents the user ID of the user whose tweets we want to retrieve.
        Endpoint::GetTweets(uid) => {
            // First, we retrieve all tweets in the `self.tweets` map.
            // Then, we use the `filter` method to keep only those tweets that were made by the user with the specified `uid`.
            // The `cloned` method creates a new collection of the same type as the original, but with all elements cloned.
            // Finally, we use the `collect` method to turn the filtered, cloned tweet collection into a new collection.
            let tweets = self.tweets
                .values()
                .filter(|tweet| tweet.user_id == uid)
                .cloned()
                .collect();

            // We return the collected tweets wrapped in the "Tweets" variant of the "ResultType" enum.
            Ok(ResultType::Tweets(tweets))
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

          // This is a variant of the `Endpoint` enum, representing a request to create a new user.
        Endpoint::CreateUser(user) => {
           // Generate a new random UUID.
           let uuid = Uuid::new_v4();
           // Clone the `user` struct to create a new mutable `new_user` struct.
           let mut new_user = user.clone();
           // Convert the UUID to a string.
           let uuid_str = uuid.to_string();
           // Set the `uid` field of the `new_user` struct to the generated UUID.
           new_user.uid = uuid_str.clone();

          // Insert the `new_user` struct into the map of users using the generated UUID as the key.
           self.users.insert(uuid_str, new_user.clone());
          // Return a `ResultType::Success` value to indicate that the request was successful.
          Ok(ResultType::Success)
}




        // This is a variant of the `Endpoint` enum, representing a request to create a new tweet.
         Endpoint::CreateTweet { user_id, body } => {
            // Generate a new random UUID.
            let uuid = Uuid::new_v4();
            // Convert the UUID to a string.
            let uuid_string = uuid.to_string();

            // Get the current date and time in the UTC time zone.
             let now = Utc::now();
            // Convert the date and time to a `NaiveDateTime` object.
             let naive_now = now.naive_utc();

            // Create a new `Tweet` struct using the provided `user_id` and `body` values,
            // the generated UUID, and the current date and time and insert into the map.
            self.tweets.insert(uuid_string.clone(), Tweet {
            tweet_id: uuid_string,
            user_id,
            body,
            created_at: naive_now,
    });
    // Return a `ResultType::Success` value to indicate that the request was successful.
    Ok(ResultType::Success)
},


          // This is a variant of the `Endpoint` enum, representing a request to follow a user.
     Endpoint::FollowUser { follower_id, followee_id } => {
          // Get the current date and time in the UTC time zone.
          let now = Utc::now();
         // Convert the date and time to a `NaiveDateTime` object.
          let naive_now = now.naive_utc();

          // Create a new `Follow` struct using the provided `follower_id` and `followee_id` values,
          // and the current date and time nd insert into the map.
          self.follows.insert((follower_id.clone(), followee_id.clone()), Follow {
             follower_id,
             followee_id, 
              created_at: naive_now,
    });
    // Return a `ResultType::Success` value to indicate that the request was successful.
    Ok(ResultType::Success)
},


            Endpoint::UnfollowUser { follower_id, followee_id } => todo!(),
            Endpoint::CreateComment { tweet_id, user_id, body } => todo!(),
            Endpoint::DeleteComment(_) => todo!(),
            Endpoint::UpdateComment { id, body } => todo!(),
            Endpoint::DeleteTweet(_) => todo!(),
            Endpoint::UpdateTweet { id, body } => todo!(),
        }
    }
}

