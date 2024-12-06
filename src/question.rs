use std::ops::Deref;

use qwreey_rocket::{RocketBuild, RouteExport};
use qwreey_utility_rs::{ErrToString, HeadingError};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::cli::Cli;

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub title: String,
    pub description: String,
    pub answer_regex: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionList(pub Vec<Question>);
impl Deref for QuestionList {
    type Target = Vec<Question>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct QuestionRegexList(Vec<Regex>);
impl Deref for QuestionRegexList {
    type Target = Vec<Regex>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl QuestionRegexList {
    pub fn from_list(list: &QuestionList) -> Result<Self, String> {
        let mut question_regex_list = Vec::<Regex>::with_capacity(list.len());
        for question in &list.0 {
            question_regex_list.push(Regex::new(question.answer_regex.as_str()).err_tostring()?);
        }
        Ok(Self(question_regex_list))
    }
}

pub struct QuestionInit;
impl RouteExport for QuestionInit {
    fn build(
        &self,
        rocket: RocketBuild,
        userdata: qwreey_utility_rs::ArcRwUserdata,
    ) -> Result<RocketBuild, String> {
        let args = userdata.get_of::<Cli>().unwrap();
        // Parse question list
        let question_list: QuestionList =
            serde_json::from_str(args.questions.as_ref().map_or("[]", |f| f.as_str()))
                .err_tostring()
                .heading_error("Error in parsing questions: ")?;

        drop(args);

        userdata.insert_of(
            QuestionRegexList::from_list(&question_list)
                .heading_error("Error in parsing question regexs: ")?,
        );
        userdata.insert_of(question_list);
        Ok(rocket)
    }
}
