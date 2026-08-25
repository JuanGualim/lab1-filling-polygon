mod framebuffer;
mod line;
mod polygon;
mod vertex;

use framebuffer::Framebuffer;
use polygon::{draw_polygon, fill_polygon};
use vertex::Vertex;

use raylib::prelude::*;

fn main() {
    // Tamaño de la imagen
    let width = 800;
    let height = 600;

    // Crear framebuffer
    let mut framebuffer = Framebuffer::new(width, height);

    // Fondo blanco
    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    // =========================================================
    // POLIGONO 1
    // =========================================================

    let polygon1 = vec![
        Vertex::new(165, 380),
        Vertex::new(185, 360),
        Vertex::new(180, 330),
        Vertex::new(207, 345),
        Vertex::new(233, 330),
        Vertex::new(230, 360),
        Vertex::new(250, 380),
        Vertex::new(220, 385),
        Vertex::new(205, 410),
        Vertex::new(193, 383),
    ];

    // Primero rellenamos el poligono
    framebuffer.set_current_color(Color::GREEN);

    fill_polygon(
        &mut framebuffer,
        &polygon1,
    );

    // Luego dibujamos el borde encima
    framebuffer.set_current_color(Color::BLACK);

    draw_polygon(
        &mut framebuffer,
        &polygon1,
    );

    // =========================================================
    // POLIGONO 3
    // =========================================================

    let polygon3 = vec![
        Vertex::new(377, 249),
        Vertex::new(411, 197),
        Vertex::new(436, 249),
    ];

    // Relleno azul
    framebuffer.set_current_color(Color::BLUE);

    fill_polygon(
        &mut framebuffer,
        &polygon3,
    );

    // Borde negro
    framebuffer.set_current_color(Color::BLACK);

    draw_polygon(
        &mut framebuffer,
        &polygon3,
    );

    // =========================================================
    // GUARDAR RESULTADO
    // =========================================================

    framebuffer.render_to_file("out.bmp");

    println!("Imagen generada correctamente en out.bmp");
}