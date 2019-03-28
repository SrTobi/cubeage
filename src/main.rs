#![feature(async_await, await_macro, futures_api)]
extern crate camera_controllers;
extern crate fps_counter;
#[macro_use] extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate piston;
extern crate piston_window;
extern crate libc;
extern crate memmap;
extern crate rustc_serialize;
extern crate shader_version;
extern crate vecmath;
extern crate zip;
extern crate vek;


//pub mod minecraft;
pub mod chunk;
pub mod shader;
pub mod utils;
pub mod model;
pub mod renderer;
pub mod viewmodel;

use std::cmp::max;
use std::f32::consts::PI;
use std::f32::INFINITY;
use std::time::Instant;

//use docopt::Docopt;
use piston::event_loop::{ Events, EventLoop, EventSettings };
//use flate2::read::GzDecoder;
use piston_window::*;
use gfx::traits::Device;
use shader::Renderer;
use vek::vec::Vec3;
//use vek::mat::Mat4;
use piston::input::{ MouseRelativeEvent, PressEvent, UpdateEvent,
    AfterRenderEvent, RenderEvent, ResizeEvent };
use piston::window::WindowSettings;
use std::cell::RefCell;

use utils::cube;
use utils::array;
use array::*;




use std::thread;

use futures::executor::{block_on, ThreadPool, LocalPool};
use futures::task::{SpawnExt, Spawn, LocalSpawnExt};
use futures::future::{self, FutureExt};
use futures::stream::{self, StreamExt};
use std::time::Duration;
use futures::task::LocalWaker;
use futures::Poll;
use futures::future::LocalFutureObj;
use futures::future::FutureObj;

fn main() {
let mut pool = LocalPool::new();
let mut spawner = pool.spawner();

let f = async {
    println!("first");
    3
};

let handle = spawner.spawn_with_handle(f).unwrap();

//drop(handle);

println!("Start run");
pool.run();



    /*block_on(future::lazy(|lw| {
        loop {
            if let Poll::Ready(Some(_)) = pool.poll_next_unpin(lw) {
                // found one more
                println!("done one more");
            } else {
                // either more ready futures (or none at all)
                println!("nothing to do");
                break;
            }
        }
    }));*/
}





/*
static USAGE: &'static str = "
CubeAge, Minecraft made in Rust!

Usage:
    CubeAge [options] <model>

Options:
    -p, --path               Fully qualified path for model folder.
    --mcversion=<version>    Minecraft version [default: 1.8.8].
";

#[derive(RustcDecodable)]
struct Args {
    arg_world: String,
    flag_path: bool,
    flag_mcversion: String,
}*/

pub fn fill_buffer(buffer: &mut Vec<shader::Vertex>, coords: Vec3<i32>, _chunks: [[[&chunk::Chunk; 3]; 3]; 3]) {

    let chunk_xyz = coords.map(|x| x as f32) * 16.0;
    for y in 0..16_usize {
        for z in 0..16_usize {
            for x in 0..16_usize {
                let block_xyz = chunk_xyz + Vec3::new(x, y, z).map(|x| x as f32);
                for face in cube::FaceIterator::new() {
                    // Up, North and South, East and West, Down have different lighting.
                    let light_factor = match face {
                        cube::Up => 1.0,
                        cube::North | cube::South => 0.8,
                        cube::East | cube::West => 0.6,
                        cube::Down => 0.5
                    };

                    let v = face.vertices(block_xyz, Vec3::one()).map(|vertex| {

                        let rgb = [1.0, 1.0, 1.0];


                        shader::Vertex {
                            xyz: vertex.into_array(),
                            //uv: face.direction(),
                            // No clue why the difference of 2 exists.
                            rgb: rgb.map(|x| x * light_factor)
                        }
                    });

                    // Split the clockwise quad into two clockwise triangles.
                    buffer.extend([0,1,2,2,3,0].iter().map(|&i| v[i]));
                }
            }
        }
    }
}


