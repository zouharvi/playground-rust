use std::collections::HashMap;

#[derive(Eq, PartialEq)]
enum SearchResult {
    NotFound,
    Word,
    PrefixOnly,
}

#[derive(Debug)]
struct Trie {
    children: HashMap<char, (Trie, bool)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: String) {
        self.insert_slice(word.as_str());
    }

    fn insert_slice(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        let child = self
            .children
            .entry(word.chars().next().unwrap())
            .or_insert((Trie::new(), word.len() == 1));
        if word.len() == 1 {
            child.1 = true;
        }

        child.0.insert_slice(&word[1..]);
    }

    fn search(&self, word: String) -> bool {
        self.search_slice(word.as_str()) == SearchResult::Word
    }

    fn search_slice(&self, word: &str) -> SearchResult {
        let cur_char = word.chars().next().unwrap();
        if !self.children.contains_key(&cur_char) {
            return SearchResult::NotFound;
        }

        if word.len() == 1 {
            if self.children.get(&cur_char).unwrap().1 {
                return SearchResult::Word;
            } else {
                return SearchResult::PrefixOnly;
            }
        }
        self.children
            .get(&cur_char)
            .unwrap()
            .0
            .search_slice(&word[1..])
    }

    fn starts_with(&self, word: String) -> bool {
        let result = self.search_slice(word.as_str());
        result == SearchResult::Word || result == SearchResult::PrefixOnly
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 */

pub fn main() {
    let mut trie: Trie = Trie::new();
    trie.insert("apple".to_string());
    // println!("{:?}", trie);
    println!("{}", trie.search("apple".to_string())); // return True
    println!("{}", trie.search("app".to_string())); // return False
    println!("{}", trie.starts_with("app".to_string())); // return True
    trie.insert("app".to_string());
    println!("{}", trie.search("app".to_string())); // return True
}
