use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Feed {
    pub blogs: Vec<Post>
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Post {
    id: usize,
    title: String,
    date: String,
    body: String
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

    pub fn set_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn build(self) -> Post {
        let id = self.id;
        let title = self.title;
        let date = self.date;
        let body = self.body;

        Post {
            id,
            title: title.to_string(),
            date: date.to_string(),
            body: body.to_string(),
        }
    }
}
