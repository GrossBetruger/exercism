
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

struct LinkedList<T> {
    head: Option<Node<T>>,

}

fn main() {
    let list = LinkedList {
        head: Some(Node {
            data: 1,
            next: None
        })
    };

    println!("Hello, world!");
}
