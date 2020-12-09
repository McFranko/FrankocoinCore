extern crate sha2; 
use sha2::{Digest, Sha224};
use std::convert::TryInto;

/// Creates a merkle tree based on some data represented as bytes
#[derive(Debug)]
pub struct MerkleTree {
    root: [u8; 28],
    layers: Vec<Layer>,
    leafs: Vec<Vec<u8>>,
}

impl MerkleTree {
    pub fn new(leafs: Vec<Vec<u8>>) -> Self {
        let mut layers: Vec<Layer> = Vec::new();
        let first_layer = Layer::from_leafs(&leafs);
        layers.push(first_layer);

        for i in 0.. {
            let last_layer = &layers.last().unwrap().0;
            if last_layer.len() == 1 {
                break;
            }
            let layer = Layer::from_layer(&mut layers[i]);
            layers.push(layer);
        }

        MerkleTree {
            root: layers.last().unwrap().0[0].hash,
            layers,
            leafs
        }
    }

    #[allow(unused_variables)]
    pub fn get_proof(&self, hash: &[u8]) -> MerkleProof {
        todo!()
    }
}

#[derive(Debug)]
struct Layer(Vec<Node>);

impl Layer {
    fn from_layer(layer: &mut Layer) -> Self {
        let mut nodes: Vec<Node> = Vec::new();


        for i in (0..layer.0.len()).step_by(2) {

            // If the last node doesn't have another node to be hashed with, it just carries the
            // node over to the next layer.
            if i + 1 >= layer.0.len() {
                let node = Node::from_node(&layer.0[i], i);
                nodes.push(node);
                break;
            }

            let node = Node::from_nodes(&layer.0[i], &layer.0[i + 1], nodes.len());
            
            layer.0[i].set_parent_index(nodes.len());
            layer.0[i + 1].set_parent_index(nodes.len());

            nodes.push(node)
        }

        Layer(nodes)
    }

    fn from_leafs(leafs: &Vec<Vec<u8>>) -> Self {
        let mut nodes: Vec<Node> = Vec::new();

        for i in (0..leafs.len()).step_by(2) {
            let left_leaf = &leafs[i];
            let right_leaf = {
                if i + 1 >= leafs.len() {
                    left_leaf
                } else {
                    &leafs[i + 1]
                }
            };

            let node = Node::from_leafs(left_leaf, right_leaf, i / 2);
            nodes.push(node);
        }

        Layer(nodes)
    }
}

#[derive(Debug)]
struct Node {
    hash: [u8; 28],
    index: usize,
    left_child_index: usize,
    right_child_index: Option<usize>,
    parent_index: Option<usize>,
}

impl Node {
    fn from_node(node: &Node, index: usize) -> Self {
        Node {
            hash: node.hash,
            index,
            left_child_index: node.index,
            right_child_index: None,
            parent_index: None,
        }
    }

    fn from_nodes(left_node: &Node, right_node: &Node, index: usize) -> Self {
        let to_hash = [left_node.hash, right_node.hash].concat();
        let hash = Sha224::digest(&to_hash).try_into().unwrap();

        Node {
            hash,
            index,
            left_child_index: left_node.index,
            right_child_index: Some(right_node.index),
            parent_index: None,
        }
    }

    fn from_leafs(left_leaf: &[u8], right_leaf: &[u8], index: usize) -> Self {
        let to_hash = [left_leaf, right_leaf].concat();
        let hash = Sha224::digest(&to_hash).try_into().unwrap();

        Node {
            hash,
            index,
            left_child_index: index,
            right_child_index: Some(index + 1),
            parent_index: None,
        }
    }

    fn set_parent_index(&mut self, index: usize) {
        self.parent_index = Some(index);
    }
}

pub struct MerkleProof;
