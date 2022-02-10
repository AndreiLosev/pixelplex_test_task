
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
    }

    #[derive(PartialEq, Debug)]
    pub struct DirectionalGraff<T, U> {
        nods: Vec<Node<T, U>>
    }

    impl<T, U> DirectionalGraff<T, U> {

        pub fn new(first_node_value: T) -> Self {
            let firs_node: Node<_, U> = Node {
                value: first_node_value,
                ribs: Vec::new(),
            };
            Self{ nods: vec![firs_node] }
        }

        pub fn len(self) -> usize {
            self.nods.len()
        }

        pub fn add_node(&mut self, from: usize, rib_value: U, node_value: T) -> Result<(), &str> {
            let mut node = match self.nods.get(from) {
                Some(node) => node,
                None => return  Err("no node with this number exists"),
            };

            let new_node_number = self.nods.len();

            let new_node: Node<_, U> = Node {
                value: node_value,
                ribs: Vec::new(),
            };

            self.nods.push(new_node);

            let rib: Rib<U> = Rib {
                target: new_node_number,
                value: rib_value,
            };

            // node.ribs = vec![rib];

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

}



// #[cfg(test)]
// mod tests {
//     use crate::graff::{self, DirectionalGraff};
// }
