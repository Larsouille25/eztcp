use std::net::TcpStream;
use std::io::{stdout, Write};

use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color};
use crossterm::{execute, ExecutableCommand, terminal, cursor};
use crossterm::cursor::MoveToPreviousLine;
use crossterm::event::{read, Event, KeyEventKind, KeyCode};


pub fn menu(stream: &Option<TcpStream>) {
    print!("  eztcp | ");
    match &stream {
        Some(s) => print!("Connected to {}\n", s.peer_addr().unwrap()),
        None => print!("Not connected\n")
    }

    let mut menu = Menu::new();
    menu.add_item(" Connect to server                ".to_string(), Box::new(|_a| {}));
    menu.add_item(" Send a message & print response  ".to_string(), Box::new(|_a| {}));
    menu.add_item(" Disconnect to server             ".to_string(), Box::new(|_a| {}));
    menu.add_item(" Exit                             ".to_string(), Box::new(|_a| {}));
    menu.draw();
    let mut event_read = true;
    
    loop {
        // `read()` blocks until an `Event` is available
        match read().unwrap() {
            Event::Key(event) 
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Up => {
                if event_read {
                    // println!("{:?}", event);
                    menu.item_up();
                    // println!("menu.selected = {}", menu.selected);
                    menu.move_cursor();
                    menu.draw();
                }
            }
            Event::Key(event) 
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Down => {
                if event_read {
                    // println!("{:?}", event);
                    menu.item_down();
                    // println!("menu.selected = {}", menu.selected);
                    menu.move_cursor();
                    menu.draw();
                }
            }
            Event::Key(event)
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Enter => {
                stdout().execute(terminal::Clear(terminal::ClearType::Purge)).expect("ERR: terminal can't be clear");
                event_read = false;
                stdout().execute(cursor::MoveTo(1, 1)).expect("ERR: Can't move the cursor");

                println!("`{}` was selected!", menu.items[menu.selected - 1].content);
                println!("AAAAAAAAAAAAAAAAAAAAAAAA");
                println!("AAAAAAAAAAAAAAAAAAAAAAAA");
                println!("AAAAAAAAAAAAAAAAAAAAAAAA");
                println!("AAAAAAAAAAAAAAAAAAAAAAAA");

                
                stdout().execute(terminal::Clear(terminal::ClearType::All)).expect("ERR: terminal can't be clear");
                event_read = true;
                // menu.draw();
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

    pub fn add_item(&mut self, content: String, action: Box<dyn Fn(TcpStream)>) {
        self.items.push(Item::new(content, action));
    }

    pub fn item_up(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len();
        } else if self.selected == 1 {
            self.selected = self.items.len();
        }else {
            self.selected -= 1;
        }
    }

    pub fn item_down(&mut self) {
        if self.selected == self.items.len() {
            self.selected = 1;
        } else {
            self.selected += 1;
        }
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

    pub fn move_cursor(&self) {
        
        match stdout().execute(MoveToPreviousLine( self.items.len() as u16 )) {
            Ok(..) => {}
            Err(err) => {
                println!("ERR: {}", err);
            }
        }
    }
}

struct Item{
    pub content: String,
    pub action: Box<dyn Fn(TcpStream)>, // action will be a closure
    pub selected: bool
}

impl Item{
    pub fn new(content: String, action:  Box<dyn Fn(TcpStream)>) -> Item
    {
        Item { content, action: action, selected: false}
    }
}