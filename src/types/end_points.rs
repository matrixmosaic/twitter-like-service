use crate::models::models::User;



// Endpoint is an enum that represents the different endpoints or API functions that the Twitter-like service supports.
// Each variant of the enum corresponds to a different endpoint and may contain additional data fields.
pub enum Endpoint {
    
    //This endpoint returns the user with the specified uid if it exists in the users hash map.
    // It takes a single String field representing the user ID.
    GetUser(String),

    // The GetTweet variant represents the endpoint for retrieving a single tweet by ID.
    // It takes a single String field representing the tweet ID.
    GetTweet(String),

    GetTweets(String),
    // The GetFollows variant represents the endpoint for retrieving a list of users that a given user is following.
    // It takes a single String field representing the user ID.
    GetFollows(String),

    // The GetComments variant represents the endpoint for retrieving a list of comments for a given tweet.
    // It takes a single String field representing the tweet ID.
    GetComments(String),

    // The CreateTweet variant represents the endpoint for creating a new tweet.
    // It takes two fields: a String field representing the user ID of the tweet creator and a String field representing the tweet content.
    CreateTweet { user_id: String, body: String },

    // The FollowUser variant represents the endpoint for following a user.
    // It takes two String fields representing the IDs of the follower and the followee.
    FollowUser { follower_id: String, followee_id: String },

    // The UnfollowUser variant represents the endpoint for unfollowing a user.
    // It takes two String fields representing the IDs of the follower and the followee.
    UnfollowUser { follower_id: String, followee_id: String },

    // The CreateComment variant represents the endpoint for creating a new comment.
    // It takes three fields: a String field representing the tweet ID, a String field representing the user ID of the comment creator,
    // and a String field representing the comment content.
    CreateComment { tweet_id: String, user_id: String, body: String },

    DeleteComment(String),
    UpdateComment {
        id: String,
       body: String,
    },

    DeleteTweet(String),
    UpdateTweet {
        id: String,
       body: String,
    },
    CreateUser(String),

}