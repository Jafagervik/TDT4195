extern crate nalgebra_glm as glm;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr};

// New import for Exercise 3
mod mesh;
mod scene_graph;
mod toolbox;

mod shader;
mod util;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;
use scene_graph::SceneNode;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

fn read_triangles_from_file() -> Result<Vec<f32>, ()> {
    // Takes in an arbitraray amount of trinagles from a file
    let mut vertices: Vec<f32>;
    match File::open(".\\src\\triangles.txt") {
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable
            file.read_to_string(&mut content).unwrap();

            vertices = content
                .split(" ")
                .map(|x| x.parse::<f32>().unwrap())
                .collect();
            println!("{}", content);
            Ok(vertices)
        }
        // Error handling
        Err(error) => {
            println!("Error message: {}", error);
            std::process::exit(1);
        }
    }
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()
// let p = 0 as *const c_void

// == // Modify and complete the function below for the first task
unsafe fn init_vao(
    vertices: &Vec<f32>,
    indices: &Vec<u32>,
    colors: &Vec<f32>,
    normals: &Vec<f32>,
) -> u32 {
    // Returns the ID of the newly instantiated vertex array object upon its creation

    // VAO - way to bind vbo with spesification
    let mut vao: u32 = 0; // Create
    gl::GenVertexArrays(1, &mut vao); // Generate
    gl::BindVertexArray(vao); // Bind

    // VBO - buffer for the vertices/positions
    let mut vbo: u32 = 0;
    gl::GenBuffers(1, &mut vbo); // creates buffer, generates an id for the vertex buffer - stored on vram
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Binding is sort of like creating layers in photoshop
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(&vertices),
        pointer_to_array(&vertices),
        gl::STATIC_DRAW,
    );

    // Vaa = Vertex attrib array
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);
    gl::EnableVertexAttribArray(0);

    // CBO - vbo for the color buffer, RGBA
    let mut cbo: u32 = 1;
    gl::GenBuffers(1, &mut cbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, cbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(&colors),
        pointer_to_array(&colors),
        gl::STATIC_DRAW,
    );

    // 2nd attribute buffer is for colors
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        size_of::<f32>() * 4,
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(1);

    // NBO - vbo for the normal buffer
    let mut nbo: u32 = 1;
    gl::GenBuffers(1, &mut nbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, nbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(&normals),
        pointer_to_array(&normals),
        gl::STATIC_DRAW,
    );

    // 3rd attribute buffer is for normals
    gl::VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        size_of::<f32>() * 3,
        0 as *const c_void,
    );
    gl::EnableVertexAttribArray(2);

    // Index buffer object = connect the dots, multiple usecases for same vertices.
    let mut ibo: u32 = 0;
    gl::GenBuffers(1, &mut ibo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(&indices),
        pointer_to_array(&indices),
        gl::STATIC_DRAW,
    );

    vao
}

unsafe fn draw_scene(
    node: &SceneNode,
    view_projection_matrix: &glm::Mat4,
    shader: &shader::Shader,
) {
    // Check if node is drawable, Set uniforms, Draw
    // Check that we have a vao attached to the node
    // TODO: Set uniforms
    if node.index_count != -1 {
        /*
        trans_loc = shader.get_uniform_location("transformation");
        time_loc = shader.get_uniform_location("time");
        opacity_loc = shader.get_uniform_location("opacity");
        shdr.activate();
        */
        gl::BindVertexArray(node.vao_id);
        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count, // Here we get the amount of indices we need
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    }

    for &child in &node.children {
        draw_scene(&*child, view_projection_matrix, shader);
    }
}

