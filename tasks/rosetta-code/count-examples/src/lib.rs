#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Task {
    #[serde(rename = "pageid")]
    page_id: u64,
    pub title: String,
}

#[derive(Debug)]
enum ParseError {
    /// Something went wrong with the HTTP request to the API.
    Http(reqwest::Error),

    /// There was a problem parsing the API response into JSON.
    Json(serde_json::Error),

    /// Unexpected JSON format from response
    UnexpectedFormat,
}

impl From<serde_json::Error> for ParseError {
    fn from(error: serde_json::Error) -> Self {
        ParseError::Json(error)
    }
}

impl From<reqwest::Error> for ParseError {
    fn from(error: reqwest::Error) -> Self {
        ParseError::Http(error)
    }
}

fn construct_query_category(category: &str) -> Url {
    let mut base_url = Url::parse("http://rosettacode.org/mw/api.php").unwrap();
    let cat = format!("Category:{}", category);
    let query_pairs = vec![("action", "query"),
                           ("format", "json"),
                           ("list", "categorymembers"),
                           ("cmlimit", "500"),
                           ("cmtitle", &cat),
                           ("continue", "")];
    base_url.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    base_url
}

fn construct_query_task_content(task_id: &str) -> Url {
    let mut base_url = Url::parse("http://rosettacode.org/mw/api.php").unwrap();
    let mut query_pairs =
        vec![("action", "query"), ("format", "json"), ("prop", "revisions"), ("rvprop", "content")];
    query_pairs.push(("pageids", task_id));
    base_url.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    base_url
}

fn query_api(url: Url) -> Result<Value, ParseError> {
    Ok(reqwest::get(url)?.json()?)
}

fn parse_all_tasks(reply: &Value) -> Result<Vec<Task>, ParseError> {
    let tasks_json = reply.pointer("/query/categorymembers")
        .and_then(|tasks| tasks.as_array())
        .ok_or(ParseError::UnexpectedFormat)?;

    tasks_json.iter().map(|json| Task::deserialize(json).map_err(From::from)).collect()
}

fn count_number_examples(task: &Value, task_id: u64) -> Result<u32, ParseError> {
    let revisions = task.pointer(&format!("/query/pages/{}/revisions", task_id.to_string()))
        .and_then(|content| content.as_array())
        .ok_or(ParseError::UnexpectedFormat)?;
    let content = revisions[0]
        .get("*")
        .and_then(Value::as_str)
        .ok_or(ParseError::UnexpectedFormat)?;
    Ok(content.split("=={{header").count() as u32)
}

pub fn query_all_tasks() -> Vec<Task> {
    let query = construct_query_category("Programming_Tasks");
    let json: Value = query_api(query).unwrap();
    parse_all_tasks(&json).unwrap()
}

pub fn query_a_task(task: &Task) -> u32 {
    let query = construct_query_task_content(&task.page_id.to_string());
    let json: Value = query_api(query).unwrap();
    count_number_examples(&json, task.page_id).unwrap()
}
