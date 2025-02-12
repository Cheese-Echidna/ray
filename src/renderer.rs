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

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
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
        let x = (i % WIDTH);
        let y = (i / WIDTH);

        let pos = Vec2::new(x as f64 / WIDTH as f64 - 0.5, 0.5 - y as f64 / HEIGHT as f64);

        let (r, g, b) = scene.trace(pos).into_components();
        let convert = |x: f32| (x * 255.) as u8;
        let rgba_slice = [r, g, b, 1.0].map(convert);

        pixel.copy_from_slice(&rgba_slice);
    }
}

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