use serde_json::{Value, Result, json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    title: String,
    url: String,
    selftext: String
}

fn main() {
    println!("Application Starting ... ");

    let response = ureq::get("https://www.reddit.com/r/lebanon/top/.json?count=20").call().into_string();
    let response_string = match response {
        Ok(r) => r,
        Err(e) => "".to_string()
    };

    if response_string.is_empty() {
        panic!("Sorry, I seem to have crashed !");
    }

    let v: Value = serde_json::from_str(response_string.as_str()).unwrap();

    let mut posts: Vec<Post> = vec![];
    let post_v = &v["data"]["children"].as_array().unwrap().to_vec();

    for (i, p) in post_v.iter().enumerate() {
        posts.push(Post {
            title: p.get("data").unwrap().get("title").unwrap().to_string(),
            selftext: p.get("data").unwrap().get("selftext").unwrap().to_string(),
            url: p.get("data").unwrap().get("url").unwrap().to_string()
        });
    }

    dbg!(posts);

}
