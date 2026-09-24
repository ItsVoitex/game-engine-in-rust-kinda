use std::ffi::CString;

use glfw::PWindow;
use shader::Shader;
use crate::mesh::Mesh;
use crate::swapper::TextureSwapper;

use crate::shader;
pub struct Player
{
    pub x:f32,
    pub y:f32,
    z:f32,
    pub current_texture:u32,
    pub textures:TextureSwapper,
    pub shader:Shader,
    data:Mesh,
    offset:i32,
    on_ground:bool,
    velocity_y:f32,
}

impl Player
{


    pub fn create(character_vertices:[f32;20],chindices:[u32;6],vertex_path:&str,fragment_path:&str,texture_path:&[&str]) -> Player
    {
        let shader = Shader::new(vertex_path, fragment_path);
        let offset_location;

        unsafe {
            let offset_name = CString::new("offset").unwrap();

            offset_location = gl::GetUniformLocation(shader.id,offset_name.as_ptr()
        );}
        
        Player
        {
            x: -921.6+72.6*5.0,
            y: 504.9,
            z:0.0,
            current_texture: 0,
            textures: TextureSwapper::create(texture_path),
            on_ground : true,
            shader: shader,
            data: Mesh::gendata(character_vertices,chindices),
            offset:offset_location,
            velocity_y:0.0,

        }
    }
    pub fn draw(&mut self)
    {
        if self.current_texture == 0
        {
            self.current_texture = self.textures.textures[0].id;
        }

        let texture_location;
        unsafe { 
            let texture_name = CString::new("texture1").unwrap();
            texture_location = gl::GetUniformLocation(self.shader.id,texture_name.as_ptr());
            gl::Uniform1i(texture_location,0);
            

            self.shader.use_program();
            self.data.mesh_bind();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.current_texture);
            gl::Uniform3f(self.offset, self.x, self.y,self.z);
            gl::DrawElements(gl::TRIANGLES, 6,gl::UNSIGNED_INT,std::ptr::null());
        }
    }

    
    pub fn moved(&mut self,window:&PWindow,delta_time:&mut f32,current:&mut i32) 
    {
       
        
        if !self.on_ground
        {
            self.velocity_y += 50.0;
            self.y += self.velocity_y * *delta_time;

            if self.y >= 504.9
            {
                self.y = 504.9;
                self.velocity_y = 0.0;
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
            self.y += 270.0 * *delta_time;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press || window.get_key(glfw::Key::Left) == glfw::Action::Press
        {
            self.x -= 480.0 * *delta_time;
           
            
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press || window.get_key(glfw::Key::Right) == glfw::Action::Press
        {
            self.x += 480.0 * *delta_time;
            
        }
        if (window.get_key(glfw::Key::Space) == glfw::Action::Press) && self.on_ground
        {
            self.y -= (270.0*130.0) * *delta_time;
            self.on_ground = false;

        }
        if self.x + 38.4*1.5 < -921.6
        {
            self.x = -921.6;
            
            
        }
    }

    pub fn _simplemove(&mut self,window:&PWindow,delta_time:&mut f32,current:&mut i32)
    {
        
        
        if window.get_key(glfw::Key::R) == glfw::Action::Press
        {
            *current = 0;
        }
        if window.get_key(glfw::Key::T) == glfw::Action::Press
        {
            *current = 0;
            self.y += 270.0 * *delta_time;
        }
        if window.get_key(glfw::Key::W) == glfw::Action::Press || window.get_key(glfw::Key::Up) == glfw::Action::Press
        {
            
            self.current_texture = self.textures.textures[3].id;
            self.y -= 270.0 * *delta_time;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press || window.get_key(glfw::Key::Left) == glfw::Action::Press
        {
            self.current_texture = self.textures.textures[1].id;
            self.x -= 480.0 * *delta_time;
           
            
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press || window.get_key(glfw::Key::Down) == glfw::Action::Press
        {
            self.current_texture = self.textures.textures[2].id;
            self.y += 270.0 * *delta_time;
            
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press || window.get_key(glfw::Key::Right) == glfw::Action::Press
        {
            self.current_texture = self.textures.textures[0].id;
            self.x += 480.0 * *delta_time;
            
        }
        

        
        if self.x + 38.4 < -921.6
        {
            self.current_texture = self.textures.textures[1].id;
            self.x = -921.6;
            
            
        }
        
        
    }

}