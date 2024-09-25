#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

fn main(){
    let list = List{head: Link::More(Box::new(Node{elem: 1, next: Link::Empty}))};
    println!("{:?}", list);
}