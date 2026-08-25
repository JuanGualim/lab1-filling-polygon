mod framebuffer;
mod line;
mod polygon;
mod vertex;

use framebuffer::Framebuffer;
use polygon::{draw_polygon, fill_polygon};
use vertex::Vertex;

use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;

    let mut framebuffer = Framebuffer::new(width, height);
    
    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    let polygon3 = vec![
        Vertex::new(377, 249),
        Vertex::new(411, 197),
        Vertex::new(436, 249),
    ];

    // Primero rellenamos
    framebuffer.set_current_color(Color::BLUE);

    fill_polygon(
        &mut framebuffer,
        &polygon3,
    );

    // Después dibujamos el borde
    framebuffer.set_current_color(Color::BLACK);

    draw_polygon(
        &mut framebuffer,
        &polygon3,
    );

    framebuffer.render_to_file("out.bmp");

    println!("Imagen generada.");
}