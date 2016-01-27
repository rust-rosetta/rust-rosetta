// http://rosettacode.org/wiki/Rosetta_Code/Find_unimplemented_tasks
extern crate hyper;
extern crate rustc_serialize;

use std::collections::HashSet;
use std::io::prelude::*;

use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json::Json;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
}

struct Category {
    name: String,
    cmcontinue: Option<String>,
    http_client: Client,
    is_first_iteration: bool,
}

impl Category {
    fn new(name: &str) -> Category {
        Category {
            name: name.to_owned(),
            cmcontinue: None,
            http_client: Client::new(),
            is_first_iteration: true,
        }
    }
}

impl Iterator for Category {
    type Item = Vec<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cmcontinue.is_none() && !self.is_first_iteration {
            return None;
        }

        self.is_first_iteration = false;

        let cmcontinue = match self.cmcontinue {
            Some(ref cmcontinue) => format!("&cmcontinue={}", cmcontinue),
            None => "".to_owned(),
        };

        let url = &format!("http://rosettacode.org/mw/api.\
                            php?action=query&list=categorymembers&cmtitle=Category:\
                            {}&cmlimit=500&format=json{}",
                           self.name,
                           cmcontinue);

        let mut res = self.http_client
                          .get(url)
                          .header(Connection::close())
                          .send()
                          .unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        let json = Json::from_str(&body).unwrap();
        let tasks = json.as_object()
                        .unwrap()
                        .get("query")
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .get("categorymembers")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|cm| {
                            let cm_obj = cm.as_object().unwrap();
                            Task {
                                id: cm_obj.get("pageid")
                                          .unwrap()
                                          .as_u64()
                                          .unwrap()
                                          .to_string(),
                                title: cm_obj.get("title")
                                             .unwrap()
                                             .as_string()
                                             .unwrap()
                                             .to_owned(),
                            }
                        })
                        .collect::<Vec<_>>();

        self.cmcontinue = json.as_object()
                              .unwrap()
                              .get("query-continue")
                              .map(|query_continue| {
                                  query_continue.as_object()
                                                .unwrap()
                                                .get("categorymembers")
                                                .unwrap()
                                                .as_object()
                                                .unwrap()
                                                .get("cmcontinue")
                                                .unwrap()
                                                .as_string()
                                                .unwrap()
                                                .to_owned()
                              });

        Some(tasks)
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

pub fn main() {
    for task in unimplemented_tasks("Rust") {
        println!("{:6} {}", task.id, task.title);
    }
}
