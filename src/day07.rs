use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

struct FileInfo {
    name: String,
    size: u128,
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

struct Node {
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    // data: Vec<FileInfo>,
}

impl Node {
    fn make(name: &str) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            name: String::from(name),
            parent: None,
            children: Vec::new(),
            // data: Vec::new(),
        }))
    }

    fn add_child(&mut self, new_child: Rc<RefCell<Node>>) {
        self.children.push(new_child);
    }

    fn print(&self) -> String {
        format!(
            "{}[{}]",
            self.name,
            self.children
                .iter()
                .map(|n| n.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

pub fn solve() {
    let raw = fs::read_to_string("data/day07.example").unwrap();
    let root = Node::make("/");
    let mut cur = root;

    for line in raw.lines() {
        if line.starts_with("$ cd") {
            let dir = line.split(" ").last().unwrap();
            match dir {
                "/" => (),
                ".." => {
                    let cur_clone = Rc::clone(&cur);
                    cur = Rc::clone(cur_clone.borrow().parent.as_ref().unwrap());
                }
                descend_dir => {
                    let new_node = Node::make(descend_dir);
                    let new_clone = Rc::clone(&new_node);
                    new_node.borrow_mut().parent = Some(Rc::clone(&cur));
                    cur = Rc::clone(new_clone.borrow().parent.as_ref().unwrap());
                    cur.borrow_mut().add_child(new_node);
                }
            }
        }
    }

    let cur_clone = Rc::clone(&cur);
    cur = Rc::clone(cur_clone.borrow().parent.as_ref().unwrap());
    println!("{}", cur.borrow().print());
}
