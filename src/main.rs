#![feature(globs)]

extern crate graphics;
extern crate sdl2_window;
extern crate shader_version;
extern crate opengl_graphics;
extern crate event;
extern crate sdl2;
extern crate gfx;
extern crate gfx_graphics;
use event::{
    EventIterator,
    EventSettings,
    WindowSettings,
    Window,
    UpdateEvent
};

use sdl2_window::Sdl2Window;
use gfx::{Device, DeviceHelper};
use gfx_graphics::{
G2D,
Texture,
};

pub mod cell;
pub mod world;
pub mod engine;

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let mut window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Conway".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0
        }
    );

    let mut world = world::World::new();

    //__#__
    //___#_
    //_###_
    //
    //__#__
    //__##_
    //___#_
    //__#__
    world.set_cell(1, 0);
    world.set_cell(2, 1);
    world.set_cell(0, 2);
    world.set_cell(1, 2);
    world.set_cell(2, 2);

    let mut engine = engine::ConwayEngine::new(box world);
    
    engine.next_generation();
    engine.next_generation();
    /*engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);
    engine.next_generation(&mut world);*/
    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let (w, h) = window.get_size();
    let frame = gfx::Frame::new(w as u16, h as u16);
    let mut renderer = device.create_renderer();
    let event_settings = EventSettings {
        updates_per_second: 120,
        max_frames_per_second: 60,
    };
    
    let mut g2d = G2D::new(&mut device);

    for e in EventIterator::new(&mut window, &event_settings) {
        use event::RenderEvent;
        e.render(|_| {
            g2d.draw(&mut renderer, &frame, |c, g| {
                use graphics::*;
                c.rgb(1.0, 1.0, 1.0).draw(g);
                for (location, cell) in engine.world_ref().cells.iter() {
                    let (state, (x, y)) = (*cell, *location);
                    if state == cell::Alive {
                        c.rect(x as f64 * 10.0, y as f64 * 10.0, 10.0, 10.0).rgb(1.0, 0.0, 0.0).draw(g);
                    }
                }
            });
            
            device.submit(renderer.as_buffer());
            renderer.reset();
        });
    }
    
}

fn printw(world: &world::World) {
    let mut last_x = 0;
    let mut last_y = 0;
    for (location, cell) in world.cells.iter() {
        let (state, (x, y)) = (*cell, *location);
        if state == cell::Alive {
            for i in range(last_x, x) {
                    print!("_");
            }
            print!("#");
            if y > last_y {
                for i in range(last_y, y) {
                    println!("");
                }
            }
            last_x = x;
            last_y = y;
        }
    }
}
