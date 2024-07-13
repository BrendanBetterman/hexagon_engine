


#[allow(unused_imports)]
use glium::{glutin, Surface};

#[derive(Copy, Clone)]
pub struct CameraState {
    aspect_ratio: f32,
    pub position: (f32, f32, f32),
    direction: (f32, f32, f32),
    angle: (f32,f32,f32),
    mouse_start_position: (Option<f32>,Option<f32>),
    delta_time: f32,
    rotate_left: bool,
    rotate_right: bool,
    pub clicked: bool,
    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    pi: f32,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1920.0 / 1080.0,//768.0
            position: (0.0, 1.6, 0.0),
            direction: (0.0, 0.0, -1.0),
            angle: (0.0,0.0,0.0),
            mouse_start_position: (None,None),
            delta_time: 0.0,
            rotate_left: false,
            rotate_right: false,
            clicked: false,
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            pi: std::f32::consts::PI,
        }
    }
    pub fn set_delta_time(&mut self,time:f32){
        self.delta_time = time;
    }
    pub fn get_rot_y(&self) -> [[f32; 4]; 4] {
        let angle:f32 = self.angle.1;
        [
            [ angle.cos(),0.0,angle.sin(),   0.0],
            [0.0         ,1.0,0.0        ,   0.0],
            [-angle.sin(),0.0,angle.cos(),   0.0],
            [0.0         ,0.0,0.0        ,   1.0],
        ]
    }
    pub fn get_rot_z(&self) -> [[f32; 4]; 4] {
        let angle:f32 = self.angle.2;
        [
            [angle.cos(),-angle.sin(),0.0,   0.0],
            [angle.sin(), angle.cos(),0.0,   0.0],
            [0.0        ,0.0        ,1.0,   0.0],
            [0.0        ,0.0        ,0.0,   1.0],
        ]
    }

    pub fn get_rot_x(&self) -> [[f32; 4]; 4] {
        let angle:f32 = self.angle.0;
        [
            [1.0,0.0        ,0.0         ,   0.0],
            [0.0,angle.cos(),-angle.sin(),   0.0],
            [0.0,angle.sin(), angle.cos(),   0.0],
            [0.0,0.0        ,0.0         ,   1.0],
        ]
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = self.pi / 2.0;
        let zfar = 1024.0;
        let znear = 0.01;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);
            
        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
                 -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
                 -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
            
        ]
    }

    pub fn update(&mut self) {
        
        let move_speed = 2.5* self.delta_time;
        //let up = (0.0, 1.0, 0.0);

        //let s = (f.1 * up.2 - f.2 * up.1,
        //         f.2 * up.0 - f.0 * up.2,
          //       f.0 * up.1 - f.1 * up.0);

        let s = {
            //let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            //let len = len.sqrt();
            ((self.angle.1+(self.pi*0.5)).sin(), 0.0, (self.angle.1+(self.pi*1.5)).cos())
        };
        if self.clicked{
            self.pi += 0.01;
            println!("{}",self.pi);
        }

        if self.moving_up {
            //self.position.0 += move_speed;
            self.position.1 += move_speed;
            //self.position.2 += move_speed;
        }
        if self.moving_down {
           // self.position.0 -=  move_speed;
            self.position.1 -=  move_speed;
           // self.position.2 -=  move_speed;
        }
        //new
        if self.rotate_left {
            self.angle.0 += 0.01;
            self.angle.1 += 0.01;
            self.angle.2 += 0.01;
        }
        if self.rotate_right {
            self.angle.0 -=  0.01;
            self.angle.1 -=  0.01;
            self.angle.2 -=  0.01;
        }
        if self.moving_left {
            self.position.0 -= s.0 * move_speed;
            self.position.1 -= s.1 * move_speed;
            self.position.2 -= s.2 * move_speed;
        }


        if self.moving_right {
            self.position.0 += s.0 * move_speed;
            self.position.1 += s.1 * move_speed;
            self.position.2 += s.2 * move_speed;
        }

        if self.moving_forward {
            self.position.0 += (self.angle.1).sin() * move_speed;
            //self.position.1 += f.1 * move_speed;
            self.position.2 += (self.angle.1+self.pi).cos() * move_speed;
        }

        if self.moving_backward {
            self.position.0 -= (self.angle.1).sin() * move_speed;
           // self.position.1 -= f.1 * move_speed;
            self.position.2 -= (self.angle.1+self.pi).cos() * move_speed;
        }
    }

    pub fn process_mouse(&mut self, event: &glutin::event::WindowEvent<'_>) {
        //get mouse position
        let position = match *event{
            glutin::event::WindowEvent::CursorMoved { position, ..} =>position,
            _ => return,
        };
        
        //check if mouse origin was set
        let mouse_x:f32 = match self.mouse_start_position.0 {
            None => {self.mouse_start_position.0 = Some(position.x as f32); position.x as f32},
            Some(v) => v,
        };
        
        let mouse_y:f32 = match self.mouse_start_position.1 {
            None => {self.mouse_start_position.1 = Some(position.y as f32); position.y as f32},
            Some(v) => v,
        };
        //self.mouse_start_position.1 = Some(mouse_y);
        //set view angle of player
        let sensitivity:f32 = 75.0;
        self.angle.0 = (position.y as f32/sensitivity) - mouse_y/sensitivity;//look in z
        self.angle.1 = (position.x as f32/sensitivity) - mouse_x/sensitivity;//look in x
        //limit up look
        if self.angle.0 < -1.0{ //limit up look
            self.angle.0 = -1.0;
        }else if self.angle.0 > 1.0{
            self.angle.0 = 1.0;
        }
        //self.direction = (self.angle.0.sin(),0.0,-1.0);
    }
    pub fn process_input(&mut self, event: &glutin::event::WindowEvent<'_>) {
        
        /* working keyboard only*/
        
        let input = match event {
            glutin::event::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
            
        };

        let pressed = input.state == glutin::event::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            
            glutin::event::VirtualKeyCode::Space => self.moving_up = pressed,
            glutin::event::VirtualKeyCode::LShift => self.moving_down = pressed,
            //new
            glutin::event::VirtualKeyCode::Left => self.rotate_left = pressed,
            glutin::event::VirtualKeyCode::Right => self.rotate_right = pressed,

            glutin::event::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::event::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::event::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::event::VirtualKeyCode::S => self.moving_backward = pressed,
            glutin::event::VirtualKeyCode::T => print!("{:?}",self.position),
            glutin::event::VirtualKeyCode::E => self.clicked = pressed,
            _ => (),
        };
        
        
    }
}
