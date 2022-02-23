use serde_json::{Value, Result, json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditPost {
    pub title: String,
    pub url: String,
    pub selftext: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubRedditPostLoader {
    subreddit_name: String,
    subreddit_url: String,
    posts: Vec<RedditPost>
}

impl SubRedditPostLoader {
    pub fn from_subreddit(subreddit_name: &str) -> SubRedditPostLoader {
        let name = subreddit_name.to_string();
        SubRedditPostLoader {
            subreddit_name: name.clone(),
            subreddit_url: "https://www.reddit.com/r/".to_owned() + name.as_str(),
            posts: Vec::new()
        }
    }

    pub fn get(&self) -> Vec<RedditPost> {
        self.posts.clone()
    }

    pub fn load_top(&mut self, count: u32) {
        let api_url = (self.subreddit_url.clone() + "/top/.json?limit=" + count.to_string().as_str());

        let response = ureq::get(api_url).call().into_string();
        let response_string = match response {
            Ok(r) => r,
            Err(e) => "".to_string()
        };

        if response_string.is_empty() {
            panic!("Sorry, I seem to have crashed !");
        }

        let v: Value = serde_json::from_str(response_string.as_str()).unwrap();
        let post_v = &v["data"]["children"].as_array().unwrap().to_vec();

        for (i, p) in post_v.iter().enumerate() {
            self.posts.push(RedditPost {
                title: p.get("data").unwrap().get("title").unwrap().to_string(),
                selftext: p.get("data").unwrap().get("selftext").unwrap().to_string(),
                url: p.get("data").unwrap().get("url").unwrap().to_string()
            });
        }
    }
}

