mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    framebuffer.set_current_color(Color::RED);

    framebuffer.set_pixel(100, 100);
    framebuffer.set_pixel(101, 100);
    framebuffer.set_pixel(102, 100);
    framebuffer.set_pixel(103, 100);
    framebuffer.set_pixel(104, 100);

    framebuffer.render_to_file("out.bmp");

    println!("Imagen generada correctamente.");
}