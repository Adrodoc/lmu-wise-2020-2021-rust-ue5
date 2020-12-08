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
        HuffTree::Leaf { occ, chr }
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
        match self {
            HuffTree::Leaf { occ, .. } => *occ,
            HuffTree::Node { left, right } => left.lettercount() + right.lettercount(),
        }
    }
}

impl Display for HuffTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        fn fmt_prefixed(s: &HuffTree, f: &mut Formatter<'_>, prefix: usize) -> Result {
            match s {
                HuffTree::Leaf { chr, occ } => write!(f, "{}{}: {}", "  ".repeat(prefix), chr, occ),
                HuffTree::Node { left, right } => {
                    write!(f, "{}left:\n", "  ".repeat(prefix))?;
                    fmt_prefixed(left, f, prefix + 1)?;
                    write!(f, "\n{}right:\n", "  ".repeat(prefix))?;
                    fmt_prefixed(right, f, prefix + 1)
                }
            }
        }
        fmt_prefixed(self, f, 0)
    }
}

impl Ord for HuffTree {
    fn cmp(&self, other: &Self) -> Ordering {
        other.lettercount().cmp(&self.lettercount())
    }
}
impl PartialOrd for HuffTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for HuffTree {}
impl PartialEq for HuffTree {
    fn eq(&self, other: &Self) -> bool {
        self.lettercount().eq(&other.lettercount())
    }
}

// Build a Huffmann tree by iteratively combining two minimal elements.
fn huffman(occ: BTreeMap<char, u32>) -> Option<HuffTree> {
    let mut heap = occ
        .into_iter()
        .map(|(chr, occ)| HuffTree::new(chr, occ))
        .collect::<BinaryHeap<_>>();

    loop {
        match (heap.pop(), heap.pop()) {
            (Some(first), Some(second)) => heap.push(first.merge(second)),
            (first, _) => return first,
        }
    }
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
    let frequency = frequency(&mut m.chars());
    huffman(frequency).map(|tree| {
        println!("{}", tree);
        let book = codebook(&tree);
        let bits = m.chars().flat_map(|c| book[&c].clone()).collect();
        (book, bits)
    })
}

fn frequency<T: Ord, I: Iterator<Item = T>>(vec: &mut I) -> BTreeMap<T, u32> {
    vec.fold(BTreeMap::new(), |mut map, element| {
        *map.entry(element).or_default() += 1;
        map
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_test_numbers() {
        // given:
        let numbers = vec![1, 2, 3, 3, 2, 3, 5];

        // when:
        let actual = frequency(&mut numbers.iter());

        // then:
        assert_eq!(actual[&1], 1);
        assert_eq!(actual[&2], 2);
        assert_eq!(actual[&3], 3);
        assert_eq!(actual[&5], 1);
    }

    #[test]
    fn frequency_test_chars() {
        // given:
        let m = "Hello World";

        // when:
        let actual = frequency(&mut m.chars());

        // then:
        assert_eq!(actual[&'H'], 1);
        assert_eq!(actual[&'e'], 1);
        assert_eq!(actual[&'l'], 3);
        assert_eq!(actual[&'o'], 2);
    }
}
