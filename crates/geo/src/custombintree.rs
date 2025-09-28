pub struct Node {
    value: i32,
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

pub enum Order {
    ASC,
    DEC,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value,
            left_node: None,
            right_node: None,
        }
    }
    pub fn push(&mut self, value: i32) {
        let target = if value > self.value {
            &mut self.right_node
        } else {
            &mut self.left_node
        };

        match target {
            Some(child) => child.push(value),
            None => {
                *target = Some(Box::new(Node::new(value)));
            }
        }
    }
    pub fn fill(&mut self, values: &Vec<i32>) {
        values.iter().for_each(|x| self.push(*x));
    }
    pub fn min(&self) -> i32 {
        match &self.left_node {
            None => self.value,
            Some(child) => child.min(),
        }
    }
    pub fn max(&self) -> i32 {
        match &self.right_node {
            None => self.value,
            Some(child) => child.max(),
        }
    }

    pub fn get_ordered(&self, order: Order) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();

        match order {
            Order::ASC => self.inorder(self, &mut results),
            Order::DEC => self.reverse_order(self, &mut results),
        }

        results
    }

    fn inorder(&self, next: &Node, list: &mut Vec<i32>) {
        if let Some(ref left) = next.left_node {
            self.inorder(left, list);
        }
        list.push(next.value);
        if let Some(ref right) = next.right_node {
            self.inorder(right, list);
        }
    }
    fn reverse_order(&self, next: &Node, list: &mut Vec<i32>) {
        if let Some(ref right) = next.right_node {
            self.reverse_order(right, list);
        }
        list.push(next.value);
        if let Some(ref left) = next.left_node {
            self.reverse_order(left, list);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::custombintree::{Node, Order};

    #[test]
    fn make_node() {
        let root = Node {
            value: 1,
            left_node: None,
            right_node: None,
        };

        println!("{}", root.value);
    }

    #[test]
    fn start_tree() {
        let root = Node::new(1);
        println!("{}", root.value);
    }

    #[test]
    fn manual_add_bigger_branch() {
        let mut root = Node::new(1);

        root.push(2);

        assert!(root.right_node.is_some_and(|x| x.value == 2));
    }

    #[test]
    fn manual_add_multi_branch() {
        let mut root = Node::new(1);

        root.push(2);
        root.push(0);

        assert!(root.right_node.is_some_and(|x| x.value == 2));
        assert!(root.left_node.is_some_and(|x| x.value == 0));
    }
    #[test]
    fn get_min_max() {
        let mut root = Node::new(2);

        root.push(1);
        root.push(3);
        root.push(0);

        assert_eq!(0, root.min());
        assert_eq!(3, root.max());
    }

    #[test]
    fn get_asc() {
        let mut root = Node::new(2);

        root.push(1);
        root.push(3);
        root.push(0);

        assert_eq!(vec![0, 1, 2, 3], root.get_ordered(Order::ASC));
    }

    #[test]
    fn get_dec() {
        let mut root = Node::new(2);

        root.push(1);
        root.push(3);
        root.push(0);

        assert_eq!(vec![3, 2, 1, 0], root.get_ordered(Order::DEC));
    }

    #[test]
    fn fill_test() {
        let mut root = Node::new(12);
        let data = vec![10, 1, 2, 13, 50];

        root.fill(&data);

        assert_eq!(vec![1, 2, 10, 12, 13, 50], root.get_ordered(Order::ASC));
    }
}
