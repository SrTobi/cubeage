/*
use vek::vec::Vec3;
use futures::future::RemoteHandle;
use crate::renderer::{BlockState, TerrainProvider, TerrainChunk};
use crate::model::WorldRef;

struct WorldChunkProvider {
    world: WorldRef
}

impl WorldChunkProvider {
    pub fn new(world: &WorldRef) -> WorldChunkProvider {
        WorldChunkProvider {
            world: world.clone()
        }
    }
}

impl TerrainProvider for WorldChunkProvider {
    fn request_chunk(&self, coords: Vec3<i32>) -> RemoteHandle<TerrainChunk> {
        let world = self.world.clone();
        self.world.eval_const(async move {
            let mut chunk = TerrainChunk::new();
            *//*for ((x, y, z), state) in chunk.blocks.indexed_iter_mut() {
                *state = BlockState { value: if z <= world.height_map[(x, y)] { 1 } else { 0 } }
            }*//*
            chunk
        })
    }
}*/
