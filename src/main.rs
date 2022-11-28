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
    Help,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "list" => Ok(Action::List),
            "done" => Ok(Action::Done),
            "add" => Ok(Action::Add),
            "delete" => Ok(Action::Delete),
            "undo" => Ok(Action::Undo),
            _ => Err(String::from("unsupported action")),
        }
    }
}

fn file() -> std::io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILENAME)
}

fn read_file() -> std::io::Result<String> {
    let mut contents = String::new();
    file()?.read_to_string(&mut contents)?;
    return Ok(contents);
}

fn write_file(todos: Vec<Todo>) -> std::io::Result<String> {
    let contents = todos
        .iter()
        .map(|todo| todo.to_s())
        .collect::<Vec<String>>()
        .join("\n");
    let mut f = file()?;
    f.set_len(0)?;
    f.write_all(contents.as_bytes())?;
    return Ok(contents);
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

struct App {
    args: Vec<String>,
    contents: String,
}

impl App {
    fn todos(&self) -> Vec<Todo> {
        let mut todos: Vec<Todo> = vec![];
        for line in self.contents.lines() {
            if let Some(todo) = Self::todo(line.to_string()) {
                todos.push(todo)
            }
        }
        return todos;
    }

    pub fn todo(line: String) -> Option<Todo> {
        if let Some(content) = line.strip_prefix("- [] ") {
            Some(Todo::new(content, false))
        } else if let Some(content) = line.strip_prefix("- [x] ") {
            Some(Todo::new(content, true))
        } else {
            None
        }
    }

    fn list(self) -> Result<String, std::io::Error> {
        let todos = self.todos();
        let mut x = 0;
        let mut output = vec![];
        for todo in todos {
            output.push(format!("{} {}", x, todo));
            x += 1;
        }
        return Ok(output.join("\n"));
    }

    fn add(&self) -> Vec<Todo> {
        let content: String = self
            .args
            .clone()
            .into_iter()
            .skip(1)
            .collect::<Vec<String>>()
            .join(" ");
        let mut todos = self.todos();
        todos.push(Todo::new(&content, false));
        return todos;
    }

    fn command(&self) -> Action {
        if self.args.len() > 0 {
            match self.args[0].parse() {
                Ok(action) => action,
                Err(_) => Action::Help,
            }
        } else {
            Action::List
        }
    }

    fn help(&self) -> String {
        String::from("Try add, done, delete or undo")
    }

    fn done(&self) -> Vec<Todo> {
        self.update(true)
    }

    fn index(&self) -> usize {
        if let Some(idx) = self.args.get(1) {
            return idx.parse().unwrap_or(0);
        } else {
            return 0;
        }
    }

    fn update(&self, completed: bool) -> Vec<Todo> {
        let mut todos = self.todos();
        if let Some(mut todo) = todos.get_mut(self.index()) {
            todo.completed = completed;
        }
        return todos;
    }

    fn delete(&self) -> Vec<Todo> {
        let mut todos = self.todos();
        let index = self.index();
        if let Some(_) = todos.get_mut(index) {
            todos.remove(index);
        }
        return todos;
    }

    fn undo(&self) -> Vec<Todo> {
        self.update(false)
    }
}

fn main() -> std::io::Result<()> {
    // get the input to the program
    // call it state, why not
    let args: Vec<String> = env::args().skip(1).collect();
    let contents: String = read_file()?;
    let app = App { args, contents };

    let output = match app.command() {
        Action::List => app.list()?,
        Action::Add => write_file(app.add())?,
        Action::Done => write_file(app.done())?,
        Action::Delete => write_file(app.delete())?,
        Action::Undo => write_file(app.undo())?,
        Action::Help => app.help(),
    };

    println!("{}", output);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        if let Some(todo) = App::todo(String::from("- [] hello")) {
            assert_eq!(todo.content, String::from("hello"));
            assert_eq!(todo.completed, false);
        }
    }

    #[test]
    fn it_works_when_completed() {
        if let Some(todo) = App::todo(String::from("- [x] hello")) {
            assert_eq!(todo.content, String::from("hello"));
            assert_eq!(todo.completed, true);
        }
    }
}
