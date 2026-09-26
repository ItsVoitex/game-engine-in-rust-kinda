use std::ffi::CString;

use glfw::PWindow;
use rmiv_engine::mesh::Mesh;
use rmiv_engine::object::{self, Object};
use rmiv_engine::swapper::TextureSwapper;

use rmiv_engine::shader::Shader;
pub struct Player
{
    pub object:Object,
    on_ground:bool,
}

impl Player
{


    pub fn create(width: f32, height: f32, depth: f32, texture_path: &[&str]) -> Player
    {
        let object = object::Object::create(width, height, depth, texture_path);

        
        Player
        {
            object: object,
            on_ground: false

        }
    }
    pub fn draw(&mut self) {
        self.object.draw();
        
    }

    pub fn moved(&mut self,window:&PWindow,delta_time:&mut f32,current:&mut i32) 
    {
       
        
        if !self.on_ground
        {
            self.object.velocity_y += 50.0;
            self.object.y += self.object.velocity_y * *delta_time;

            if self.object.y >= 504.9
            {
                self.object.y = 504.9;
                self.object.velocity_y = 0.0;
                self.on_ground = true
            }
            
            
        }
        if window.get_key(glfw::Key::R) == glfw::Action::Press
        {
            *current = 0;
        }
        if window.get_key(glfw::Key::T) == glfw::Action::Press
        {
            *current = 0;
            self.object.y += 270.0 * *delta_time;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press || window.get_key(glfw::Key::Left) == glfw::Action::Press
        {
            self.object.x -= 480.0 * *delta_time;
           
            
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press || window.get_key(glfw::Key::Right) == glfw::Action::Press
        {
            self.object.x += 480.0 * *delta_time;
            
        }
        if (window.get_key(glfw::Key::Space) == glfw::Action::Press) && self.on_ground
        {
            self.object.y -= (270.0*130.0) * *delta_time;
            self.on_ground = false;

        }
        if self.object.x + 38.4*1.5 < -921.6
        {
            self.object.x = -921.6;
            
            
        }
    }


}