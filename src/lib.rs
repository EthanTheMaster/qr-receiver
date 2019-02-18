extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use std::collections::HashSet;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

//Rng State using Xor Shift PRNG
#[wasm_bindgen]
pub struct XorShift {
    state: u32
}

impl XorShift {
    pub fn from_seed(seed: u32) -> XorShift {
        XorShift {state: seed}
    }

    pub fn rand(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;

        self.state = x;
        return x;
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Block {
    indices: Vec<u32>,
    xor_idx: u32,
    xor_content: u8,
}

#[wasm_bindgen]
pub struct QrReceiver {
    data: Vec<u8>,
    decoded_blocks: HashSet<u32>,
    //Use Option as a "hack" to take ownership of the vec from the struct
    pending_blocks: Option<Vec<Block>>,
} 

#[wasm_bindgen]
impl QrReceiver {
    pub fn new() -> QrReceiver {
        QrReceiver {
            data: Vec::new(),
            decoded_blocks: HashSet::new(),
            pending_blocks: Some(Vec::new()),
        }
    }

    pub fn process(&mut self, data: Vec<u8>) {
        if data.len() % 20 == 0 {
            for packet in data.chunks(20) {
                //Decode total size
                let mut total_size: u32 = 0;
                total_size |= (packet[0] as u32) << 24;
                total_size |= (packet[1] as u32) << 16;
                total_size |= (packet[2] as u32) << 8;
                total_size |= (packet[3] as u32);

                //Decode degree
                let mut degree: u32 = 0;
                degree |= (packet[4] as u32) << 24;
                degree |= (packet[5] as u32) << 16;
                degree |= (packet[6] as u32) << 8;
                degree |= (packet[7] as u32);

                //Decode rng_seed
                let mut rng_seed: u32 = 0;
                rng_seed |= (packet[8] as u32) << 24;
                rng_seed |= (packet[9] as u32) << 16;
                rng_seed |= (packet[10] as u32) << 8;
                rng_seed |= (packet[11] as u32);

                let mut rng = XorShift::from_seed(rng_seed);

                //Decode block with xored indices
                let mut xor_block_indices: u32 = 0;
                xor_block_indices |= (packet[12] as u32) << 24;
                xor_block_indices |= (packet[13] as u32) << 16;
                xor_block_indices |= (packet[14] as u32) << 8;
                xor_block_indices |= (packet[15] as u32);

                //Decode the xored content
                let mut xor_block_content = packet[16];

                //Decode parity to verify data integrity
                let mut parity_bytes: u32 = 0;
                parity_bytes |= (packet[17] as u32) << 16;
                parity_bytes |= (packet[18] as u32) << 8;
                parity_bytes |= (packet[19] as u32);

                //Verify transmission by comparing parity
                let mut parity_rng = XorShift::from_seed(rng_seed);
                let mut parity_gen = parity_rng.rand();
                parity_rng = XorShift::from_seed(parity_gen ^ total_size);
                parity_gen = parity_rng.rand();
                parity_rng = XorShift::from_seed(parity_gen ^ degree);
                parity_gen = parity_rng.rand();
                parity_rng = XorShift::from_seed(parity_gen ^ xor_block_indices);
                parity_gen = parity_rng.rand();
                parity_rng = XorShift::from_seed(parity_gen ^ (xor_block_content as u32));
                parity_gen = parity_rng.rand();

                if (parity_gen & 0xFFFFFF) != parity_bytes {
                    //Parity check failed
                    continue;
                }

                if self.data.len() == 0 {
                    //Initiate the data buffer
                    self.data = vec![0; total_size as usize];
                }

                //Block is already in decoded state
                if degree == 1 {
                    self.decoded_blocks.insert(xor_block_indices);
                    self.data[xor_block_indices as usize] = xor_block_content;
                } else {
                    //Attempt to partially decode the block
                    let mut unsolved_idx: Vec<u32> = Vec::new();
                    for _ in 0..degree {
                        let idx = rng.rand() % (self.data.len() as u32);
                        if self.decoded_blocks.contains(&idx) {
                            xor_block_indices ^= idx;
                            xor_block_content ^= self.data[idx as usize];
                        } else {
                            unsolved_idx.push(idx);
                        }
                    }

                    self.pending_blocks.as_mut().unwrap().push(Block {
                        indices: unsolved_idx,
                        xor_idx: xor_block_indices,
                        xor_content: xor_block_content
                    });
                }

                //Update Pending List
                let new_list: Vec<Block> = self.pending_blocks.take().unwrap().into_iter().filter_map(|block| {
                    let mut new_xor_idx = block.xor_idx;
                    let mut new_xor_content = block.xor_content;
                    let new_indices: Vec<u32> = block.indices.into_iter().filter(|i| {
                        if self.decoded_blocks.contains(&i) {
                            new_xor_idx ^= i;
                            new_xor_content ^= self.data[*i as usize];
                            return false;
                        } else {
                            return true;
                        }
                    }).collect();

                    if new_indices.len() == 0 {
                        return None;
                    } else if new_indices.len() == 1 {
                        self.decoded_blocks.insert(new_xor_idx);
                        self.data[new_xor_idx as usize] = new_xor_content;
                        return None;
                    } else {
                        return Some(Block {
                            indices: new_indices,
                            xor_idx: new_xor_idx,
                            xor_content: new_xor_content,
                        });
                    }
                }).collect();

                self.pending_blocks = Some(new_list);
            }
        }
    }

    pub fn get_progress_percentage(&self) -> f64 {
        if self.data.len() == 0 {
            return 0.0;
        }

        return (self.decoded_blocks.len() as f64) / (self.data.len() as f64);
    }

    pub fn get_finished_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn has_completed_download(&self) -> bool {
        self.data.len() != 0 && self.decoded_blocks.len() >= self.data.len()
    }

    pub fn get_num_pending_blocks(&self) -> u32 {
        self.pending_blocks.as_ref().unwrap().len() as u32
    }
}