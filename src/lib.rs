use std::collections::BTreeMap;

struct Node {
    next: BTreeMap<char, usize>,
    is_leaf: bool,
}

pub struct Trie {
    trie: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            next: BTreeMap::new(),
            is_leaf: false,
        }
    }
}

impl Trie {
    pub fn new() -> Trie {
        let trie = vec![Node::new()];

        Trie { trie }
    }

    pub fn add(&mut self, word: &str) -> Result<(), String> {
        let mut v = 0;

        for ch in word.chars() {
            v = match self.trie[v].next.get(&ch) {
                Some(idx) => *idx,
                None => {
                    let idx = self.trie.len();

                    self.trie[v].next.insert(ch, idx);
                    self.trie.push(Node::new());

                    idx
                }
            };
        }

        if self.trie[v].is_leaf {
            return Err("Already added".to_string());
        }

        self.trie[v].is_leaf = true;

        Ok(())
    }

    pub fn remove(&mut self, word: &str) -> bool {
        let mut rm = Vec::new();
        let mut v = 0;

        for ch in word.chars() {
            rm.push(v);

            v = match self.trie[v].next.get(&ch) {
                Some(idx) => *idx,
                None => {
                    return false;
                }
            }
        }

        let result = self.trie[v].is_leaf;

        self.trie[v].is_leaf = false;

        result
    }

    pub fn exists(&self, word: &str) -> bool {
        let mut v = 0;

        for ch in word.chars() {
            v = match self.trie[v].next.get(&ch) {
                Some(idx) => *idx,
                None => {
                    return false;
                }
            }
        }

        self.trie[v].is_leaf
    }

    pub fn get_words_with_prefix(&self, mut word: String) -> Vec<String> {
        let mut v = 0;

        for ch in word.chars() {
            v = match self.trie[v].next.get(&ch) {
                Some(idx) => *idx,
                None => {
                    return vec![];
                }
            }
        }

        let mut words = vec![];

        self.dfs(v, &mut word, &mut words);

        words
    }

    fn dfs(&self, v: usize, current_word: &mut String, words: &mut Vec<String>) {
        if self.trie[v].is_leaf {
            words.push(current_word.clone());
        }

        for (ch, idx) in self.trie[v].next.iter() {
            current_word.push(*ch);

            self.dfs(*idx, current_word, words);

            current_word.pop();
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn check_add_ok() -> Result<(), String> {
        let mut trie = Trie::new();

        trie.add("foo")?;

        Ok(())
    }

    #[test]
    pub fn check_add_err() {
        let mut trie = Trie::new();

        trie.add("foo").unwrap();

        assert_eq!(trie.add("foo"), Err("Already added".to_string()));
    }

    #[test]
    pub fn check_remove() {
        let mut trie = Trie::new();

        trie.add("foo").unwrap();

        assert!(trie.remove("foo"));
    }

    #[test]
    pub fn check_exists() {
        let mut trie = Trie::new();

        trie.add("foo").unwrap();

        assert!(trie.exists("foo"));
    }

    #[test]
    pub fn starts_with() {
        let mut trie = Trie::new();

        trie.add("abc").unwrap();
        trie.add("bcd").unwrap();
        trie.add("cde").unwrap();

        trie.add("abcd").unwrap();
        trie.add("abcde").unwrap();
        trie.add("abcx").unwrap();
        trie.add("abczzz").unwrap();

        let result = trie.get_words_with_prefix("abcd".to_string());

        assert_eq!(vec!["abcd", "abcde"], result);
    }
}
