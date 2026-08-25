mod framebuffer;
mod line;
mod vertex;

use framebuffer::Framebuffer;
use line::line;
use vertex::Vertex;

use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    framebuffer.set_current_color(Color::RED);

    line(
        &mut framebuffer,
        Vertex::new(100, 100),
        Vertex::new(400, 300),
    );

    framebuffer.set_current_color(Color::BLUE);

    line(
        &mut framebuffer,
        Vertex::new(400, 300),
        Vertex::new(600, 100),
    );

    framebuffer.set_current_color(Color::GREEN);

    line(
        &mut framebuffer,
        Vertex::new(600, 100),
        Vertex::new(100, 100),
    );

    framebuffer.render_to_file("out.bmp");

    println!("Imagen generada.");
}