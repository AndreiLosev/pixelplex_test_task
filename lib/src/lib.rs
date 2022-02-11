
pub mod graff {
    use std::{result::Result};

    #[derive(PartialEq, Debug)]
    struct Rib<T> {
        target: usize,
        value: T,
    }

    impl<T> Rib<T> {
        fn new(target: usize, value: T) -> Self {
            Rib { target, value }
        }
    }

    #[derive(PartialEq, Debug)]
    struct  Node<T, U> {
        value: T,
        ribs: Vec<Rib<U>>
    }

    impl<T, U> Node<T, U> {
        fn new(value: T) -> Self {
            Node { value, ribs: Vec::new() }
        }

        fn add_rib(&mut self, target: usize, value: U) {
            let new_rib = Rib::new(target, value);
            self.ribs.push(new_rib);
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct DirectionalGraff<T, U> {
        nods: Vec<Node<T, U>>
    }

    impl<T, U> DirectionalGraff<T, U> {

        pub fn new(first_node_value: T) -> Self {
            Self{ nods: vec![Node::new(first_node_value)] }
        }

        pub fn len(self) -> usize {
            self.nods.len()
        }

        pub fn add_node(&mut self, from: usize, rib_value: U, node_value: T) -> Result<(), &str> {

            let new_node_number = self.nods.len();

            if new_node_number <= from {
                return  Err("no node with this number exists");
            }

            self.nods.push(Node::new(node_value));
            self.nods[from].add_rib(new_node_number, rib_value);

            Ok(())
        }

        pub fn add_rib(&mut self, from: usize, to: usize, rib_value: U) -> Result<(), &str> {

            if from >= self.nods.len() || to >= self.nods.len() {
                return  Err("no node with this number exists");
            }

            let new_rib = Rib::new(to, rib_value);
            self.nods[from].ribs.push(new_rib);

            Ok(())
        }
    }

    #[test]
    fn it_create_new() {
        let result: DirectionalGraff<u8, u8> = DirectionalGraff::new(2);
        let example_node: Node<u8, u8> = Node { value: 2, ribs: Vec::new() };
        let example: DirectionalGraff<u8, u8> = DirectionalGraff { nods: vec![example_node] };
        assert_eq!(result, example);
    }

    #[test]
    fn add_node_test() {
        let mut result: DirectionalGraff<u8, u8> = DirectionalGraff::new(2);
        result.add_node(0, 5, 4);

        let example_rib1: Rib<u8> = Rib { target: 1, value: 5 };
        let example_node1: Node<u8, u8> = Node { value: 2, ribs: vec![example_rib1] };
        let example_node2: Node<u8, u8> = Node { value: 4, ribs: Vec::new() };
        let example: DirectionalGraff<u8, u8> = DirectionalGraff { nods: vec![example_node1, example_node2] };
        assert_eq!(result, example);
    }

}