unsafe fn update_node_transformations(node: &mut SceneNode, transformation_so_far: &glm::Mat4) {
    // Construct the correct transformation matrix
    // TODO: Find out what to do here?
    let mut trans: glm::Mat4 = glm::identity();
    trans = glm::translate(&trans, &node.position);
    trans = glm::rotate(&trans, 90.0, &node.rotation);
    trans = glm::scale(&trans, &node.scale);

    // Update the node's transformation matrix
    // Task 3d)
    // TODO: Find out if this is the correct order
    node.current_transformation_matrix = trans * transformation_so_far;

    // Need to find correct location to use
    // gl::UniformMatrix4fv(trans_loc, 1, gl::FALSE, node.current_transformation_matrix.as_ptr());
    // Recurse
    for &child in &node.children {
        update_node_transformations(&mut *child, &node.current_transformation_matrix);
    }
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!(
                "{}: {}",
                util::get_gl_string(gl::VENDOR),
                util::get_gl_string(gl::RENDERER)
            );
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!(
                "GLSL\t: {}",
                util::get_gl_string(gl::SHADING_LANGUAGE_VERSION)
            );
        }
        let terrain_mesh = mesh::Terrain::load(".\\resources\\lunarsurface.obj");
        // task 2a)
        let helicopter_mesh = mesh::Helicopter::load(".\\resources\\helicopter.obj");

        // VAO IDs
        let terrain_vao: u32;
        // task 2a)
        let helicopter_body_vao: u32;
        let helicopter_door_vao: u32;
        let helicopter_main_rotor_vao: u32;
        let helicopter_tail_rotor_vao: u32;

        // == // Set up your VAO here
        unsafe {
            terrain_vao = init_vao(
                &terrain_mesh.vertices,
                &terrain_mesh.indices,
                &terrain_mesh.colors,
                &terrain_mesh.normals,
            );
            helicopter_body_vao = init_vao(
                &helicopter_mesh[0].vertices,
                &helicopter_mesh[0].indices,
                &helicopter_mesh[0].colors,
                &helicopter_mesh[0].normals,
            );
            helicopter_main_rotor_vao = init_vao(
                &helicopter_mesh[1].vertices,
                &helicopter_mesh[1].indices,
                &helicopter_mesh[1].colors,
                &helicopter_mesh[1].normals,
            );
            helicopter_tail_rotor_vao = init_vao(
                &helicopter_mesh[2].vertices,
                &helicopter_mesh[2].indices,
                &helicopter_mesh[2].colors,
                &helicopter_mesh[2].normals,
            );
            helicopter_door_vao = init_vao(
                &helicopter_mesh[3].vertices,
                &helicopter_mesh[3].indices,
                &helicopter_mesh[3].colors,
                &helicopter_mesh[3].normals,
            );
        }
        // Set up scene graph: task 2b)
        let mut root = SceneNode::new();
        let mut helicopter_root = SceneNode::new();
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain_mesh.index_count);
        let mut helicopter_body_node =
            SceneNode::from_vao(helicopter_body_vao, helicopter_mesh[0].index_count);
        let mut helicopter_main_rotor_node =
            SceneNode::from_vao(helicopter_main_rotor_vao, helicopter_mesh[1].index_count);
        let mut helicopter_tail_rotor_node =
            SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter_mesh[2].index_count);
        let mut helicopter_door_node =
            SceneNode::from_vao(helicopter_door_vao, helicopter_mesh[3].index_count);

        // 3b) Reference points
        // FIXME: Found these by eyeballing helicopter.obj file
        terrain_node.reference_point = glm::vec3(0.0, 0.0, 0.0);
        helicopter_body_node.reference_point = glm::vec3(0.35, 1.5, 10.4);
        helicopter_main_rotor_node.reference_point = glm::vec3(0.0, 1.5, 0.0);
        helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
        helicopter_door_node.reference_point = glm::vec3(1.2, 0.0, 0.0);

        // For now I say that every part of the helicopter is dependent of where the body us
        helicopter_body_node.add_child(&helicopter_main_rotor_node);
        helicopter_body_node.add_child(&helicopter_tail_rotor_node);
        helicopter_body_node.add_child(&helicopter_door_node);
        // The entire helicopter get's one parent nore TODO: Could this just be body?
        helicopter_root.add_child(&helicopter_body_node);
        // Since terrain only has one vao, I let it be the root, and add helicopter as a child
        terrain_node.add_child(&helicopter_root);
        // The scene root get's the terrain itself added as a child, since every object revolves around the terrain
        root.add_child(&terrain_node);

        // Setup uniform locations
        let shdr: shader::Shader;
        let trans_loc: i32;
        let time_loc: i32;
        let opacity_loc: i32;
        unsafe {
            // Creates shader. using multiple attaches since they return self, and link them all together at the end
            shdr = shader::ShaderBuilder::new()
                .attach_file(".\\shaders\\simple.vert")
                .attach_file(".\\shaders\\simple.frag")
                .link();
            // Get uniform locations
            trans_loc = shdr.get_uniform_location("transformation");
            time_loc = shdr.get_uniform_location("time");
            opacity_loc = shdr.get_uniform_location("opacity");
            shdr.activate();
        }
        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;
        // The main rendering loop

        let persp_mat: glm::Mat4 =
            glm::perspective((SCREEN_H as f32) / (SCREEN_W as f32), 90.0, 1.0, 1000.0);

        let persp_trans: glm::Mat4 = glm::translation(&glm::vec3(0.0, 0.0, -2.0));

        let mut proj: glm::Mat4 = persp_mat * persp_trans;

        let model: glm::Mat4 = glm::identity();
        let mut trans_matrix: glm::Mat4 = glm::identity();

        let mut rot_x = 0.0;
        let mut rot_y = 0.0;
        let rot_step: f32 = 2.0;

        let mut opacity: f32 = 0.0;
        let mut v_time: f32 = 0.0;

        let mut trans_x = 0.0;
        let mut trans_y = 0.0;
        let mut trans_z = -4.0;
        let trans_step: f32 = 0.1;

        let mut view: glm::Mat4 = glm::identity();

        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    // I'm using WASDEQ to handle inputs
                    // Also use arrowkeys for rotation
                    match key {
                        VirtualKeyCode::W => {
                            trans_z += trans_step;
                        }
                        VirtualKeyCode::A => {
                            trans_x += trans_step;
                        }
                        VirtualKeyCode::S => {
                            trans_z -= trans_step;
                        }
                        VirtualKeyCode::D => {
                            trans_x -= trans_step;
                        }
                        VirtualKeyCode::E => {
                            trans_y -= trans_step;
                        }
                        VirtualKeyCode::Q => {
                            trans_y += trans_step;
                        }
                        VirtualKeyCode::R => {
                            // Reset camera
                            view = glm::identity();
                        }
                        VirtualKeyCode::Up => {
                            rot_x -= rot_step;
                        }
                        VirtualKeyCode::Down => {
                            rot_x += rot_step;
                        }
                        VirtualKeyCode::Left => {
                            rot_y -= rot_step;
                        }
                        VirtualKeyCode::Right => {
                            rot_y += rot_step;
                        }
                        _ => {}
                    }
                }
            }

            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                *delta = (0.0, 0.0);
            }

            opacity = (elapsed * 10.0).sin() / 2.0 + 0.6;
            v_time = elapsed.sin();
            let trans: glm::Mat4 = glm::translation(&glm::vec3(trans_x, trans_y, trans_z));
            let rot: glm::Mat4 = glm::rotation(rot_x.to_radians(), &glm::vec3(1.0, 0.0, 0.0))
                * glm::rotation(rot_y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
            let scale: glm::Mat4 = glm::identity();

            view = rot * trans * view;
            let mut mod_view = view * model;
            // Transmat here becomes MVP matrix after getting built up by model,
            // view ( rotation, translation ), and projection
            let view_proj_mat = proj * view;
            let trans_mat = proj * mod_view;

            // Reset values
            trans_x = 0.0;
            trans_y = 0.0;
            trans_z = 0.0;
            rot_y = 0.0;
            rot_x = 0.0;
            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                // Now we can use these uniforms in our shaders
                gl::Uniform1f(opacity_loc, opacity);
                gl::Uniform1f(time_loc, v_time);
                gl::UniformMatrix4fv(trans_loc, 1, gl::FALSE, trans_mat.as_ptr());

                /*
                // Before task 3c)
                gl::BindVertexArray(terrain_vao);
                // Issue the necessary commands to draw your scene here
                gl::DrawElements(
                    gl::TRIANGLES,
                    terrain_mesh.index_count, // Here we get the amount of indices we need
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );*/
                // After task 2c
                draw_scene(&root, &view_proj_mat, &shdr);
            }
            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}
