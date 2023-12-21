use ark_ff::PrimeField;
use super::hash::Hash;


//One stop checking trait that would import the tree content of a Valid Merkle Tree
pub trait checkForValidMerkleTree: Default {
    type Node: PartialEq + Eq + Clone + Sync + Send;
    type Value: Sync + Send;

    fn convert_to_node(leaf: &Self::Value) -> Self::Node;

    fn hash_leaves(unhashed_leaves: &[Self::Value]) -> Vec<Self::Node> {
        
    }

}