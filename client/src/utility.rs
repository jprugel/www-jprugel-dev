use serde::{Serialize, Deserialize};
use std::fmt::*;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Feed {
    pub articles: Vec<Post>
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub hook: String,
    pub body: String
}

impl Post {
    pub fn builder() -> PostBuilder {
        PostBuilder::default()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_date(&self) -> String {
        self.date.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }
}

#[derive(Default)]
pub struct PostBuilder {
    id: usize,
    title: String,
    date: String,
    hook: String,
    body: String,
}

impl PostBuilder {
    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn set_date(mut self, date: &str) -> Self {
        self.date = date.to_string();
        self
    }

    pub fn set_hook(mut self, hook: &str) -> Self {
        self.hook = hook.to_string();
        self
    }

    pub fn set_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn build(self) -> Post {
        let id = self.id;
        let title = self.title;
        let date = self.date;
        let hook = self.hook;
        let body = self.body;

        Post {
            id,
            title: title.to_string(),
            date: date.to_string(),
            hook: hook.to_string(),
            body: body.to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct SearchQuery(pub String);

impl SearchQuery {
    pub fn new(query: &str) -> Self {
        Self(query.to_string())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Themes {
    Latte,
    Frappe,
}

impl Themes {
    pub fn from_string(target: String) -> Themes {
        match target.as_str() {
            "latte" => Themes::Latte,
            "frappe" => Themes::Frappe,
            _ => Themes::Latte,
        }
    }
}

impl Display for Themes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Themes::Latte => write!(f, "latte"),
            Themes::Frappe => write!(f, "frappe"),
        }
    }
}
