use bit_vec::BitVec;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, PartialOrd, Clone)]
pub struct HuffmanTree {
    val: Option<u8>,
    whole_tree_min_val: u8,
    weigths_sum: usize,
    left: Option<Box<HuffmanTree>>,
    right: Option<Box<HuffmanTree>>,
}

impl HuffmanTree {
    pub fn single_node(val: u8, weights_sum: usize) -> Self {
        HuffmanTree {
            val: Some(val),
            whole_tree_min_val: val,
            weigths_sum: weights_sum,
            left: None,
            right: None,
        }
    }

    pub fn merge_right(&mut self, tree: HuffmanTree) -> Self {
        HuffmanTree {
            val: None,
            whole_tree_min_val: min(self.whole_tree_min_val, tree.whole_tree_min_val),
            weigths_sum: self.weigths_sum + tree.weigths_sum,
            left: Some(Box::new((*self).clone())),
            right: Some(Box::new(tree)),
        }
    }

    pub fn extract_mapping(&self) -> HashMap<u8, BitVec> {
        let mut mapping = HashMap::new();
        let mut prev_bits = BitVec::from_elem(1, true);
        self.recursive_extract_mapping(&mut prev_bits, &mut mapping);
        mapping
    }

    fn recursive_extract_mapping(&self, prev_bits: &mut BitVec, mapping: &mut HashMap<u8, BitVec>) {
        if let Some(val) = self.val {
            mapping.insert(val, prev_bits.clone());
        };

        if let Some(t) = &self.left {
            prev_bits.push(false);
            t.recursive_extract_mapping(prev_bits, mapping);
            prev_bits.pop();
        };

        if let Some(t) = &self.right {
            prev_bits.push(true);
            t.recursive_extract_mapping(prev_bits, mapping);
            prev_bits.pop();
        };
    }
}

pub struct HuffmanTreeCreator;

impl HuffmanTreeCreator {
    pub fn get_mappings(weights: &HashMap<u8, u8>) -> (HashMap<u8, BitVec>, HashMap<BitVec, u8>) {
        let on_bits = HuffmanTreeCreator::get_mapping_on_bits(weights);
        let on_bytes = HuffmanTreeCreator::get_mapping_on_bytes(weights);
        (on_bits, on_bytes)
    }

    pub fn get_mapping_on_bits(weights: &HashMap<u8, u8>) -> HashMap<u8, BitVec> {
        let mut trees_seq = (*weights)
            .clone()
            .into_iter()
            .map(|(byte, w)| HuffmanTree::single_node(byte, w as usize))
            .collect::<Vec<HuffmanTree>>();

        let operations_num = trees_seq.len() - 1;
        for _ in 0..operations_num {
            let mut tree_left = HuffmanTreeCreator::get_and_rm_next_elem(&mut trees_seq);
            let tree_right = HuffmanTreeCreator::get_and_rm_next_elem(&mut trees_seq);

            let new_tree = tree_left.merge_right(tree_right);
            trees_seq.push(new_tree);
        }

        trees_seq[0].extract_mapping()
    }

    fn get_and_rm_next_elem(trees_seq: &mut Vec<HuffmanTree>) -> HuffmanTree {
        let mut index = usize::default();
        let mut min_w = usize::MAX;
        let mut min_val = u8::MAX;

        trees_seq.iter().enumerate().for_each(|(i, t)| {
            if min_w > t.weigths_sum || (min_w == t.weigths_sum && min_val > t.whole_tree_min_val) {
                min_w = t.weigths_sum;
                min_val = t.whole_tree_min_val;
                index = i;
            }
        });

        trees_seq.remove(index)
    }

    pub fn get_mapping_on_bytes(weights: &HashMap<u8, u8>) -> HashMap<BitVec, u8> {
        let on_bits = HuffmanTreeCreator::get_mapping_on_bits(weights);
        on_bits.into_iter().map(|(byte, bit)| (bit, byte)).collect()
    }
}
