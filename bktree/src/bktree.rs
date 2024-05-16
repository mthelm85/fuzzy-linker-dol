use std::collections::HashMap;
use crate::levenshtein::levenshtein_distance;

pub struct BKTree {
    root: Option<Node>,
}

struct Node {
    value: String,
    children: HashMap<u32, Node>,
    row_data: Vec<String>,
}

impl BKTree {
    pub fn new() -> Self {
        BKTree { root: None }
    }

    pub fn insert(&mut self, value: String, row_data: Vec<String>) {
        if let Some(root) = &mut self.root {
            root.insert(value, row_data);
        } else {
            self.root = Some(Node::new(value, row_data));
        }
    }

    pub fn search(&self, value: &str, tolerance: u32) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        if let Some(root) = &self.root {
            root.search(value, tolerance, &mut results);
        }
        results
    }
}

impl Node {
    fn new(value: String, row_data: Vec<String>) -> Self {
        Node {
            value,
            row_data,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, value: String, row_data: Vec<String>) {
        let distance = levenshtein_distance(&self.value, &value);
        if let Some(child) = self.children.get_mut(&distance) {
            child.insert(value, row_data);
        } else {
            self.children.insert(distance, Node::new(value, row_data));
        }
    }

    fn search<'a>(&'a self, value: &str, tolerance: u32, results: &mut Vec<Vec<String>>) {
        let distance = levenshtein_distance(&self.value, value);
    
        if distance <= tolerance {
            results.push(self.row_data.clone());
        }
    
        let min_distance = distance.saturating_sub(tolerance);
        let max_distance = distance.saturating_add(tolerance);
    
        for (_, child) in &self.children {
            if distance >= min_distance && distance <= max_distance {
                child.search(value, tolerance, results);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bktree() {
        let mut tree = BKTree::new();

        tree.insert("hello".to_string(), vec!["row1colA".to_string(), "row1colB".to_string()]);
        tree.insert("world".to_string(), vec!["row2colA".to_string(), "row2colB".to_string()]);
        tree.insert("rust".to_string(), vec!["row3colA".to_string(), "row3colB".to_string()]);

        let results = tree.search("hello", 0);
        assert_eq!(results[0], vec!["row1colA", "row1colB"]);

        let results = tree.search("world", 0);
        assert_eq!(results[0], vec!["row2colA", "row2colB"]);

        let results = tree.search("rust", 0);
        assert_eq!(results[0], vec!["row3colA", "row3colB"]);

        let results = tree.search("rest", 1);
        assert!(results.iter().any(|row_data| row_data == &vec!["row3colA", "row3colB"]));
    }
}