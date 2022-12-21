use crate::models::models::{User, Tweet, Follow, Comment};

//The ResultType enum represents the possible types of results that can be returned from the TwitterLikeAPI's handle_endpoint method.
// Each variant of the enum represents a different type of data that can be returned from the API.
// The User variant represents a single user object.
// The Tweet variant represents a single tweet object.
// The Follows variant represents a vector of follow objects.
// The Comments variant represents a vector of comment objects.
// The Success variant represents a successful request that does not return any data.
pub enum ResultType {
    User(User),
    Tweet(Tweet),
    Follows(Vec<Follow>),
    Comments(Vec<Comment>),
    Tweets(Vec<Tweet>),
    Success,
}