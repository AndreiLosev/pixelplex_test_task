
pub mod graff {

    #[derive(PartialEq, Debug)]
    struct  Node<T, U> {
        value: T,
        ribs: Vec<Rib<U>>
    }

    #[derive(PartialEq, Debug)]
    struct Rib<T> {
        target: u64,
        value: T,
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

        // pub fn add_node()
    }

    #[test]
    fn it_create_new() {
        let result: DirectionalGraff<u8, i8> = DirectionalGraff::new(2);
        let example_node: Node<u8, i8> = Node { value: 2, ribs: Vec::new() };
        let example: DirectionalGraff<u8, i8> = DirectionalGraff { nods: vec![example_node] };
        assert_eq!(result, example);
    }

}



// #[cfg(test)]
// mod tests {
//     use crate::graff::{self, DirectionalGraff};
// }
