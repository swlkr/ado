use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};

const FILENAME: &str = "./TODO.txt";

#[derive(Default)]
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

    fn file() -> Result<File> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(FILENAME)
    }

    fn read() -> Result<String> {
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

    fn add(content: String, completed: bool) -> Result<Vec<Todo>> {
        let mut todos = Todo::all();
        let todo = Todo::new(&content, completed);
        todos.push(todo);
        Todo::write(&todos)?;
        return Ok(todos);
    }

    fn complete_by(index: usize) -> Result<Vec<Todo>> {
        let mut todos = Todo::all();
        let mut todo = &mut todos[index];
        todo.completed = true;
        Todo::write(&mut todos)?;
        return Ok(todos);
    }

    fn write(todos: &Vec<Todo>) -> Result<()> {
        let contents = todos
            .iter()
            .map(|todo| todo.to_s())
            .collect::<Vec<String>>()
            .join("\n");
        Self::file()?.write_all(contents.as_bytes())?;
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

fn list() -> Vec<Todo> {
    Todo::all()
}

fn add(args: Vec<&str>) -> Result<Vec<Todo>> {
    Todo::add(args.join(" "), false)
}

fn done(index: usize) -> Result<Vec<Todo>> {
    Todo::complete_by(index)
}

fn main() -> Result<()> {
    if let Ok(todos) = done(0) {
        for todo in todos {
            println!("{}", todo);
        }
    };
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
