use std::collections::HashMap;

struct Node {
    next: HashMap<char, u64>,
    leaf: bool,
}

struct Trie {
    trie: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        Node { next: HashMap::new(), leaf: false }
    }
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            trie: Vec::new(),
        }
    }

    pub fn add(&mut self, word: &str) {
        todo!()
    }

    pub fn exists(&self, word: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn check_exists() {
        let mut trie = Trie::new();

        trie.add("foo");
        
        assert!(trie.exists("foo"));
    }
}