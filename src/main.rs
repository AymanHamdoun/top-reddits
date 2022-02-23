#[macro_use]
extern crate colour;

use crate::reddit::{RedditPost, SubRedditPostLoader};
mod reddit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        red!("{}\n", "Please prove all the necessary arguments");
    }

    let subreddit_name: &str = args[1].as_str();

    let mut top_count: u32 = 3;
    if args.len() >= 3 {
        top_count = args[2].parse::<u32>().unwrap();
    }

    let mut filter: String = String::from(":top");
    if args.len() >= 4 {
        filter = args[3].to_string();
    }

    green!("Getting {} {} posts from r/{}\n", top_count, filter, subreddit_name);


    let mut subreddit_post_loader = SubRedditPostLoader::from_subreddit(subreddit_name);

    match filter.as_str() {
        ":top" => subreddit_post_loader.load_top(top_count),
        ":new" => subreddit_post_loader.load_new(top_count),
        _ => {}
    }

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