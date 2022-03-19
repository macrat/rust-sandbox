use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
enum ItemError {
    IsNotList,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Symbol(String),
    Int(i64),
    String(String),
    Cons(Box<Item>, Box<Item>),
    Nil,
}

impl Item {
    fn pair(car: Item, cdr: Item) -> Item {
        Item::Cons(Box::new(car), Box::new(cdr))
    }

    fn single(val: Item) -> Item {
        Item::pair(val, Item::Nil)
    }

    fn parse_atom(s: String) -> Item {
        if let Ok(i) = s.parse::<i64>() {
            Item::Int(i)
        } else if let Some('"') = s.chars().next() {
            Item::String(s[1..s.len() - 1].to_string())
        } else {
            Item::Symbol(s)
        }
    }

    fn push(&mut self, val: Item) -> Result<(), ItemError> {
        let mut current = self;
        loop {
            match current {
                Item::Nil => {
                    *current = Item::single(val);
                    return Ok(());
                }
                Item::Cons(_, cdr) => {
                    current = &mut **cdr;
                }
                _ => return Err(ItemError::IsNotList),
            }
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Symbol(name) => write!(f, "{}", name),
            Item::Int(number) => write!(f, "{}", number),
            Item::String(string) => write!(f, "\"{}\"", string),
            Item::Cons(_, _) => {
                let mut current = self;
                let mut xs: Vec<String> = Vec::new();
                loop {
                    match current {
                        Item::Nil => break,
                        Item::Cons(car, cdr) => {
                            xs.push(format!("{}", car));
                            current = cdr;
                        }
                        _ => {
                            xs.push(format!(". {}", current));
                        }
                    }
                }
                write!(f, "({})", xs.join(" "))
            }
            Item::Nil => write!(f, "()"),
        }
    }
}

impl From<String> for Item {
    fn from(input: String) -> Item {
        let mut list = Item::Nil;
        for x in ListIterator::new(&mut input.chars()) {
            let _ = list.push(x);
        }
        list
    }
}

struct ListIterator<'a> {
    reader: &'a mut dyn Iterator<Item = char>,
    buf: String,
    in_string: bool,
    stack: Vec<Item>,
    result: VecDeque<Item>,
}

impl ListIterator<'_> {
    fn new(reader: &mut dyn Iterator<Item = char>) -> ListIterator {
        ListIterator {
            reader,
            buf: String::new(),
            in_string: false,
            stack: Vec::new(),
            result: VecDeque::new(),
        }
    }

    fn flush(&mut self) {
        if self.buf.len() > 0 {
            let buf = std::mem::replace(&mut self.buf, String::new());
            let value = Item::parse_atom(buf);

            if let Some(mut last) = self.stack.pop() {
                let _ = last.push(value);
                self.stack.push(last);
            } else {
                self.result.push_front(value);
            }
        }
    }

    fn push(&mut self, c: char) {
        match c {
            '(' if !self.in_string => {
                self.flush();
                self.stack.push(Item::Nil);
            }
            ')' if !self.in_string => {
                self.flush();
                if let Some(value) = self.stack.pop() {
                    if let Some(mut last) = self.stack.pop() {
                        let _ = last.push(value);
                        self.stack.push(last);
                    } else {
                        self.result.push_front(value);
                    }
                } else {
                    // TODO: error handling here.
                }
            }
            '"' => {
                self.buf.push(c);
                self.in_string = !self.in_string;
                if !self.in_string {
                    self.flush();
                }
            }
            ' ' | '\t' | '\r' | '\n' => self.flush(),
            _ => self.buf.push(c),
        }
    }
}

impl Iterator for ListIterator<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        while self.result.len() == 0 {
            match self.reader.next() {
                Some(c) => self.push(c),
                None => {
                    self.flush();
                    return self.result.pop_back();
                }
            }
        }
        self.result.pop_back()
    }
}

fn main() {
    println!(
        "{}",
        Item::from(String::from("hello (\"world\" (123 456) 789)"))
    );
}
