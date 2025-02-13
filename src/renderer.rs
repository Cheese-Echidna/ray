use std::f64::consts::TAU;
use crate::*;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub fn render(mut scene: Scene) {
    let start_time = std::time::SystemTime::now();
    let event_loop = EventLoop::new();
    let window = {
        let size = PhysicalSize::new(WIDTH as u32, HEIGHT as u32);
        WindowBuilder::new()
            .with_title("Raytracer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            // .with_maximized(true)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    // let light_fn = |t: f64| {
    //     let num_periods = t / 15.0;
    //     let angle = num_periods * TAU;
    //     let p = Vec3::new(angle.cos(), angle.sin(), 1.) * 5.0;
    //     LightSource::new(p, 0.7, 3.0)
    // };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // scene.lights = vec![light_fn(start_time.elapsed().unwrap().as_secs_f64())];

            draw(&scene, pixels.frame_mut());

            if let Err(err) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        window.request_redraw();
    });

}

fn draw(scene: &Scene, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let px = i % WIDTH;
        let py = i / WIDTH;
        let x = px / SCALE;
        let y = py / SCALE;

        let pos = Vec2::new(x as f64 / (WIDTH / SCALE) as f64 - 0.5, 0.5 - y as f64 / (HEIGHT / SCALE) as f64);


        // LinSrgb<f32> -> Srgb<f32> -> Srgb<u8> -> (u8, u8, u8)
        let (r, g, b) = {
            let x: LinSrgb<f32> = scene.trace_from_image_prop(pos);
            let y: Srgb<f32> = x.into_encoding();
            let z: Srgb<u8> = y.into_format();
            z.into_components()
        };

        pixel.copy_from_slice(&[r, g, b, 255]);
        //dprintln!();
    }
    //dprintln!("Frame")
}

// let exact = (px % SCALE + py % SCALE) == 0;
// let flag = (x,y) == (3,5) && exact;
// if flag {
//     //dprintln!("Flag hit!")
// }
// //dprintln!("Px Pos = ({x}, {y})");

// pub fn render2(scene:Scene) -> Option<()> {
//     let mut image = RgbaImage::new(WIDTH, HEIGHT, Rgba::transparent());
//     for px in 0..WIDTH {
//         for py in 0..HEIGHT {
//             let (x,y) = (px as f64 / WIDTH as f64 - 0.5, py as f64 / HEIGHT as f64 - 0.5);
//             let colour = scene.trace(x,y);
//             image.set_pixel(px, py, colour).unwrap()
//         }
//     }
//     image.save("latest.png")
// }
