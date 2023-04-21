use std::net::TcpStream;
use std::thread::{self, JoinHandle};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};
use crossterm::execute;
use std::io::stdout;

pub fn menu(stream: &Option<TcpStream>) {
    print!("  eztcp | ");
    match &stream {
        Some(s) => print!("Connected to {}\n", s.peer_addr().unwrap()),
        None => print!("Not connected\n")
    }

    let mut menu = Menu::new();
    menu.add_item("Connect to server".to_string());
    menu.add_item("Send a message & print response".to_string());
    menu.add_item("Disconnect to server".to_string());
    menu.add_item("Exit".to_string());
    menu.draw();
    
    loop {
        // `read()` blocks until an `Event` is available
        match read().unwrap() {
            Event::Key(event) 
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Up => {
                // println!("{:?}", event);
                menu.selected -= 1;
            }
            Event::Key(event) 
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Down => {
                // println!("{:?}", event);
                menu.selected += 1;
            }
            Event::Key(event)
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Enter => {
                println!("`{}` was selected!", menu.items[menu.selected].content);
            }
            _ => {}
        }
    }
}

struct Menu {
    items: Vec<Item>,
    selected: usize,
}

impl Menu {
    pub fn new() -> Menu { Menu { items: vec![], selected: 1}}

    pub fn add_item(&mut self, content: String) {
        self.items.push(Item::new(content));
    }

    pub fn draw(&mut self) {
        for (pos, item) in self.items.iter_mut().enumerate() {
            // println!("pos: {pos}, item = {:?}", item);
            if (pos + 1) == self.selected {
                item.selected = true;
                execute!(
                    stdout(),
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(Color::White),
                    Print(&item.content),
                    ResetColor
                ).expect("ERROR with colors");
                println!();
            }else {
                item.selected = false;
                println!("{}", &item.content);
            }
        }
    }
}

#[derive(Debug)]
struct Item{
    pub content: String,
    pub action: (), // action will be a closure
    pub selected: bool
}

impl Item {
    pub fn new(content: String) -> Item { Item { content, action: (), selected: false} }
}

use crossterm::event::{read, Event, KeyEventKind, KeyCode};

pub fn print_events() -> crossterm::Result<()> {
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) 
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Up => {
                println!("{:?}", event);
            }
            Event::Key(event) 
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Down => {
                println!("{:?}", event);
            }
            Event::Key(event)
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Enter => {
                println!("{:?}", event);
            }
            _ => {}
        }
    }
}