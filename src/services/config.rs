pub mod service_config{
    use actix_web::web::ServiceConfig;
    use actix_web::{web};
    use crate::services::handler::*;

    // configure is a function that registers the routes and handlers for the Twitter-like service.

        pub fn configure(cfg: &mut ServiceConfig) {

        // This route registers the get_user and create_user handlers for the "/users/{uid}" resource.
        // The get_user handler will handle GET requests and the create_user handler will handle POST requests.

     
        cfg.service(
            web::resource("/users/{uid}")
                .route(web::get().to(get_user))
                .route(web::post().to(create_user)),
        );

        // This route registers the get_tweet handler for the "/tweets/{tid}" resource.
        // It will handle GET requests.
        cfg.service(web::resource("/tweets/{tid}").route(web::get().to(get_tweet)));

        // This route registers the get_follows, follow_user, and unfollow_user handlers for the "/follows/{uid}" resource.
        // The get_follows handler will handle GET requests, the follow_user handler will handle POST requests, and the unfollow_user handler will handle DELETE requests.
        cfg.service(
            web::resource("/follows/{uid}")
                .route(web::get().to(get_follows))
                .route(web::post().to(follow_user))
                .route(web::delete().to(unfollow_user)),
        );

        // Register the get_comments handler for the "/comments/{tid}" resource.
        // It will handle GET requests.
        cfg.service(web::resource("/comments/{tid}").route(web::get().to(get_comments)));

        // Register the create_tweet, update_tweet, and delete_tweet handlers for the "/tweets" resource.
        // The create_tweet handler will handle POST requests, the update_tweet handler will handle PUT requests, and the delete_tweet handler will handle DELETE requests.
        cfg.service(
            web::resource("/user/{uid}/tweet")
                .route(web::post().to(create_tweet))
                .route(web::put().to(update_tweet))
                .route(web::delete().to(delete_tweet)),
        );

        cfg.service(
            web::resource("/user/:uid/feed")
                .route(web::get().to(get_feed)),
               
        );

        // Register the create_comment, update_comment, and delete_comment handlers for the "/comments" resource.
        // The create_comment handler will handle POST requests, the update_comment handler will handle PUT requests, and the delete_comment handler will handle DELETE requests.
        cfg.service(
            web::resource("/comments")
                .route(web::post().to(create_comment))
                .route(web::put().to(update_comment))
                .route(web::delete().to(delete_comment)),
        );
    }
}