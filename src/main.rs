mod entities;
use crate::entities::entity;

use clap::{Arg, App};
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use select::document::Document;
use select::predicate::Name;
use std::env;

fn make_request(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::blocking::get(url)?.text()?;
    Ok(res)
}

fn retrieve_title(url: &str) -> String {
    let html = make_request(&url).expect("No valid URL provided.");

    let document = Document::from(html.as_str());
    let title_node= document
        .find(Name("title"))
        .into_selection()
        .first();
    
    match title_node {
        Some(node) => format!("{}", node.text().trim()),
        None => "(empty title)".to_string()
    }
}

fn post_item(json_item: String, secret_key: String) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let _res = client.post("https://api.notion.com/v1/pages")
        .header(AUTHORIZATION, format!("Bearer {}", secret_key))
        .header(CONTENT_TYPE, "application/json")
        .header("Notion-Version", "2021-08-16")
        .body(json_item)
        .send()?;

    Ok(())
}

fn main() {
    let arguments = App::new("Links to Notion (l2n)")
        .version("0.1")
        .author("Francesco Cauteruccio <f.cauteruccio@gmail.com>")
        .about("Quick add a link to a table within a Notion page.")
        .arg(Arg::with_name("url")
            .help("URL to add")
            .required(true))
        .arg(Arg::with_name("tag")
            .short("t")
            .help("Add a single tag")
            .takes_value(true)
            .multiple(true))
        .get_matches();
    
    let database_id = env::var("NOTION_DATABASE_ID").expect("Please set the NOTION_DATABASE_ID environment variable.");
    let secret_key = env::var("NOTION_KEY").expect("Please set the NOTION_KEY environment variable.");

    // easy to call .unwrap() here cause "url" is required
    let url = arguments.value_of("url").unwrap().to_string();
    let tags = arguments.values_of("tag").map(|values| values.map(|s| s.to_string()).collect());
    let title = retrieve_title(&url);

    let item = entity::Item::new(database_id, title, url, tags);
    let json_item = serde_json::to_string(&item).unwrap();

    post_item(json_item, secret_key).expect("Error on sending the item to Notion.");
}