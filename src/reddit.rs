use serde_json::{Value, Result, json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedditPost {
    pub title: String,
    pub url: String,
    pub selftext: String,
    pub author: String,
    pub upvotes: i32,
    pub downvotes: i32,
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
        self.load_posts_from_url(api_url, count);
    }

    pub fn load_new(&mut self, count: u32) {
        let api_url = (self.subreddit_url.clone() + "/new/.json?limit=" + count.to_string().as_str());
        self.load_posts_from_url(api_url, count);
    }

    fn load_posts_from_url(&mut self, url: String, count: u32) {
        let response = ureq::get(url).call().into_string();
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
            let mut clean_text= p.get("data").unwrap().get("selftext").unwrap().to_string();
            clean_text = clean_text.replace("\\n", "\n");
            clean_text = clean_text.replace("\\\"", "");

            self.posts.push(RedditPost {
                title: p.get("data").unwrap().get("title").unwrap().to_string(),
                selftext: clean_text,
                url: p.get("data").unwrap().get("url").unwrap().to_string(),
                author: format!("u/{}", p.get("data").unwrap().get("author_fullname").unwrap().to_string()),
                upvotes: p.get("data").unwrap().get("ups").unwrap().to_string().parse::<i32>().unwrap(),
                downvotes: p.get("data").unwrap().get("downs").unwrap().to_string().parse::<i32>().unwrap()
            });
        }
    }
}

