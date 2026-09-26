mod player;
use std::ffi::{CString, c_void};
use glfw::{Context, Key::Space, fail_on_errors};
use gl;
use player::Player;
mod scene;
use rmiv_engine::engine;
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

    
    let width = 2560;
    let height = 1440;

    let mut engine = engine::Engine::init(width, height, "Zombie Dash");
    
    
    /*let character_vertices: [f32; 20] = [
     998.4,  575.1,  -1.0, 0.0, 0.0,
     921.6,  575.1,  -1.0, 1.0, 0.0,
     921.6,  504.9,  -1.0, 1.0, 1.0,
     998.4,  504.9,  -1.0, 0.0, 1.0,
    ];
    */
    let width = 76.8;
    let height = 70.2;
    let character_vertices: [f32; 20] = [
        960.0+width/2.0,  540.0+height/2.0,  -0.1, 0.0, 0.0,
        960.0-width/2.0,  540.0+height/2.0,  -0.1, 1.0, 0.0,
        960.0-width/2.0,  540.0-height/2.0,  -0.1, 1.0, 1.0,
        960.0+width/2.0,  540.0-height/2.0,  -0.1, 0.0, 1.0,
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

    

    
    
    let mut Scene1 = scene::Scene::create();
    Scene1.spawn_spikes();
    let mut player = Player::create(
        76.6,70.2,-0.1,
        &["Assets/textures/zombie.png"]);
    
    


    let mut current = 0;

    
    
    
    let mut getkey = false;

    while !engine.should_close(){
        
        

        engine.begin_drawing();
        
        
        Scene1.draw();

        player.draw();

        engine.end_drawing();
        
        player.moved(&engine.window,&mut engine.delta_time,&mut current);
        
        if engine.window.get_key(glfw::Key::Space) == glfw::Action::Press
        {
            getkey = true;
        }
        Scene1.move_spikes();
        if getkey{Scene1.move_all(&mut engine.delta_time);}
        

        

    }


}