use std::ffi::CString;
use crate::swapper::TextureSwapper;
use crate::texture::Texture;
use shader::Shader;
use crate::mesh::Mesh;

use crate::shader;

pub struct Object
{
    pub x:f32,
    pub y:f32,
    z:f32,
    offset:i32,
    pub object_data:ObjectData
}

pub struct ObjectData
{
    pub current_texture:u32,
    pub textures:TextureSwapper,
    pub shader:Shader,
    data:Mesh,
}


impl Object 
{
    pub fn multi_create(object_data:ObjectData) -> Object
    {
        
        let offset_location;
        unsafe {
            let offset_name = CString::new("offset").unwrap();

            offset_location = gl::GetUniformLocation(object_data.shader.id,offset_name.as_ptr()
        );}

        Object
        {
            x:0.0,
            y:0.0,
            z:0.0,
            offset:offset_location,
            object_data : object_data
        }
    }
    pub fn create(object_vertices:[f32;20],indices:[u32;6],vertex_path:&str,fragment_path:&str,texture_path:&[&str]) -> Object
    {
        let shader = Shader::new(vertex_path, fragment_path);
        let offset_location;

        unsafe {
            let offset_name = CString::new("offset").unwrap();

            offset_location = gl::GetUniformLocation(shader.id,offset_name.as_ptr()
        );}
        
        Object
        {
            x:0.0,
            y:0.0,
            z:0.0,
            offset:offset_location,
            object_data : ObjectData::create(object_vertices, indices, vertex_path, fragment_path, texture_path)
        }
    }
    
    
    pub fn draw(&mut self)
    {
        if self.object_data.current_texture == 0
        {
            self.object_data.current_texture = self.object_data.textures.textures[0].id;
        }

        let texture_location;
        unsafe { 
            let texture_name = CString::new("texture1").unwrap();
            texture_location = gl::GetUniformLocation(self.object_data.shader.id,texture_name.as_ptr());
            gl::Uniform1i(texture_location,0);

            self.object_data.shader.use_program();
            self.object_data.data.mesh_bind();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.object_data.current_texture);
            gl::Uniform3f(self.offset, self.x, self.y,self.z);
            gl::DrawElements(gl::TRIANGLES, 6,gl::UNSIGNED_INT,std::ptr::null());
        }
    }
   

   
   
}

impl ObjectData
{
    pub fn create(object_vertices:[f32;20],indices:[u32;6],vertex_path:&str,fragment_path:&str,texture_path:&[&str]) -> ObjectData
    {
        let shader = Shader::new(vertex_path, fragment_path);
        ObjectData {
                current_texture: 0,
                textures: TextureSwapper::create(texture_path),
                shader: shader,
                data: Mesh::gendata(object_vertices,indices),

        
        }
    }
    
    
    
}