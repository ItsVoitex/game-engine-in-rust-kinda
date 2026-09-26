use glfw::{Context,fail_on_errors};
use std::ffi::{c_void,CString};


pub struct Engine
{
    pub window:glfw::PWindow,
    pub glfw:glfw::Glfw,


    pub delta_time:f32,
    last_time:f64,
    current_time:f64
}

impl Engine
{
    pub fn init(width:u32,height:u32,window_title:&str) -> Engine
    {
        let mut glfw = glfw::init(fail_on_errors).unwrap();
        let (mut window,_events) = glfw.create_window(width, height, window_title, glfw::WindowMode::Windowed).expect("failed to init window");
        window.make_current();

        gl::load_with(|symbol| {
            window.get_proc_address(symbol)
            .map_or(std::ptr::null(), |f| f as *const c_void)
        });
        let last_time:f64 =  glfw.get_time();

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
        }


        Engine { 
            window, 
            glfw,
            delta_time: 0.0,
            last_time: last_time,
            current_time: 0.0
        }
        

    }
    pub fn should_close(&mut self) ->bool
    {
        self.current_time = self.glfw.get_time();
        self.delta_time = (self.current_time - self.last_time) as f32;
        self.last_time = self.current_time;
        self.window.should_close()
        
    }

    pub fn begin_drawing(&mut self)
    {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn end_drawing(&mut self)
    {
        self.window.swap_buffers();
        self.glfw.poll_events();
    }
    
}