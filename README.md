# twitter-like-service
A Twitter-like service is a web-based application that allows users to create and share short messages, similar to the social media platform Twitter. It is implemented using Rust and the Actix-web framework.  Users of the service can post tweets, and follow other users to see their tweets in their feeds. 

## Introduction

The Twitter-like service is a web-based mvp application that allows users to create and share short messages, similar to the social media platform Twitter. It is implemented using Rust and the Actix-web framework. Users of the service can post tweets, and follow other users to see their tweets in their feeds.


## Overview

The Twitter-like application system architecture exposes a set of APIs(RESTful-like JSON API endpoints) that allow developers to access the functionality and data of the service. The APIs are designed to be easy to use and allow to Create an user, Create a tweet, Get a user tweet feed, Follow and Unfollow another user.

## Documentation

The System Architeture Document for this application can be found at:

[Twitter Like Service System Architeture Document](https://documents-geobude.s3.amazonaws.com/twitter-like-service-architecture-document.pdf) 

## Building And Running The Application

### Building And Running The Application( Without a container ).
0. Clone the application at [https://github.com/matrixmosaic/twitter-like-service.git](https://github.com/matrixmosaic/twitter-like-service.git).

1. First, install Rust by following the instructions on the official
    website: https://www.rust-lang.org/tools/install    
    
2.  Once Rust is  installed, navigate inside the  project folder( default folder is twitter-like-service )  
3.  You can build the project using the following command:
   
     `cargo build`

This will compile the project and create an executable file in the `target/debug` directory.

4.  To run the project, use the following command:

    `cargo run`

This will build the project (if it has not already been built) and then run the executable.

This will also start the applicalion web server and bind it to the host's port 8080. You can then access the application by visiting [http://localhost:8080](http://localhost:8080/) in your web browser.


You can also use the `--release` flag when building or running the project to create an optimized release build.

cargo build --release cargo run --release

  

  ### Building And Running With Docker
  0. Clone the application at [https://github.com/matrixmosaic/twitter-like-service.git](https://github.com/matrixmosaic/twitter-like-service.git)
  1. Make sure the target machine has Docker already installed.
  2. Navigate inside the  project folder( default folder is twitter-like-service )  
  3. To build the Docker image, you can run the following command:

  `docker build -t twitter-like-service .`
  
  4. To start a Docker container based on the image, you can run the following command:
  `docker run -p 8080:8080 twitter-like-service`

  This will start a new container running your Rust application, and bind the container's port 8080 to the host's port 8080. You can then access the application by visiting [http://localhost:8080](http://localhost:8080/) in your web browser.


## Data Model
In our Twitter-like service , for the MVP purposes we have implemented four basic entities.


#### User: 
Each user has a unique account and can create tweets, follow other users, and be followed by other users.

#### Tweet:
 Each tweet is created by a single user and can be commented and retweeted by multiple users.

#### Follow : 
 A user can follow multiple other users, and each user can have multiple followers. This is a many-to-many (M:M) relationship.

#### Comment:
 A tweet can be commented/liked by multiple users, and each user can comment/like multiple tweets. This is also a many-to-many (M:M) relationship.

Generally the entity relationship diagram below shows the design of AN MVP twItter-like-service database (click to open in new tab and expand for clarity).


![MVP Schema](https://architecture-design-diagrams.s3.amazonaws.com/db.PNG) 



## API Endpoint
The APIs allows to Create an user, Create a tweet, Get a user tweet feed, Follow and Unfollow another user. All API requests should be made to the following endpoint:

### Create an user

This API call creates a new user id.

**Request**: POST /user

**Request body**: Nothing, empty body.

**Response code**: HTTP 200

**Response body**:

    { "uid": "user id here" }



### Create a tweet

This API call creates a new tweet. The maximum length of a tweet is 128 UTF-8 characters.

**Request**: POST /user/**:uid**/tweet

**Request parameters**: uid → user id

**Request body**:

    {
    "user_id":"user id here",
    "body": "tweet text here"
    }

**Response code**: HTTP 200.


### Follow a user

This API call registers a user as a follower of another user. Users cannot follow themselves.

**Request**: POST /user/**:uid**/follow/

**Request parameters**: uid → user id of the user performing the operation.

**Request body**:

    {
    "follower_id": "2",
    "followee_id": "1"
    }

**Response code**: HTTP 200

**Response body**: None, empty



### Get a user tweet feed

This API call returns a list of tweets in the user home timeline feed. For the purpose of the MVP this does not have to be paged.

**Request**: GET /user/:**uid**/feed

**Request parameters**: uid → user id of the user performing the operation.

**Response code**: HTTP 200

**Response body**:

      [    
        {    
         "tweet_id": "67eeca7c-f33a-45a7-ac5e-b1e3235f6c97",    
         "user_id": "1",    
         "body": "tweet text here",    
         "created_at": "2022-12-22T12:19:51.500658800"    
        },    
        {    
         "tweet_id": "48b14ac0-e2d2-46ae-bcec-dfd421967ad5",    
         "user_id": "1",    
         "body": "tweet text here",    
         "created_at": "2022-12-22T12:19:45.852020800"   
        },    
        {    
         "tweet_id": "fd96558c-50c4-42e4-907d-c132676949d3",    
         "user_id": "1",    
         "body": "tweet text here",    
         "created_at": "2022-12-22T12:19:36.270353300"    
        }      
    ]


