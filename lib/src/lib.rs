
pub mod graff {
    use std::{result};
    use std::collections::HashMap;
    use std::str;
    use std::fmt;
    use queues::*;

    #[derive(PartialEq, Debug)]
    pub struct Rib<T> {
        target: usize,
        value: T,
    }

    impl<T> Rib<T> {
        pub fn new(target: usize, value: T) -> Self {
            Rib { target, value }
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct Node<T, U> {
        value: T,
        ribs: Vec<Rib<U>>
    }

    impl<T, U> Node<T, U> {
        pub fn new(value: T) -> Self {
            Node { value, ribs: Vec::new() }
        }

        fn add_rib(&mut self, target: usize, value: U) {
            let new_rib = Rib::new(target, value);
            self.ribs.push(new_rib);
        }

        fn remove_rib(&mut self, to: usize) {

            let remove_ids = self.ribs.iter()
                .enumerate()
                .filter(|(_, rib)| rib.target == to)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            for i in remove_ids {
                self.ribs.remove(i);
            }
        }

    }

    impl <T: fmt::Display, U> fmt::Display for Node<T, U> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let adjacent = self.ribs.iter().map(|i| i.target).map(|i| i.to_string()).collect::<Vec<_>>().join(", ");
            write!(f, "{}, adjacent: {}",self.value, adjacent)
        }
    }

    #[derive(PartialEq, Debug)]
    pub struct DirectionalGraff<T, U> {
        nodes: HashMap<usize, Node<T, U>>
    }

    impl<T, U> DirectionalGraff<T, U> {

        pub fn get_next_key(&self) -> usize {
            let max_key = self.nodes.keys().max();
            match max_key {
                Some(&k) => k + 1,
                None => panic!("it can't be like that."),
            }
        }

        fn get_mut(&mut self, node_number: &usize) -> result::Result<&mut Node<T, U>, &str> {
            let result = match self.nodes.get_mut(node_number) {
                Some(node) => Ok(node),
                None => Err("no node with this number exists"),
            };

            result
        }

        fn get(&self, node_number: &usize) -> result::Result<&Node<T, U>, &str> {
            let result = match self.nodes.get(node_number) {
                Some(node) => Ok(node),
                None => Err("no node with this number exists"),
            };

            result
        }

        fn push(&mut self, node: Node<T, U>) {
            let next_key = self.get_next_key();
            self.nodes.insert(next_key, node);
        }

        pub fn new(first_node_value: T) -> Self {
            let mut nodes: HashMap<usize, Node<T, U>> = HashMap::new();
            nodes.insert(0, Node::new(first_node_value));
            Self{ nodes }
        }

        pub fn add_node(&mut self, from: usize, rib_value: U, node_value: T) -> Result<(), &str> {

            let new_node_number = self.get_next_key();

            self.push(Node::new(node_value));
            self.get_mut(&from)?.add_rib(new_node_number, rib_value);

            Ok(())
        }

        pub fn add_rib(&mut self, from: usize, to: usize, rib_value: U) -> result::Result<(), &str> {

            let new_rib = Rib::new(to, rib_value);

            self.get_mut(&from)?.ribs.push(new_rib);

            Ok(())
        }

        pub fn bfs(&self, from: usize, exit_condition: Box<dyn Fn(&Node<T, U>, usize) -> bool>) -> result::Result<Option<usize>, &str> {

            let mut queue: Queue<&Node<T, U>> = queue![];
            queue.add(self.get(&from)?).unwrap();

            if exit_condition(queue.peek().unwrap(), from) {
                return Ok(Some(from));
            }

            let mut is_used: HashMap<usize, bool> = HashMap::new();
            is_used.insert(from, true);

            loop {
                if queue.size() == 0 {
                    break Ok(None);
                }

                let node = queue.remove().unwrap();

                for rib in &node.ribs {

                    let next = match is_used.get(&rib.target) {
                        Some(&v) => v,
                        None => false,
                    };

                    if next {
                        continue;
                    } else {
                        is_used.insert(rib.target, true);
                    }


                    let node = self.get(&rib.target)?;
                    if exit_condition(node, rib.target) {
                        return Ok(Some(rib.target));
                    } else {
                        queue.add(node).unwrap();
                    }
                }
            }
        }

        pub fn remove_rib(&mut self, from: usize, to: usize) -> result::Result<(), &str> {

            let from_node = self.get_mut(&from)?;
            from_node.remove_rib(to);

            Ok(())
        }

        pub fn remove_node(&mut self, node_number: usize) -> result::Result<(), &str> {

            for node in self.nodes.values_mut() {
                node.remove_rib(node_number)
            }

            match self.nodes.remove(&node_number) {
                Some(_) => Ok(()),
                None => Err("no node with this number exists"),
            }
        }
    }

    impl<T: str::FromStr + 'static, U: str::FromStr + 'static> DirectionalGraff<T, U> {

