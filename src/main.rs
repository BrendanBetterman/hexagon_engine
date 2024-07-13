#[macro_use]

extern crate glium;
#[allow(unused_imports)]
use glium::{glutin, Surface};
use std::time::Instant;
mod render;


fn main() {
    // building the display, ie. the main object
    let event_loop = glutin::event_loop::EventLoop::new();
    
    let mut renderer = render::render::Renderer::new(&event_loop);
    let mut camera = render::support::camera::CameraState::new();
    let mut now = Instant::now();
    let mut frame = 0;
    let mut delta_time = Instant::now();
    
    // the main loop
    render::support::start_loop(event_loop, move |events| {
        //now = Instant::now();
        camera.set_delta_time(delta_time.elapsed().as_secs_f32());
        delta_time = Instant::now();
        camera.update();
        
        frame +=1;
         if frame >=10000{
            println!("{}FPS",1.0/(now.elapsed().as_secs_f64()/10000.0));
            frame = 0;
            now = Instant::now();

        }

        
        renderer.render_frame(&camera);
        
        let mut action = render::support::Action::Continue;

        // polling and handling the events received by the window
        for event in events {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = render::support::Action::Stop,
                    
                    ev => camera.process_input(&ev),
                },
                _ => (),
            }
           
        };
        for event in events {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => action = render::support::Action::Stop,
                    
                    ev => camera.process_mouse(&ev),
                },
                _ => (),
            }
           
        };
        
        //latency 
        /*if frame >= 1000{
            frame =0;
            renderer.update_mesh(0);
        }*/
        
        action
        
    });
}
