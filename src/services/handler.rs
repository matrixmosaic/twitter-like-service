use std::sync::{Arc, Mutex};

//the handler functions for each endpoint. These functions will use the TwitterLikeAPI struct to handle the request and return a response.
use actix_web::{  web,  HttpResponse};
use crate::{services::api::TwitterLikeAPI, types::{end_points::Endpoint, result::ResultType}, models::models::{Follow, Tweet, Comment}};


pub struct TwitterLikeAPIHandler {
    pub api: Arc<Mutex<TwitterLikeAPI>>,

}




// get_tweet is an async function that handles a request to retrieve a tweet by its ID.
// It takes in two arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the tweet to be retrieved
// The function returns a HttpResponse object indicating the result of the request.
pub  async fn get_tweet(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
) -> HttpResponse {
    // The first step is to extract the tweet ID from the path object.
    let tid = path.into_inner();

    // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::GetTweet variant
// with the tweet ID as its argument. This will trigger the appropriate handler function in the TwitterLikeAPI
// struct to retrieve the requested tweet.
    let mut api = api_handler.api.lock().unwrap();

    match api.handle_endpoint(Endpoint::GetTweet(tid)) {

    // If the call is successful and returns a ResultType::Tweet variant, we return an HTTP 200 OK response
    // with the tweet data serialized as the response body in JSON format
        Ok(ResultType::Tweet(tweet)) => HttpResponse::Ok().json(tweet),

    // If the call is successful but returns a ResultType::Success variant, we return an HTTP 200 OK response
    // with no response body.
        Ok(_) => HttpResponse::Ok().into(),

    // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
    // as the response body.
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}





// get_comments is an async function that handles a request to retrieve the comments for a tweet by its ID.
// It takes in two arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the tweet to retrieve comments for
// The function returns a HttpResponse object indicating the result of the request.
    pub async fn get_comments(
        api_handler: web::Data<TwitterLikeAPIHandler>,
        path: web::Path<String>,
    ) -> HttpResponse {

        // The first step is to extract the tweet ID from the path object.
        let tid = path.into_inner();

       // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::GetComments variant
       // with the tweet ID as its argument. This will trigger the appropriate handler function in the TwitterLikeAPI
       // struct to retrieve the comments for the requested tweet.
       let mut api = api_handler.api.lock().unwrap();

        match api.handle_endpoint(Endpoint::GetComments(tid)) {
            
           // Return HTTP 200 OK response with  the comments data serialized as the response body in JSON format.
            Ok(ResultType::Comments(comments)) => HttpResponse::Ok().json(comments),
                    
            //return an HTTP 200 OK response with no response body.
            Ok(_) => HttpResponse::Ok().into(),
            
            //eturn an HTTP 500 Internal Server Error response with the error message as the response body.
            Err(error) => HttpResponse::InternalServerError().body(error),
        }
    }
   


    
    // get_user is an async function that handles a request to retrieve a user by their ID.
    // It takes in two arguments:
    // - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
    // - path: a web::Path object containing the String of the user to be retrieved
    // The function returns a HttpResponse object indicating the result of the request.
    pub async fn get_user(
        api_handler: web::Data<TwitterLikeAPIHandler>,
        path: web::Path<String>,
    ) -> HttpResponse {
        let uid = path.into_inner();
        let mut api = api_handler.api.lock().unwrap();

          match api.handle_endpoint(Endpoint::GetUser(uid)) {
            Ok(ResultType::User(user)) => HttpResponse::Ok().json(user),
            Ok(_) => HttpResponse::Ok().into(),
            Err(error) => HttpResponse::InternalServerError().body(error),
        }
    }
    
   
    // Defines an asynchronous function for creating a new user
pub async fn create_user(
    // Takes in an instance of the TwitterLikeAPI as a data object, accessed through the web crate
    api_handler: web::Data<TwitterLikeAPIHandler>,
    // Takes in a JSON object representing a User struct, accessed through the web crate
    path: web::Path<String>,
) -> HttpResponse {

      // The first step is to extract the user ID from the path object.
      let uid = path.into_inner();
    // Match statement to handle the result of calling the "handle_endpoint" function with the endpoint
    // "Endpoint::CreateUser" and the user data as arguments
    let mut api = api_handler.api.lock().unwrap();

      match api.handle_endpoint(Endpoint::CreateUser(uid)) {
        // If the result is Ok(ResultType::Success), return an HTTP OK response
        Ok(_) => HttpResponse::Ok().into(),
        // If the result is an Err variant, return an HTTP Internal Server Error response with the error message as the body
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}

    



pub async fn get_follows(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
) -> HttpResponse {

    // The first step is to extract the user ID from the path object.
    let uid = path.into_inner();

    let mut api = api_handler.api.lock().unwrap();

      match api.handle_endpoint(Endpoint::GetFollows(uid)) {
    // If the call is successful and returns a ResultType::User variant, we return an HTTP 200 OK response
    // with the user data serialized as the response body in JSON format.
    Ok(ResultType::User(user)) => HttpResponse::Ok().json(user),

    // If the call is successful but returns a ResultType::Success variant, we return an HTTP 200 OK response
    // with no response body.
    Ok(_) => HttpResponse::Ok().into(),

    // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
    // as the response body.
    Err(error) => HttpResponse::InternalServerError().body(error),
    }
}



//This function receives two arguments: api_handler, which is a web::Data object containing a TwitterLikeAPIHandler object, 
//and path, which is a web::Path object containing a tuple of a single String value representing the user ID
//The function uses a match expression to handle each possible variant of the ResultType enum.
// If the call is successful and returns a  ResultType::Tweets,  or `Result

pub async fn get_feed(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
) -> HttpResponse {
    // The first step is to extract the user ID from the path object.
    let uid = path.into_inner();

    // Lock the api for exclusive access and assign it to the mutable variable `api`.
    let mut api = api_handler.api.lock().unwrap();

    // Call the `handle_endpoint` method on the `api` object, passing it the `Endpoint::GetTweet` variant
    // with the user ID as an argument.
    match api.handle_endpoint(Endpoint::GetTweets(uid)) {
    
        // If the call is successful and returns a ResultType::Tweets variant, we return an HTTP 200 OK response
        // with the tweet data serialized as the response body in JSON format.
        Ok(ResultType::Tweets(tweets)) => HttpResponse::Ok().json(tweets),
        
        
        Ok(_)=> HttpResponse::Ok().into(),
         // If the call is unsuccessful and returns a ResultType::Error variant, we return an HTTP 500 Internal Server Error response.
         Err(error) => HttpResponse::InternalServerError().body(error),
        }
}


// follow_user is an async function that handles a request to follow another user.
// It takes in three arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the current user making the follow request
// - follow: a web::Json object containing the follow request data, including the String of the user to be followed
// The function returns a HttpResponse object indicating the result of the request.
pub async fn follow_user(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
    follow: web::Json<Follow>,
) -> HttpResponse {

    // The first step is to extract the current user's ID from the path object and the follow request data from the follow object.
    let uid = path.into_inner();
    let follow = follow.into_inner();
 
   // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::FollowUser variant
   // with the current user's ID and the user to be followed's ID as its arguments. This will trigger the appropriate
   // handler function in the TwitterLikeAPI struct to update the current user's list of followed users.

   let mut api = api_handler.api.lock().unwrap();

       match api.handle_endpoint(Endpoint::FollowUser {
    follower_id: uid,
    followee_id: follow.followee_id,
    }) {
    // If the call is successful, we return an HTTP 200 OK response with no response body.
    Ok(_) => HttpResponse::Ok().into(),

    // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
    // as the response body.
    Err(error) => HttpResponse::InternalServerError().body(error),
   }
}



// unfollow_user is an async function that handles a request to unfollow another user.
// It takes in three arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the current user making the unfollow request
// - follow: a web::Json object containing the unfollow request data, including the String of the user to be unfollowed
// The function returns a HttpResponse object indicating the result of the request.
pub async fn unfollow_user(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
    follow: web::Json<Follow>,
) -> HttpResponse {

    // The first step is to extract the current user's ID from the path object and the unfollow request data from the follow object.
    let uid = path.into_inner();
    let follow = follow.into_inner();
  
   // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::UnfollowUser variant
   // with the current user's ID and the user to be unfollowed's ID as its arguments. This will trigger the appropriate
   // handler function in the TwitterLikeAPI struct to update the current user's list of followed users.

   let mut api = api_handler.api.lock().unwrap();

     match api.handle_endpoint(Endpoint::UnfollowUser {
      follower_id: uid,
      followee_id: follow.followee_id,
   }) {
    // If the call is successful, we return an HTTP 200 OK response with no response body.
    Ok(_) => HttpResponse::Ok().into(),

    // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
    // as the response body.
    Err(error) => HttpResponse::InternalServerError().body(error),
   }
}



// create_tweet is an async function that handles a request to create a new tweet.
// It takes in three arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the current user making the tweet creation request
// - tweet: a web::Json object containing the tweet data, including the tweet content
// The function returns a HttpResponse object indicating the result of the request.

pub async fn create_tweet(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
    tweet: web::Json<Tweet>,
) -> HttpResponse {

    // The first step is to extract the current user's ID from the path object and the tweet data from the tweet object.
    let uid = path.into_inner();
    let tweet = tweet.into_inner();
 
   // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::CreateTweet variant
   // with the current user's ID and the tweet content as its arguments. This will trigger the appropriate
   // handler function in the TwitterLikeAPI struct to create a new tweet.

   let mut api = api_handler.api.lock().unwrap();

     match api.handle_endpoint(Endpoint::CreateTweet {
    user_id: uid,
    body: tweet.body,
    }) {
    // If the call is successful, we return an HTTP 200 OK response with no response body.
    Ok(_) => HttpResponse::Ok().into(),

    // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
    // as the response body.
    Err(error) => HttpResponse::InternalServerError().body(error),
    }
}



// update_tweet is an async function that handles a request to update an existing tweet.
// It takes in three arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the tweet to be updated
// - tweet: a web::Json object containing the updated tweet data, including the new tweet content
// The function returns a HttpResponse object indicating the result of the request.

pub async fn update_tweet(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
    tweet: web::Json<Tweet>,
) -> HttpResponse {
   // The first step is to extract the tweet ID from the path object and the updated tweet data from the tweet object.
   let tid = path.into_inner();
   let tweet = tweet.into_inner();

    // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::UpdateTweet variant
    // with the tweet ID and the updated tweet content as its arguments. This will trigger the appropriate
    // handler function in the TwitterLikeAPI struct to update the tweet.

    let mut api = api_handler.api.lock().unwrap();

      match api.handle_endpoint(Endpoint::UpdateTweet {
        id: tid,
       body: tweet.body,
    }) {
      // If the call is successful, we return an HTTP 200 OK response with no response body.
      Ok(_) => HttpResponse::Ok().into(),

        // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
        // as the response body.
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}



// delete_tweet is an async function that handles a request to delete an existing tweet.
// It takes in two arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the tweet to be deleted
// The function returns a HttpResponse object indicating the result of the request.
pub async fn delete_tweet(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
) -> HttpResponse {
    // The first step is to extract the tweet ID from the path object.
     let tid = path.into_inner();
// Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::DeleteTweet variant
// with the tweet ID as its argument. This will trigger the appropriate handler function in the TwitterLikeAPI
// struct to delete the tweet.

   let mut api = api_handler.api.lock().unwrap();
  match api.handle_endpoint(Endpoint::DeleteTweet(tid)) {
    // If the call is successful, we return an HTTP 200 OK response with no response body.
    Ok(_) => HttpResponse::Ok().into(),

    // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
    // as the response body.
    Err(error) => HttpResponse::InternalServerError().body(error),
}

}


// create_comment is an async function that handles a request to create a new comment on a tweet.
// It takes in three arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the tweet on which the comment is being made
// - comment: a web::Json object containing the comment data, including the String of the user making the comment and the comment content
// The function returns a HttpResponse object indicating the result of the request.
pub async fn create_comment(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
    comment: web::Json<Comment>,
  ) -> HttpResponse {
   // The first step is to extract the tweet ID from the path object and the comment data from the comment object.
   let tid = path.into_inner();
   let comment = comment.into_inner();

   // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::CreateComment variant
   // with the tweet ID, user ID, and comment content as its arguments. This will trigger the appropriate
   // handler function in the TwitterLikeAPI struct to create a new comment.

   let mut api = api_handler.api.lock().unwrap();
     match api.handle_endpoint(Endpoint::CreateComment {
       tweet_id: tid,
       user_id: comment.user_id,
       body: comment.body,
  }) {
      // If the call is successful, we return an HTTP 200 OK response with no response body.
      Ok(_) => HttpResponse::Ok().into(),

      // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
      // as the response body.
      Err(error) => HttpResponse::InternalServerError().body(error),
}
  }



// update_comment is an async function that handles a request to update an existing comment.
// It takes in three arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the comment to be updated
// - comment: a web::Json object containing the updated comment data, including the new comment content
// The function returns a HttpResponse object indicating the result of the request.
pub async fn update_comment(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
    comment: web::Json<Comment>,
) -> HttpResponse {
    // The first step is to extract the comment ID from the path object and the updated comment data from the comment object.
     let cid = path.into_inner();
     let comment = comment.into_inner();

   // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::UpdateComment variant
   // with the comment ID and the updated comment content as its arguments. This will trigger the appropriate
   // handler function in the TwitterLikeAPI struct to update the comment.

   let mut api = api_handler.api.lock().unwrap();
     match api.handle_endpoint(Endpoint::UpdateComment {
       id: cid,
      body: comment.body,
   }) {
      // If the call is successful, we return an HTTP 200 OK response with no response body.
      Ok(_) => HttpResponse::Ok().into(),

      // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
      // as the response body.
      Err(error) => HttpResponse::InternalServerError().body(error),
}

}


// delete_comment is an async function that handles a request to delete an existing comment.
// It takes in two arguments:
// - api_handler: a shared data object containing an instance of the TwitterLikeAPI struct
// - path: a web::Path object containing the String of the comment to be deleted
// The function returns a HttpResponse object indicating the result of the request.
pub async fn delete_comment(
    api_handler: web::Data<TwitterLikeAPIHandler>,
    path: web::Path<String>,
) -> HttpResponse {
    // The first step is to extract the comment ID from the path object.
    let cid = path.into_inner();

  // Next, we call the handle_endpoint method on the api_handler object, passing in an Endpoint::DeleteComment variant
  // with the comment ID as its argument. This will trigger the appropriate handler function in the TwitterLikeAPI
  // struct to delete the comment.

  let mut api = api_handler.api.lock().unwrap();
     match api.handle_endpoint(Endpoint::DeleteComment(cid)) {
    // If the call is successful, we return an HTTP 200 OK response with no response body.
    Ok(_) => HttpResponse::Ok().into(),

    // If the call returns an error, we return an HTTP 500 Internal Server Error response with the error message
    // as the response body.
    Err(error) => HttpResponse::InternalServerError().body(error),
}

}

