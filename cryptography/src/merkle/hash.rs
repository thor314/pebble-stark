use std::{marker::PhantomData, fmt::Debug};

use ark_ff::PrimeField;
use rs_merkle::{algorithms::Sha256, Hasher};

pub trait Hash<F: PrimeField> {
    type HashedVal: Clone + PartialEq + Debug + Copy;
    fn hash_value(data: F) -> Self::HashedVal;
    fn hash_left_right_child(lchild: Self::HashedVal, rchild: Self::HashedVal) -> Self::HashedVal;
}

#[derive(Clone, Debug)]
pub struct Sha256_ <F: PrimeField> {
    _marker: PhantomData<F>,
}

pub fn usize_slice_to_u8_slice(usizes: &[usize]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for &usize_val in usizes {
        let usize_bytes = usize_val.to_le_bytes();
        bytes.extend_from_slice(&usize_bytes);
    }
    bytes
}

impl <F: PrimeField> Hash<F> for Sha256_<F>{
    type HashedVal = F;

    fn hash_value(data: F) -> Self::HashedVal {
        let vals: Vec<usize> = data.to_string().into() as Vec<usize>;
        F::from_le_bytes_mod_order(&Sha256::hash(usize_slice_to_u8_slice(&vals)))
    }

    fn hash_left_right_child(lchild: Self::HashedVal, rchild: Self::HashedVal) -> Self::HashedVal {
        let mut d_left : Vec<usize> = lchild.to_string().into() as Vec<usize>;
        let mut d_right: Vec<usize> = rchild.to_string().into() as Vec<usize>;

        d_left.append(&mut d_right);
        let hashed_children: [u8; 32] = Sha256::hash(&d_left);
        F::from_le_bytes_mod_order(&hashed_children)
    }
}