        pub fn dessireolization<'a>(&mut self, str: String) -> result::Result<(), &'a str> {

            let mut divisor = 0;

            for (i, char) in str.chars().enumerate() {
                if char == '#' {
                    divisor = i + 1;
                }
            }

            let str_nodes = &str[0..divisor];
            let str_ribs = &str[divisor..];

            let mut raf_nodes: HashMap<usize, Node<T, U>> = HashMap::new();

            for line in str_nodes.rsplit('\n') {

                if line.len() <= 2 {
                    continue;
                }

                let (key, value) = self.parse_line(line)?;

                let value = match value.parse::<T>() {
                    Ok(v) => v,
                    Err(_) => return  Err("serialization error 3"),
                };

                let key = match key.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return  Err("serialization error 4"),
                };

                raf_nodes.insert(key, Node::new(value));
            }

            for line in str_ribs.rsplit('\n') {
                if line.len() <= 2 {
                    continue;
                }

                let (from, value) = self.parse_line(line)?;
                let (to, value) = self.parse_line(&value[..])?;

                let from = match from.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return  Err("serialization error 5"),
                };

                let to = match to.parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => return  Err("serialization error 6"),
                };

                let value = match value.parse::<U>() {
                    Ok(v) => v,
                    Err(_) => return Err("serialization error 7"),
                };

