use lib::graff::{DirectionalGraff, Node};
use std::{fs, env};

fn main() {

    let args = env::args().collect::<Vec<_>>();

    let path_to_file = &args[1];

    let str_graff = fs::read_to_string(path_to_file).unwrap();

    let mut graff: DirectionalGraff<String, String> = DirectionalGraff::new(String::new());
    graff.dessireolization(str_graff).unwrap();

    let exit_cond = |node: &Node<String, String>, i: usize| {
        let string = format!("{} -> {}", i, node);
        println!("{}", string);
        return false;
    };

    graff.bfs(0, Box::new(exit_cond)).unwrap();
}
