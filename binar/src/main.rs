#[derive(Debug)]
struct Node<T, U> {
    value: T,
    to: Vec<Rib<U>>,
}

#[derive(Debug)]
struct Rib<T> {
    target: u128,
    value: T,
}

fn main() {

    let to_example: Vec<Rib<&str>> = Vec::new();

    let x = Node {
        value: "test",
        to: to_example,
    };

    dbg!(x.value, x.to);
}
