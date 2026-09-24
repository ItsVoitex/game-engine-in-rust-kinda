use std::ffi::CString;
use std::{mem, vec};

use glfw::PWindow;
use crate::object::{self, Object};
use crate::texture::Texture;
use shader::Shader;
use crate::mesh::Mesh;
use crate::swapper::TextureSwapper;

use crate::shader;
pub struct Scene
{
    pub x:f32,
    pub y:f32,
    z:f32,
    pub objects:Vec<Object>,
}

fn read_lines(file_path:&str) -> Vec<String>
    {
        let mut result = Vec::new();
        for line in std::fs::read_to_string(file_path).unwrap().lines() 
        {
            result.push(line.to_string())
        }
        result
    
    }
impl Scene
{
    

    pub fn create(object:Vec<Object>) -> Scene
    {
        Scene { 
            x: 0.0, 
            y: 0.0, 
            z: 0.0, 
            objects: object
        }
    }
    pub fn move_all(&mut self,delta_time:&mut f32)
    {
        for i in 0..self.objects.len()
        {
            self.objects[i].x -= 1100.0 * *delta_time 
        }
    }
    pub fn draw(&mut self)
    {
        for i in 0..self.objects.len()
            {
                self.objects[i].draw();
            }
    }

    pub fn spawn_spikes(&mut self)
    {
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
    
    


        let level1 = read_lines("Assets/level1/level.dat");
        let mut scounter = self.objects.len();
        let mut soffset: f32 = 0.0;
        for j in 0..5
        {
            for i in 0..26
            {
                if level1[j].chars().nth(i) == Some('!')
                {
                    self.objects.push(Object::create(backround,chindices,"Assets/shaders/shader.vert","Assets/shaders/shader.frag",&["Assets/textures/backround.png"]));
                    self.objects[scounter].x += 1920.0*j as f32;
                    scounter += 1;
                }
                if level1[j].chars().nth(i) == Some('*')
                {
                    soffset += 76.8
                } 
                if level1[j].chars().nth(i) == Some('^')
                { 
                    self.objects.push(Object::create(character_vertices,chindices,"Assets/shaders/shader.vert","Assets/shaders/shader.frag",&["Assets/textures/spike.png"]));
                    self.objects[scounter].x = -921.0 + soffset;
                    self.objects[scounter].y = 504.9;
                    soffset += 76.8;
                    scounter += 1;
                }
                
            } 
        }

        
    }
   
}
    
//formula to enter a width and height and make a array for you for all the positions
//optmises by making a limited amount of spikes and just moving them  in front of the player when its off screen
    