                match raf_nodes.get_mut(&from) {
                    Some(v) => v.ribs.push(Rib::new(to, value)),
                    None => return Err("serialization error 8"),
                }
            }

            self.nodes = raf_nodes;

            Ok(())
        }

        fn parse_line<'a, 'b>(&self, line: &'a str) -> Result<(String, String), &'b str> {
            let mut key = String::new();
            let mut space_position = 0;

            for (i, char) in line.chars().enumerate() {
                if char == ' ' {
                    space_position = i + 1;
                    break;
                }

                key.push(char);
            }

            Ok((key, String::from(&line[space_position..])))
        }
    }

    impl<T: fmt::Display, U: fmt::Display> DirectionalGraff<T, U> {

        pub fn serialize(&self) -> result::Result<String, &str> {

            let mut string_nodes: Vec<String> = Vec::new();
            let mut string_ribs: Vec<String> = Vec::new();

            let mut sort_keys = self.nodes.keys().collect::<Vec<_>>();
            sort_keys.sort();

            for i in sort_keys {
                let node = self.get(i)?;
                let string = format!("{} {}\n", i, node.value);
                string_nodes.push(string);

                for rib in &node.ribs {
                    let string = format!("{} {} {}\n", i, rib.target, rib.value);
                    string_ribs.push(string);
                }
            }

            let mut result = string_nodes.join("") + "#\n";
            let x = &string_ribs.join("")[..];
            result.push_str(x);

            Ok(result)
        }
    }

    #[test]
    fn it_create_new() {
        let result: DirectionalGraff<u8, u8> = DirectionalGraff::new(2);

        let example_node: Node<u8, u8> = Node { value: 2, ribs: Vec::new() };
        let mut map: HashMap<usize, Node<u8, u8>> = HashMap::new();
        map.insert(0, example_node);
        let example: DirectionalGraff<u8, u8> = DirectionalGraff { nodes: map };

        assert_eq!(result, example);
    }

    #[test]
    fn add_node_test() {
        let mut result: DirectionalGraff<u8, u8> = DirectionalGraff::new(2);
        result.add_node(0, 5, 4).unwrap();

        let example_rib1: Rib<u8> = Rib { target: 1, value: 5 };
        let example_node1: Node<u8, u8> = Node { value: 2, ribs: vec![example_rib1] };
        let example_node2: Node<u8, u8> = Node { value: 4, ribs: Vec::new() };
        let mut map: HashMap<usize, Node<u8, u8>> = HashMap::new();
        map.insert(0, example_node1);
        map.insert(1, example_node2);
        let example: DirectionalGraff<u8, u8> = DirectionalGraff { nodes: map };
        assert_eq!(result, example);
    }

    #[test]
    fn add_rib_test() {
        let mut result: DirectionalGraff<&str, &str> = DirectionalGraff::new("mid");
        result.add_node(0, "to_left", "left").unwrap();
        result.add_node(0, "to_right","right").unwrap();
        result.add_rib(1, 2, "to_right").unwrap();

        let test_rib1 = Rib::new(1, "to_left");
        let test_rib2 = Rib::new(2, "to_right");
        let test_rib3 = Rib::new(2, "to_right");
        let test_node1= Node {
            value: "mid",
            ribs: vec![test_rib1, test_rib2],
        };

        let test_node2 = Node {
            value: "left",
            ribs: vec![test_rib3],
        };
        let test_node3: Node<_, &str> = Node {
            value: "right",
            ribs: vec![],
        };

        let mut map: HashMap<usize, Node<&str, &str>> = HashMap::new();
        map.insert(0, test_node1);
        map.insert(1, test_node2);
        map.insert(2, test_node3);

        let test = DirectionalGraff { nodes: map };

        assert_eq!(result, test);
    }

    #[test]
    fn bfs_test() {
        let mut result: DirectionalGraff<&str, &str> = DirectionalGraff::new("mid");
        result.add_node(0, "to_left", "left").unwrap();
        result.add_node(0, "to_right","right").unwrap();
        result.add_node(1, "to_left_level2", "left_level2").unwrap();
        result.add_node(1, "to_mid_level2", "mid_level2").unwrap();
        result.add_rib(2, 4, "to_mid_level2").unwrap();
        result.add_node(2, "to_roght_level2", "right_level2").unwrap();

        let exit_1 = Box::new(|node: &Node<&str, &str>, _: usize| node.value == "mid_level2");
        let exit_2 = Box::new(|node: &Node<&str, &str>, _: usize| node.value == "right");
        let res_1 = result.bfs(0, exit_1);
        let res_2 = result.bfs(1, exit_2);

        assert_eq!([res_1, res_2], [Ok(Some(4)), Ok(None)]);

    }

    #[test]
    fn remove_rib_test() {
        let mut result: DirectionalGraff<&str, &str> = DirectionalGraff::new("mid");
        result.add_node(0, "to_left", "left").unwrap();
        result.add_node(0, "to_right","right").unwrap();
        result.add_rib(1, 2, "to_right").unwrap();
        result.remove_rib(0, 2).unwrap();

        let test_rib1 = Rib::new(1, "to_left");
        let test_rib3 = Rib::new(2, "to_right");
        let test_node1= Node {
            value: "mid",
            ribs: vec![test_rib1],
        };

        let test_node2 = Node {
            value: "left",
            ribs: vec![test_rib3],
        };
        let test_node3: Node<_, &str> = Node {
            value: "right",
            ribs: vec![],
        };

        let mut map: HashMap<usize, Node<&str, &str>> = HashMap::new();
        map.insert(0, test_node1);
        map.insert(1, test_node2);
        map.insert(2, test_node3);

        let test = DirectionalGraff { nodes: map };

        assert_eq!(result, test);

    }

    #[test]
    fn remove_node_test() {
        let mut result: DirectionalGraff<&str, &str> = DirectionalGraff::new("mid");
        result.add_node(0, "to_left", "left").unwrap();
        result.add_node(0, "to_right","right").unwrap();
        result.add_rib(1, 2, "to_right").unwrap();
        result.remove_node(2).unwrap();

        let test_rib1 = Rib::new(1, "to_left");
        let test_node1= Node {
            value: "mid",
            ribs: vec![test_rib1],
        };

        let test_node2 = Node {
            value: "left",
            ribs: vec![],
        };

        let mut map: HashMap<usize, Node<&str, &str>> = HashMap::new();
        map.insert(0, test_node1);
        map.insert(1, test_node2);

        let test = DirectionalGraff { nodes: map };

        assert_eq!(result, test);
    }

    #[test]
    fn dessireolization_test() {

        let mut result: DirectionalGraff<String, String> = DirectionalGraff::new(String::from("morning"));
        result.add_node(0, String::from("lunch is coming soon"), String::from("Noon")).unwrap();
        result.add_node(1, String::from("go home"), String::from("evnin")).unwrap();
        result.add_node(2, String::from("go to sleep"), String::from("night")).unwrap();
        result.add_node(0, String::from("this day"), String::from("Monday")).unwrap();
        result.add_node(4, String::from("next day"), String::from("Tuesday")).unwrap();
        result.add_rib(3, 5, String::from("next day")).unwrap();

        let mut text = String::new();
        text.push_str("0 morning\n");
        text.push_str("1 Noon\n");
        text.push_str("2 evnin\n");
        text.push_str("3 night\n");
        text.push_str("4 Monday\n");
        text.push_str("5 Tuesday\n");
        text.push_str("#\n");
        text.push_str("0 4 this day\n");
        text.push_str("0 1 lunch is coming soon\n");
        text.push_str("1 2 go home\n");
        text.push_str("2 3 go to sleep\n");
        text.push_str("3 5 next day\n");
        text.push_str("4 5 next day\n");

        let mut test_obj: DirectionalGraff<String, String> = DirectionalGraff::new(String::new());
        test_obj.dessireolization(text).unwrap();

        assert_eq!(result, test_obj);

    }

    #[test]
    fn serialize_test() {
        let mut text = String::new();
        text.push_str("0 morning\n");
        text.push_str("1 Noon\n");
        text.push_str("2 evnin\n");
        text.push_str("3 night\n");
        text.push_str("4 Monday\n");
        text.push_str("5 Tuesday\n");
        text.push_str("#\n");

        let mut text1 = text.clone();

        text1.push_str("0 1 lunch is coming soon\n");
        text1.push_str("0 4 this day\n");
        text1.push_str("1 2 go home\n");
        text1.push_str("2 3 go to sleep\n");
        text1.push_str("3 5 next day\n");
        text1.push_str("4 5 next day\n");

        text.push_str("0 4 this day\n");
        text.push_str("0 1 lunch is coming soon\n");
        text.push_str("1 2 go home\n");
        text.push_str("2 3 go to sleep\n");
        text.push_str("3 5 next day\n");
        text.push_str("4 5 next day\n");

        let mut test_obj: DirectionalGraff<String, String> = DirectionalGraff::new(String::new());
        test_obj.dessireolization(text1.clone()).unwrap();
        let res = test_obj.serialize().unwrap();

        assert_eq!(res, text);
    }

}