fn _main2() {

    // Automagically pull MC assets
    //minecraft::fetch_assets(&args.flag_mcversion);

    // Automagically expand path if model is located at
    // $MINECRAFT_ROOT/saves/<world_name>
    /*let model = if args.flag_path {
        PathBuf::from(&args.arg_world)
    } else {
        let mut mc_path = minecraft::vanilla_root_path();
        mc_path.push("saves");
        mc_path.push(args.arg_world);
        mc_path
    };*/

    //let file_name = PathBuf::from(model.join("level.dat"));
    //let level_reader = GzDecoder::new(File::open(file_name).unwrap()).unwrap();
    //let level = minecraft::nbt::Nbt::from_reader(level_reader).unwrap();
    //println!("{:?}", level);
    let player_pos: [f32; 3] = [0.0, 18.0, 0.0];
    let player_chunk = [0, 0];
    //let player_rot = level["Data"]["Player"]["Rotation"]
    //    .as_float_list().unwrap();
    let player_yaw = 0.0; // player_rot[0];
    let player_pitch = 0.0; //player_rot[1];

    //let regions = player_chunk.map(|x| x >> 5);
    //let region_file = model.join(
    //        format!("region/r.{}.{}.mca", regions[0], regions[1])
    //    );
    //let region = minecraft::region::Region::open(&region_file).unwrap();

    let loading_title = format!(
            "CubeAge loading..."
        );

    let mut window: PistonWindow = WindowSettings::new(
            loading_title,
            Size { width: 854, height: 480 })
            .fullscreen(false)
            .exit_on_esc(true)
            .samples(0)
            .vsync(false)
            .opengl(shader_version::opengl::OpenGL::V3_2)
            .build()
            .unwrap();

    //let Size { width: w, height: h } = window.size();


    //let assets = Path::new("./assets");

    // Load biomes.
    //let biomes = Biomes::load(&assets);

    // Load block state definitions and models.
    //et block_states = BlockStates::load(&assets, &mut factory);
	let encoder = window.factory.create_command_buffer().into();

    let mut renderer = Renderer::new(&mut window.factory, encoder);

    let mut chunk_manager = chunk::ChunkManager::new();

    println!("Started loading chunks...");
    let c_bases = player_chunk.map(|x| max(0, (x & 0x1f) - 8) as u8);
    for cz in c_bases[1]..c_bases[1] + 16 {
        for cx in c_bases[0]..c_bases[0] + 16 {
            chunk_manager.add_chunk_column(cx as i32, cz as i32, chunk::ChunkColumn {
                chunks: vec!(chunk::Chunk { blocks: [[[chunk::BlockState { value: 0 }; chunk::CHUNK_SIZE]; chunk::CHUNK_SIZE]; chunk::CHUNK_SIZE] }),
                buffers: Array::from_fn(|_| RefCell::new(None))
            })
        }
    }
    println!("Finished loading chunks.");

    //renderer.set_projection(Mat4::from(projection_mat));

    let mut first_person_settings = camera_controllers::FirstPersonSettings::keyboard_wasd();
    first_person_settings.mouse_sensitivity_horizontal = 0.5;
    first_person_settings.mouse_sensitivity_vertical = 0.5;
    first_person_settings.speed_horizontal = 8.0;
    first_person_settings.speed_vertical = 4.0;
    let mut first_person = camera_controllers::FirstPerson::new(
        player_pos,
        first_person_settings
    );
    first_person.yaw = PI - player_yaw / 180.0 * PI;
    first_person.pitch = player_pitch / 180.0 * PI;

    let mut fps_counter = fps_counter::FPSCounter::new();

    let mut pending_chunks = vec![];
    chunk_manager.each_chunk_and_neighbors(
        |coords, buffer, chunks| {
            pending_chunks.push((coords, buffer, chunks));
        }
    );

    let mut capture_cursor = false;
    println!("Press C to capture mouse");

    let mut staging_buffer = vec![];
    let mut events = Events::new(EventSettings::new().ups(120).max_fps(10_000));
    while let Some(e) = events.next(&mut window) {
        use piston::input::Button::Keyboard;
        use piston::input::Key;

        if let Some(_) = e.render_args() {
            // Apply the same y/z camera offset vanilla minecraft has.
            let mut camera = first_person.camera(0.0);
            camera.position[1] += 1.62;
            let mut xz_forward = Vec3::from(camera.forward);
            xz_forward[1] = 0.0;
            xz_forward.normalize();
            camera.position = (Vec3::from(camera.position) + xz_forward * 0.1).into_array();

            let target = shader::RenderOutput::new(&window.output_color, &window.output_stencil);


            let projection_mat = camera_controllers::CameraPerspective {
                fov: 70.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: {
                    let Size { width: w, height: h } = window.size();
                    (w as f32) / (h as f32)
                }
            }.projection();

            let view_mat = camera.orthogonal();
            //renderer.set_view(view_mat);
            renderer.clear(&target);
            let mut num_chunks: usize = 0;
            let mut num_sorted_chunks: usize = 0;
            let mut num_total_chunks: usize = 0;
            let start_time = Instant::now();
            chunk_manager.each_chunk(|cx, cy, cz, _, buffer| {
                match buffer.borrow_mut().as_mut() {
                    Some(buffer) => {
                        num_total_chunks += 1;

                        let inf = INFINITY;
                        let mut bb_min = [inf, inf, inf];
                        let mut bb_max = [-inf, -inf, -inf];
                        let xyz = Vec3::new(cx, cy, cz).map(|x| x as f32 * 16.0);
                        for &dx in [0.0, 16.0].iter() {
                            for &dy in [0.0, 16.0].iter() {
                                for &dz in [0.0, 16.0].iter() {
                                    use vecmath::col_mat4_transform;

                                    let v = xyz + Vec3::new(dx, dy, dz);
                                    let xyzw = col_mat4_transform(view_mat, [v[0], v[1], v[2], 1.0]);
                                    let v = col_mat4_transform(projection_mat, xyzw);
                                    let xyz = vecmath::vec3_scale([v[0], v[1], v[2]], 1.0 / v[3]);
                                    bb_min = Array::from_fn(|i| bb_min[i].min(xyz[i]));
                                    bb_max = Array::from_fn(|i| bb_max[i].max(xyz[i]));
                                }
                            }
                        }

                        let cull_bits: [bool; 3] = Array::from_fn(|i| {
                            let (min, max) = (bb_min[i], bb_max[i]);
                            min.signum() == max.signum()
                                && min.abs().min(max.abs()) >= 1.0
                        });

                        if !cull_bits.iter().any(|&cull| cull) {
                            renderer.render(&target, projection_mat, view_mat, buffer.clone());
                            num_chunks += 1;

                            if bb_min[0] < 0.0 && bb_max[0] > 0.0
                            || bb_min[1] < 0.0 && bb_max[1] > 0.0 {
                                num_sorted_chunks += 1;
                            }
                        }
                    }
                    None => {}
                }
            });
            let end_duration = start_time.elapsed();
            renderer.flush(&mut window.device);
            let frame_end_duration = start_time.elapsed();

            let fps = fps_counter.tick();
            let title = format!(
                    "CubeAge sort={} render={} total={} in {:.2}ms+{:.2}ms @ {}FPS",
                    num_sorted_chunks,
                    num_chunks,
                    num_total_chunks,
                    end_duration.as_secs() as f64 + end_duration.subsec_nanos() as f64 / 1000_000_000.0,
                    frame_end_duration.as_secs() as f64 + frame_end_duration.subsec_nanos() as f64 / 1000_000_000.0,
                    fps, //model.file_name().unwrap().to_str().unwrap()
                );
            window.set_title(title);
        }

        if let Some(_) = e.after_render_args() {
            window.device.cleanup();
        }

        if let Some(_) = e.update_args() {
            use std::i32;
            // HACK(eddyb) find the closest chunk to the player.
            // The pending vector should be sorted instead.
            let pp = first_person.position.map(|x| (x / 16.0).floor() as i32);
            let closest = pending_chunks.iter().enumerate().fold(
                (None, i32::max_value()),
                |(best_i, best_dist), (i, &(cc, _, _))| {
                    let xyz = [cc[0] - pp[0], cc[1] - pp[1], cc[2] - pp[2]]
                        .map(|x| x * x);
                    let dist = xyz[0] + xyz[1] + xyz[2];
                    if dist < best_dist {
                        (Some(i), dist)
                    } else {
                        (best_i, best_dist)
                    }
                }
            ).0;
            let pending = closest.and_then(|i| {
                // Vec swap_remove doesn't return Option anymore
                match pending_chunks.len() {
                    0 => None,
                    _ => Some(pending_chunks.swap_remove(i))
                }
            });
            match pending {
                Some((coords, buffer, chunks)) => {
                    fill_buffer(
                        &mut staging_buffer,
                        Vec3::from(coords), chunks
                    );

                    *buffer.borrow_mut() = Some(
                        renderer.create_buffer(&mut window.factory, &staging_buffer[..])
                    );
                    staging_buffer.clear();

                    if pending_chunks.is_empty() {
                        println!("Finished filling chunk vertex buffers.");
                    }
                }
                None => {}
            }
        }

        if let Some(_) = e.resize_args() {
            let Size { width, height } = window.size();
            println!("Window size changed to {}, {}", width, height);

            /*let (target_view, depth_view) = create_main_targets(
                (width as u16, height as u16, 1, (0 as gfx::texture::NumSamples).into()));
            renderer.data = shader::pipe::Data {
                vbuf: renderer.data.vbuf,
                transform: vecmath::mat4_id(),
                view: vecmath::mat4_id(),
                //color: (texture_view, sampler),
                out_color: target_view,
                out_depth: depth_view,
            };
            projection_mat = camera_controllers::CameraPerspective {
                fov: 70.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: {
                    (width as f32) / (height as f32)
                }
            }.projection();
            renderer.set_projection(projection_mat);*/
        }

        if let Some(Keyboard(Key::C)) = e.press_args() {
            println!("Turned cursor capture {}",
                if capture_cursor { "off" } else { "on" });
            capture_cursor = !capture_cursor;

            window.set_capture_cursor(capture_cursor);
        }

        if let Some(_) = e.mouse_relative_args() {
            if !capture_cursor {
                // Don't send the mouse event to the FPS controller.
                continue;
            }
        }

        first_person.event(&e);
    }
}