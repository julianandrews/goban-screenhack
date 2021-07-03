use crate::sgf_parse::{go, SgfNode};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error;
use std::ptr;

// Self referential struct using a raw pointer to keep track of the current node.
pub struct SgfWalker {
    sgfs: std::pin::Pin<Box<Vec<SgfNode<go::Prop>>>>,
    node_ptr: ptr::NonNull<SgfNode<go::Prop>>,
}

impl SgfWalker {
    pub fn new(sgfs: Vec<SgfNode<go::Prop>>) -> Result<SgfWalker, SgfWalkerError> {
        if sgfs.is_empty() {
            Err(SgfWalkerError::NoSgfs)?;
        }
        let sgfs = Box::pin(sgfs);
        let mut rng = thread_rng();
        let node_ptr = ptr::NonNull::from(sgfs.choose(&mut rng).unwrap()); // sgfs is never empty

        Ok(SgfWalker {
            sgfs: sgfs,
            node_ptr: node_ptr,
        })
    }

    pub fn node(&self) -> &SgfNode<go::Prop> {
        unsafe { self.node_ptr.as_ref() }
    }

    pub fn next_node(&mut self) -> GameState {
        let mut game_state = GameState::Ongoing;
        let next_node = match self.node().children().next() {
            None => {
                game_state = GameState::Ended;
                let mut rng = thread_rng();
                self.sgfs.choose(&mut rng).unwrap() // sgfs is never empty
            }
            Some(node) => node
        };
        self.node_ptr = ptr::NonNull::from(next_node);

        game_state
    }
}

pub enum GameState {
    New,
    Ongoing,
    Ended,
}

#[derive(Debug)]
pub enum SgfWalkerError {
    NoSgfs,
}

impl std::fmt::Display for SgfWalkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SgfWalkerError::NoSgfs => write!(f, "No valid sgf files found in search path."),
        }
    }
}

impl error::Error for SgfWalkerError {}
