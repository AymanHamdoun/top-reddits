#[macro_use]
extern crate colour;

use crate::reddit::{RedditPost, SubRedditPostLoader};
mod reddit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let subreddit_name: &str = args[1].as_str();
    let top_count: u32 = args[1].parse().unwrap_or(2);

    green!("Getting top {} posts from r/{}\n", top_count, subreddit_name);


    let mut subreddit_post_loader = SubRedditPostLoader::from_subreddit(subreddit_name);
    subreddit_post_loader.load_top(top_count);

    print_to_terminal(subreddit_post_loader.get())
}

fn print_to_terminal(posts: Vec<RedditPost>) {
    for post in posts {
        cyan!("{}\n", post.title);
        if !post.selftext.is_empty() {
            grey!("{}\n", post.selftext);
        }
        yellow!("{}\n\n", post.url)
    }
}