#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::collections::{BTreeMap, HashSet};

use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;

/// A Rosetta Code task.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
pub struct Task {
    /// The ID of the page containing the task in the MediaWiki API.
    #[serde(rename = "pageid")]
    pub id: u64,

    /// The human-readable title of the task.
    pub title: String,
}

/// Encapsulates errors that might occur during JSON parsing.
#[derive(Debug)]
enum TaskParseError {
    /// Something went wrong with the HTTP request to the API.
    Http(reqwest::Error),

    /// Could not parse a URL
    Url(reqwest::UrlError),

    /// There was a problem parsing the API response into JSON.
    Json(serde_json::Error),

    /// The response JSON contained unexpected keys or values.
    UnexpectedFormat,
}

impl From<serde_json::Error> for TaskParseError {
    fn from(err: serde_json::Error) -> Self {
        TaskParseError::Json(err)
    }
}

impl From<reqwest::UrlError> for TaskParseError {
    fn from(err: reqwest::UrlError) -> Self {
        TaskParseError::Url(err)
    }
}

impl From<reqwest::Error> for TaskParseError {
    fn from(err: reqwest::Error) -> Self {
        TaskParseError::Http(err)
    }
}

/// Represents a category of pages on Rosetta Code, such as "Rust".
struct Category {
    name: String,
    continue_params: Option<BTreeMap<String, String>>,
}

impl Category {
    fn new(name: &str) -> Category {
        let mut continue_params = BTreeMap::new();
        continue_params.insert("continue".to_owned(), "".to_owned());

        Category {
            name: name.to_owned(),
            continue_params: Some(continue_params),
        }
    }
}

/// Sends a request to Rosetta Code through the MediaWiki API. If successful, returns the response
/// as a JSON object.
fn query_api(category_name: &str,
             continue_params: &BTreeMap<String, String>)
             -> Result<Value, TaskParseError> {
    let mut url = Url::parse("http://rosettacode.org/mw/api.php")?;
    url.query_pairs_mut()
        .append_pair("action", "query")
        .append_pair("list", "categorymembers")
        .append_pair("cmtitle", &format!("Category:{}", category_name))
        .append_pair("cmlimit", "500")
        .append_pair("format", "json")
        .extend_pairs(continue_params);

    Ok(reqwest::get(url)?.json()?)
}

/// Given a JSON object, parses the task information from the MediaWiki API response.
fn parse_tasks(json: &Value) -> Result<Vec<Task>, TaskParseError> {
    let tasks_json = json.pointer("/query/categorymembers")
        .and_then(Value::as_array)
        .ok_or(TaskParseError::UnexpectedFormat)?;

    tasks_json.iter()
        .map(|json| Task::deserialize(json).map_err(From::from))
        .collect()
}

impl Iterator for Category {
    type Item = Vec<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.continue_params.is_none() {
            return None;
        }

        query_api(&self.name, self.continue_params.as_ref().unwrap())
            .and_then(|result| {
                // If there are more pages of results to request, save them for the next iteration.
                self.continue_params = result.get("continue")
                    .and_then(Value::as_object)
                    .map(|continue_params| {
                        continue_params.iter()
                            .map(|(key, value)| {
                                (key.to_owned(),
                                 value.as_str()
                                    .unwrap()
                                    .to_owned())
                            })
                            .collect()
                    });

                parse_tasks(&result)
            })
            .map_err(|err| println!("Error parsing response: {:?}", err))
            .ok()
    }
}

pub fn all_tasks() -> Vec<Task> {
    Category::new("Programming Tasks")
        .flat_map(|tasks| tasks)
        .collect()
}

pub fn unimplemented_tasks(lang: &str) -> Vec<Task> {
    let all_tasks = all_tasks().iter().cloned().collect::<HashSet<_>>();
    let implemented_tasks = Category::new(lang)
        .flat_map(|tasks| tasks)
        .collect::<HashSet<_>>();
    let mut unimplemented_tasks = all_tasks.difference(&implemented_tasks)
        .cloned()
        .collect::<Vec<Task>>();
    unimplemented_tasks.sort_by(|a, b| a.title.cmp(&b.title));
    unimplemented_tasks
}
