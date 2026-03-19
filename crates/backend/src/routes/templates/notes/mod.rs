use askama::Template;

use crate::models::notes::Note;

#[derive(Template)]
#[template(path = "notes/base.html")]
struct Base;

pub fn root() -> String {
    Base.render().unwrap()
}

#[derive(Template)]
#[template(path = "notes/list.html")]
struct List {
    notes: Vec<Note>,
}
pub fn list(notes: Vec<String>) -> String {
    let notes = List {
        notes: notes
            .into_iter()
            .enumerate()
            .map(|(i, content)| Note {
                name: i.to_string(),
                content,
            })
            .collect(),
    };

    notes.render().unwrap()
}

#[derive(Template)]
#[template(path = "notes/new.html")]
struct New;

pub fn new() -> String {
    New.render().unwrap()
}

#[derive(Template)]
#[template(path = "notes/edit.html")]
struct Edit<'a> {
    id: u32,
    content: &'a str,
}

pub fn edit(id: u32, content: &str) -> String {
    Edit { id, content }.render().unwrap()
}
