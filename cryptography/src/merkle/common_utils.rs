
use super::tree::checkForValidMerkleTree;

pub fn parent_idx (node_idx: usize) -> usize {
    if node_idx % 2 == 0{
        (node_idx - 1) / 2
    }
    else {
        node_idx / 2
    }
}

pub fn sibling_idx (node_idx: usize) -> usize {
    if node_idx % 2 == 0 {
        node_idx - 1
    }
    else {
        node_idx + 1
    }
}

pub fn fill_untill_pow_two <T:Clone>(values: &mut Vec<T>) -> Vec<T> {
    while !is_power_of_two(values.len()){
        values.push(values[values.len() - 1].clone())
    }

    values.to_vec() 
}

pub fn is_power_of_two(x: usize) -> bool {
     (x != 0) && ((x & (x-1)) == 0) 
}