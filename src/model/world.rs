/*
use noise::{NoiseFn, Perlin};
use ndarray::prelude::*;
use ndarray::ArcArray;
use futures::executor::LocalPool;
use vek::vec::Vec3;
use futures::future::RemoteHandle;
use std::sync::Arc;
use crate::renderer::{TerrainProvider, TerrainChunk, BlockState};
use futures::task::SpawnExt;
use futures::executor::ThreadPool;
use futures::future::FutureObj;
use futures::future;
use futures::Future;
use std::sync::RwLock;
use futures::stream::FuturesUnordered;
use futures::executor::block_on_stream;
use futures::Poll;
use futures::executor::block_on;
use futures::stream::StreamExt;

pub type HeightMap = Array2<usize>;
pub type WorldRef = Arc<World>;

pub struct World {
    pub height_map: HeightMap,
    const_tasks: FuturesUnordered<FutureObj<'static, ()>>,
    mut_tasks: FuturesUnordered<FutureObj<'static, ()>>,
}
trait AssertSync: Send {}
impl AssertSync for World {}



pub struct WorldRunner {
    world: WorldRef,
    mut_pool: LocalPool
}

impl WorldRunner {
    pub fn run(&mut self) {
        WorldRunner::run_futures(&mut self.world.const_tasks);
        WorldRunner::run_futures(&mut self.world.const_tasks);
    }

    fn run_futures(futures: &mut FuturesUnordered<FutureObj<'static, ()>>) {
        block_on(future::lazy(|lw| {
            loop {
                if let Poll::Ready(None) = futures.poll_next_unpin(lw) {
                    return;
                }
            }
        }));
    }
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        *//*let const_pool = ThreadPool::builder()
            .create()
            .expect("Failed to create constant thread-pool for World");*//*


        World {
            height_map: Array2::from_elem((width, height), 2),
            const_tasks: FuturesUnordered::new(),
            mut_tasks: FuturesUnordered::new(),
        }
    }

    pub fn map_width(&self) -> usize {
        self.height_map.raw_dim()[0]
    }

    pub fn map_height(&self) -> usize {
        self.height_map.raw_dim()[1]
    }

    pub fn generate(&mut self) {
        let perlin = Perlin::new();

        for ((x, y), height) in self.height_map.indexed_iter_mut() {
            *height = perlin.get([x as f64, y as f64]) as usize;
        }
    }

    pub fn spawn_const<Fut>(&self, f: Fut)
        where
            Fut: Future<Output = ()> + Send + 'static {
        self.const_tasks
            .spawn(f)
            .expect("Failed to spawn const job")
    }

    pub fn eval_const<Fut>(&self, f: Fut) -> RemoteHandle<Fut::Output>
        where
            Fut: Future + Send + 'static,
            Fut::Output: Send {
        self.const_tasks
            .spawn_with_handle(f)
            .expect("Failed to spawn const eval job")
    }

    *//*pub fn run_mut<Fut>(&self, f: Fut)
        where
            Fut: Future<Output = ()> + Send + 'static {
        self.mut_pool
            .spawner()
            .spawn(f)
            .expect("Failed to spawn mut job")
    }*//*
}*/
