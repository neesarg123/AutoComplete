use std::collections::HashMap;
use std::io;

// Defining a TrieNode struct
#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool
}

// Implementing methods for TrieNode
impl TrieNode {
    // Constructor method
    fn new() -> TrieNode {
        TrieNode {children: HashMap::new(), is_end_of_word: false}
    }

}

// Defining a Trie struct (made up of TrieNodes)
#[derive(Debug)]
struct Trie {
    root: TrieNode  // there is no initialization at this point
}

// Implementing methods for Trie
impl Trie {
    // Constructor method
    fn new() -> Trie {
        Trie {
            root: TrieNode::new()
        }
    }

    // insert a word method
    fn insert(&mut self, word: &str) {
        let mut cur = &mut self.root;

        for c in word.chars() {
            cur = cur.children.entry(c).or_insert(TrieNode::new())
        }
        cur.is_end_of_word = true;
    }

    // search for a word method
    fn search(&mut self, word: &str) -> bool {
        let mut cur = &mut self.root;

        for c in word.chars() {
           if !cur.children.contains_key(&c) {
                return false;
           } 
           cur = cur.children.get_mut(&c).unwrap();  // getting value from Option<char, TrieNode>
        }
        return cur.is_end_of_word;
    }

    // search for a partial word method
    fn search_partial(&mut self, partial_word: &str) -> bool {
        let mut cur = &mut self.root;

        for c in partial_word.chars() {
           if !cur.children.contains_key(&c) {
                return false;
           } 
           cur = cur.children.get_mut(&c).unwrap();  // getting value from Option<char, TrieNode>
        }
        return true;
    }
}


fn main() {
    println!("Hello, User!");
    
    let mut auto_complete_trie = Trie::new();

    // need a string to store user's input
    let mut user_word = String::new();

    println!("Enter word you would like to add to the Trie: ");
    // read line
    io::stdin().read_line(&mut user_word).expect("Could not read input!");

    // continously asking user for input
    while !user_word.trim().is_empty() {
        // trim EOL
        let word = user_word.trim();
        println!("You inputted the word: {}", word);
        // add word to auto_complete_trie
        auto_complete_trie.insert(&word);

        println!("{:?}", auto_complete_trie);

        user_word = String::new();
        println!("Enter word you would like to add to the Trie: ");
        // read line
        io::stdin().read_line(&mut user_word).expect("Could not read input!");
    }
}
