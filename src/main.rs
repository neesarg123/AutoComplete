use std::collections::HashMap;
use std::io;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Read;

// Defining a TrieNode struct
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
    fn search_partial(&self, partial_word: &str) -> Option<&TrieNode> {
        let mut cur = &self.root;

        for c in partial_word.chars() {
           if !cur.children.contains_key(&c) {
                return None;
           } 
           cur = cur.children.get(&c).unwrap();  // getting value from Option<char, TrieNode>
        }
        return Some(cur);
    }

    // get whole word from partial word
    // start at root
    // if char in children, then look at children's children
    // for each, repeat above step until depth limit is reached or is_end_of_word is True
    fn dfs(&self) {
        self.dfs_helper(&self.root);
    }

    fn dfs_helper(&self, trieNode: &TrieNode) {
        for (key, t_node) in trieNode.children.iter() {
            println!("key: {}", key);
            self.dfs_helper(t_node); 
        }
    } 
}


fn main() {
    println!("Hello, User!");
    
    let mut auto_complete_trie = Trie::new();

    // insert any words from serialized file "serialized_trie.json" into auto_complete_trie
    let mut file = File::open("serialized_trie.json").expect("File not found.");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read file.");

    auto_complete_trie = serde_json::from_str(&mut file_contents).expect("There was a problem deserializing into auto_complete_trie.");

    auto_complete_trie.dfs();

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

        // print words down that branch
        let t_node = auto_complete_trie.search_partial(&word);
        println!("t_node: {:?}", t_node); 

        match t_node {
            Some(node) => {
                auto_complete_trie.dfs_helper(node);
            },
            None => {
                println!("No partial matches found");
            }
        }
        // add word to auto_complete_trie
        auto_complete_trie.insert(&word);

        println!("{:?}", auto_complete_trie);


        // serialize the trie
        let serialized = serde_json::to_string(&auto_complete_trie).unwrap();
        println!("Serialized: {}", serialized);
        // write to file
        std::fs::write("serialized_trie.json", serialized).expect("Failed to write to the persistent file storing the trie");

        user_word = String::new();
        println!("Enter word you would like to add to the Trie: ");
        // read line
        io::stdin().read_line(&mut user_word).expect("Could not read input!");
    }
}
