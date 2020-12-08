use std::clone::Clone;
use std::cmp::{Ord, Ordering};
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::fmt::{Debug, Display, Formatter, Result};

type BitVec = Vec<bool>;
fn bitvec_str(bv: &BitVec) -> String {
    bv.iter().map(|&b| if b { "1" } else { "0" }).collect()
}

enum HuffTree {
    Leaf {
        occ: u32,
        chr: char,
    },
    Node {
        left: Box<HuffTree>,
        right: Box<HuffTree>,
    },
}

impl HuffTree {
    fn new(chr: char, occ: u32) -> HuffTree {
        HuffTree::Leaf { occ, chr: chr }
    }
    fn merge(self, other: HuffTree) -> HuffTree {
        HuffTree::Node {
            left: Box::new(self),
            right: Box::new(other),
        }
    }
    fn chars(&self) -> String {
        match self {
            HuffTree::Node { left, right, .. } => left.chars() + &right.chars(),
            HuffTree::Leaf { chr, .. } => chr.to_string(),
        }
    }
    fn lettercount(&self) -> u32 {
        unimplemented!();
    }
}

// Build a Huffmann tree by iteratively combining two minimal elements.
fn huffman(occ: BTreeMap<char, u32>) -> Option<HuffTree> {
    unimplemented!();
}

type Codebook = BTreeMap<char, BitVec>;

// Get a mapping from character to bit vector from the Huffman tree
fn codebook(huff: &HuffTree) -> Codebook {
    fn traverse(huff: &HuffTree, mut bv: BitVec) -> Codebook {
        match huff {
            HuffTree::Leaf { chr, .. } => {
                let mut btm = BTreeMap::new();
                btm.insert(chr.clone(), bv);
                btm
            }
            HuffTree::Node { left, right, .. } => {
                let mut br = bv.clone();
                br.push(true); // bit-vector right
                bv.push(false); // bit-vector left
                let mut btm = traverse(left, bv);
                btm.append(&mut traverse(right, br));
                btm
            }
        }
    }
    traverse(huff, BitVec::new())
}

// Given a message m, encode returns the Huffman encoded message.
fn encode(m: &str) -> Option<(Codebook, BitVec)> {
    let charvec: Vec<char> = m.chars().collect();
    unimplemented!();
}

fn main() {
    let examples = vec!["BACADAEAFABBAAAGAH", "aardvarks ate apples around aachen"];
    for input in examples {
        if let Some((cb, cs)) = encode(input) {
            for (chr, bitvec) in &cb {
                println!("{}: {}", chr, bitvec_str(bitvec));
            }
            println!("String: {}\n", bitvec_str(&cs));
        }
    }
}
