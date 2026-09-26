use std::ffi::CString;

use glfw::PWindow;
use rmiv_engine::object::Object;
use rmiv_engine::object::ObjectData;



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
    

    pub fn create() -> Scene
    {
        let object :Vec<Object> = Vec::new();
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
            if self.objects[i].object_data.object_type != "backround"
            {
                self.objects[i].x -= 1100.0 * *delta_time 
            }
            
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
    

        let obdat = ObjectData::create(76.8,70.0,-1.0,&["Assets/textures/spike.png"],String::from("spike"));
        let obdat1 = ObjectData::create(1920.0,1080.0,0.0,&["Assets/textures/backround.png"],String::from("backround"));
        
        self.objects.push(Object::multi_create(&obdat1,String::from("backround")));

        for i in 0..20
        {
            self.objects.push(Object::multi_create(&obdat,String::from("spike")));
        }
    }
    pub fn move_spikes(&mut self)
    {
        let level1 = read_lines("Assets/level1/level.dat");
        let mut scounter = 1;
        let mut ocounter= 0;
        let mut soffset: f32 = 0.0;

        for j in 0..5
        {
            for i in 0..26
            {
                
                if level1[j].chars().nth(i) == Some('*')
                {
                    soffset += 76.8
                } 
                if scounter > 20
                {
                    scounter = 1;
                }
                if level1[j].chars().nth(i) == Some('^')
                { 
                    if self.objects[scounter].object_data.object_type == "spike"
                    {
                        if self.objects[scounter].x < -960.0
                        {
                            ocounter += 1;
                            soffset+=76.8;
                        }
                        if ocounter == 5
                        {
                            self.objects[scounter].x = -921.6 + soffset;
                            self.objects[scounter].y = 504.9;
                            soffset += 76.8;
                            scounter += 1;
                            ocounter = 0;
                        }
                        
                    }
                }
                
            } 
        }

        
    
    }   
   
}
    
//formula to enter a width and height and make a array for you for all the positions
//optmises by making a limited amount of spikes and just moving them  in front of the player when its off screen
    


