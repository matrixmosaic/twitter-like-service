# twitter-like-service
A Twitter-like service is a web-based application that allows users to create and share short messages, similar to the social media platform Twitter. It is implemented using Rust and the Actix-web framework.  Users of the service can post tweets, and follow other users to see their tweets in their feeds. 

## Introduction

The Twitter-like service is a web-based mvp application that allows users to create and share short messages, similar to the social media platform Twitter. It is implemented using Rust and the Actix-web framework. Users of the service can post tweets, and follow other users to see their tweets in their feeds.


## Overview

The Twitter-like application system architecture exposes a set of APIs(RESTful-like JSON API endpoints) that allow developers to access the functionality and data of the service. The APIs are designed to be easy to use and allow to Create an user, Create a tweet, Get a user tweet feed, Follow and Unfollow another user.

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


