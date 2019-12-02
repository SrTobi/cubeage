/*
use vek::vec::Vec3;
use futures::future::FutureObj;
use futures::future::RemoteHandle;
use ndarray::Array3;
use std::sync::Arc;


pub trait TerrainProvider {
    fn request_chunk(&self, coords: Vec3<i32>) -> RemoteHandle<TerrainChunk>;
}

#[derive(Copy, Clone)]
pub struct BlockState {
    pub value: u16
}

pub const CHUNK_SIZE: usize = 16;


pub struct TerrainChunk {
    pub blocks: Array3<BlockState>
}


impl TerrainChunk {
    pub fn new() -> TerrainChunk {
        TerrainChunk {
            blocks: Array3::from_elem((CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE), BlockState { value: 0 })
        }
    }
}

*/
