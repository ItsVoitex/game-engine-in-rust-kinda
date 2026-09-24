mod player;
mod mesh;
mod shader;
mod texture;
mod object;
mod swapper;
mod scene;
use std::ffi::{CString, c_void};
use glfw::{Context,fail_on_errors};
use gl;
use player::Player;
use object::Object;
/*
max
-921.6
921.6

min
-504.9
504.9
*/
// screen split into 25
// 76.8 for gap



fn main()
{
    let mut glfw = glfw::init(fail_on_errors).unwrap();

    
    let width = 2560;
    let height = 1440;

    let (mut window,_events) = glfw.create_window(width, height, "the best", glfw::WindowMode::Windowed).expect("failed to init window");

    

    window.make_current();

    gl::load_with(|symbol| {
        window.get_proc_address(symbol)
        .map_or(std::ptr::null(), |f| f as *const c_void)

    });
    
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
    }
    
    
    
    let character_vertices: [f32; 20] = [
     998.4,  575.1,  -1.0, 0.0, 0.0,
     921.6,  575.1,  -1.0, 1.0, 0.0,
     921.6,  504.9,  -1.0, 1.0, 1.0,
     998.4,  504.9,  -1.0, 0.0, 1.0,
    ];
    

    let chindices: [u32; 6]= [
        0,  1,  2,
        0,  2,  3,
    ];

    let backround: [f32; 20] = [
     1920.0, 1080.0,  0.0, 0.0, 0.0,
        0.0, 1080.0,  0.0, 1.0, 0.0,
        0.0,    0.0,  0.0, 1.0, 1.0,
     1920.0,    0.0,  0.0, 0.0, 1.0,
    ];
    
    
    

    let baindices: [u32; 6]= [
        0,  1,  2,
        0,  2,  3,
    ];

    

    
    let mut temp = Vec::new();
    temp.push(Object::create(backround,baindices,"Assets/shaders/shader.vert","Assets/shaders/shader.frag",&["Assets/textures/backround.png"]));
    let mut Scene1 = scene::Scene::create(temp);
    Scene1.spawn_spikes();
    let mut player = Player::create(
        character_vertices,
        chindices,
        "Assets/shaders/shader.vert",
        "Assets/shaders/shader.frag",
        &["Assets/textures/zombie.png"]);
    
    


    let mut current = 0;

    let mut last_time = glfw.get_time();
    
    
    
    
    

    while !window.should_close(){
        let current_time = glfw.get_time();
        let mut delta_time = (current_time - last_time) as f32;
        last_time = current_time;
        player.moved(&window,&mut delta_time,&mut current);

        
        

        unsafe {
            
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            player.draw();

            Scene1.draw();

            
            
            Scene1.move_all(&mut delta_time);
            
        }
        window.swap_buffers();
        glfw.poll_events();

    }


}