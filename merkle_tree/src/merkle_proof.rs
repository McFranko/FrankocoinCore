use crate::{Digest, MerkleTree, Sha224};
use std::convert::TryInto;

/// The proof that a given hash exists in the merkle tree.
#[derive(Debug)]
pub struct MerkleProof(Vec<Layer>);

impl MerkleProof {
    pub fn new(hash: [u8; 28], merkle_tree: &MerkleTree) -> Option<Self> {
        let mut layers: Vec<Layer> = Vec::new();
        let layer = Layer::from_hash(&hash, 0, merkle_tree)?;
        layers.push(layer);

        // Iterates through the layers until the second to last layer, as the last layer of the
        // merkle tree contains just the root.
        for layer_index in 1..merkle_tree.layers.len() - 1 {
            let previous_hash = &layers[layer_index - 1].hash();
            let layer = Layer::from_hash(previous_hash, layer_index, merkle_tree)?;
            layers.push(layer);
        }

        Some(MerkleProof(layers))
    }

    pub fn is_proof(&self, merkle_root: &[u8; 28]) -> bool {
        let layers = &self.0;
        for (i, layer) in layers.iter().take(layers.len() - 2).enumerate() {
            if layers[i + 1].left_hash != layer.hash() {
                return false;
            }
        }


        if &layers.last().unwrap().hash() != merkle_root {
            return false
        }
        
        true
    }
}


#[derive(Debug)]
pub struct Layer {
    left_hash: [u8; 28],
    right_hash: Option<[u8; 28]>,
}

impl Layer {
    /// Returns a merkle proof layer from a hash, the current layer in the tree, and a merkle
    /// tree. It finds the hash that
    fn from_hash(
        hash: &[u8; 28],
        current_layer: usize,
        merkle_tree: &MerkleTree,
    ) -> Option<Self> {
        let node = merkle_tree.layers[current_layer]
            .0
            .iter()
            .find(|node| &node.hash == hash)?;

        let parent_node =
            &merkle_tree.layers[current_layer + 1].0[node.parent_index?];

        let left_hash = merkle_tree.layers[current_layer].0
            [parent_node.left_child_index]
            .hash;

        let right_hash = match parent_node.right_child_index.is_some() {
            true => Some(
                merkle_tree.layers[current_layer].0
                    [parent_node.right_child_index?]
                    .hash,
            ),
            false => None,
        };

        Some(Self {
            left_hash,
            right_hash,
        })
    }

    /// Hashes the left and right hash.
    ///
    /// If a layer in a MerkleTree has an odd number of nodes, it will carry the last node in the
    /// layer over to the next layer. Because of this, if right_hash is None, this function will
    /// just return left_hash, mimicing this behaviour.
    fn hash(&self) -> [u8; 28] {
        match self.right_hash.is_some() {
            true => {
                let to_hash = [self.left_hash, self.right_hash.unwrap()].concat();
                Sha224::digest(&to_hash).try_into().unwrap()
            }
            false => {
                self.left_hash
            }
        }
    }
}

