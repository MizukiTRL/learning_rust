#![allow(unused)]

use std::fmt::{format, Display};

pub struct NewsArticle{
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn sumarize(&self) -> String{
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn sumarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary{
    fn sumarize(&self) -> String{
        String::from("(read more)")
    }
}

pub fn notify<T: Summary>(item: &T){
    println!("breaking news: {}", item.sumarize());
}

fn main() {
    let tweet = Tweet{
        username: format!("pedro"),
        content: String::from("el perro perron perrea"),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle{
        author: format!("pedrito pedrares"),
        headline: format!("pedro tweetea algo insolente"),
        content: format!("bla bla bla bla bla ofensivo bla bla bla"),
    };

    println!("tweet summary: {}", tweet.sumarize());
    println!("news summary: {}", news_article.sumarize());
    
    notify(&tweet);
}
