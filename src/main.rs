use std::env;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::str::FromStr;

const FILENAME: &str = "./TODO.txt";

#[derive(Debug)]
enum Action {
    Add,
    Done,
    List,
    Delete,
    Undo,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(Action::List),
            "done" => Ok(Action::Done),
            "add" => Ok(Action::Add),
            "del" => Ok(Action::Delete),
            "undo" => Ok(Action::Undo),
            _ => Err(String::from("unsupported action")),
        }
    }
}

#[derive(Default, Debug)]
struct Todo {
    completed: bool,
    content: String,
}

impl Todo {
    fn new(content: &str, completed: bool) -> Self {
        Self {
            content: content.to_string(),
            completed,
        }
    }

    fn file() -> std::io::Result<File> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(FILENAME)
    }

    fn read() -> std::io::Result<String> {
        let mut contents = String::new();
        Self::file()?.read_to_string(&mut contents)?;
        return Ok(contents);
    }

    fn parse(contents: String) -> Vec<Self> {
        let mut todos: Vec<Todo> = vec![];
        for line in contents.lines() {
            if let Some(todo) = Self::parse_line(line.to_string()) {
                todos.push(todo)
            }
        }
        return todos;
    }

    pub fn parse_line(line: String) -> Option<Self> {
        if let Some(content) = line.strip_prefix("- [] ") {
            Some(Todo::new(content, false))
        } else if let Some(content) = line.strip_prefix("- [x] ") {
            Some(Todo::new(content, true))
        } else {
            None
        }
    }

    // this will call read
    // and parse each new line into todo
    fn all() -> Vec<Self> {
        match Self::read() {
            Ok(contents) => Self::parse(contents),
            Err(_) => todo!(),
        }
    }

    fn add(content: String, completed: bool) -> std::io::Result<Vec<Todo>> {
        let mut todos = Todo::all();
        let todo = Todo::new(&content, completed);
        todos.push(todo);
        Todo::write(&todos)?;
        return Ok(todos);
    }

    fn update_by(index: usize, completed: bool) -> std::io::Result<Vec<Todo>> {
        let mut todos = Todo::all();
        let mut todo = &mut todos[index];
        todo.completed = completed;
        Todo::write(&mut todos)?;
        return Ok(todos);
    }

    fn delete_by(index: usize) -> std::io::Result<Vec<Todo>> {
        let mut todos = Todo::all();
        todos.remove(index);
        Todo::write(&todos)?;
        return Ok(todos);
    }

    fn write(todos: &Vec<Todo>) -> std::io::Result<()> {
        let contents = todos
            .iter()
            .map(|todo| todo.to_s())
            .collect::<Vec<String>>()
            .join("\n");
        let mut f = Self::file()?;
        f.set_len(0)?;
        f.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn to_s(&self) -> String {
        let x = match self.completed {
            true => "x",
            false => "",
        };
        return format!("- [{}] {}", x, self.content);
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.to_s()))
    }
}

fn list() {
    let todos = Todo::all();
    let mut x = 0;
    for todo in todos {
        println!("{} {}", x, todo);
        x += 1;
    }
}

fn add(args: Vec<String>) -> std::io::Result<Vec<Todo>> {
    Todo::add(args.join(" "), false)
}

fn done(index: usize) -> std::io::Result<Vec<Todo>> {
    Todo::update_by(index, true)
}

fn delete(index: usize) -> std::io::Result<Vec<Todo>> {
    Todo::delete_by(index)
}

fn undo(index: usize) -> std::io::Result<Vec<Todo>> {
    Todo::update_by(index, false)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        if let Ok(action) = args[0].parse() {
            match action {
                Action::List => {
                    list();
                }
                Action::Add => {
                    add(args.into_iter().skip(1).collect::<Vec<String>>())?;
                    list();
                }
                Action::Done => {
                    if args.len() > 1 {
                        let idx: usize = args[1].parse().unwrap();
                        done(idx)?;
                        list();
                    }
                }
                Action::Delete => {
                    if args.len() > 1 {
                        let idx: usize = args[1].parse().unwrap();
                        delete(idx)?;
                        list();
                    }
                }
                Action::Undo => {
                    if args.len() > 1 {
                        let idx: usize = args[1].parse().unwrap();
                        undo(idx)?;
                        list();
                    }
                }
            }
        } else {
            println!("Try add, list, done, del or undo")
        }
    } else {
        list();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        if let Some(todo) = Todo::parse_line(String::from("- [] hello")) {
            assert_eq!(todo.content, String::from("hello"));
            assert_eq!(todo.completed, false);
        }
    }

    #[test]
    fn it_works_when_completed() {
        if let Some(todo) = Todo::parse_line(String::from("- [x] hello")) {
            assert_eq!(todo.content, String::from("hello"));
            assert_eq!(todo.completed, true);
        }
    }
}
