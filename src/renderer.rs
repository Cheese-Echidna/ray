use crate::*;
use rayon::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{PhysicalSize};
use winit::event::WindowEvent;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode};
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

    // let light_fn = |t: f32| {
    //     let num_periods = t / 15.0;
    //     let angle = num_periods * TAU;
    //     let p = Vec3::new(angle.cos(), angle.sin(), 1.) * 5.0;
    //     LightSource::new(p, 0.7, 3.0)
    // };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Use Poll to continuously redraw; or Wait if you prefer.

        match event {
            Event::WindowEvent {
                window_id: _window_id,
                event,
            } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    // Exit when the user presses Escape.
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },

            Event::MainEventsCleared => {
                draw(&scene, pixels.frame_mut());
                // also update application state
                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                // Draw the current frame to the screen.
                if let Err(err) = pixels.render() {
                    println!("pixels.render() failed: {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}

fn draw(scene: &Scene, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let px = i % WIDTH;
        let py = i / WIDTH;
        let x = px / SCALE;
        let y = py / SCALE;

        let pos = Vec2::new(
            x as f32 / (WIDTH / SCALE) as f32 - 0.5,
            0.5 - y as f32 / (HEIGHT / SCALE) as f32,
        );

        let (r, g, b) = {
            let colour = scene.trace_from_image_prop(pos);
            <Srgb<u8>>::from_vec3(colour).into_components()
        };

        pixel.copy_from_slice(&[r, g, b, 255]);
    }
}


use image;
use image::ImageResult;
use rayon::prelude::IntoParallelIterator;

pub fn render2(scene: Scene) -> ImageResult<()> {
    let (width, height) = (WIDTH as u32, HEIGHT as u32);
    let mut image = image::RgbImage::new(width, height);
    let scale = SCALE as u32;

    (0..width).into_iter().flat_map(|px| {
        (0..height).into_iter().map(move |py| {
            let x = px / scale;
            let y = py / scale;
            let pos = Vec2::new(
                x as f32 / (WIDTH / SCALE) as f32 - 0.5,
                0.5 - y as f32 / (HEIGHT / SCALE) as f32,
            );
            (px, py, pos)
        })
    }).collect::<Vec<_>>().into_par_iter().map(|(px, py, pos)| {
        let (r, g, b) = {
            let colour = scene.trace_from_image_prop(pos);
            <Srgb<u8>>::from_vec3(colour).into_components()
        };
        (px, py, [r, g, b])
    }).collect::<Vec<_>>().into_iter().for_each(|(px, py, colour)| {
        image.put_pixel(px, py, image::Rgb::from(colour));

    });

    image.save("latest.png")
}
