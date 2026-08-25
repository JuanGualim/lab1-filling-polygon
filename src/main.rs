mod framebuffer;
mod line;
mod polygon;
mod vertex;

use framebuffer::Framebuffer;
use polygon::{draw_polygon, fill_polygon, fill_polygon_with_hole};
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
    // POLIGONO 2
    // =========================================================

    let polygon2 = vec![
        Vertex::new(321, 335),
        Vertex::new(288, 286),
        Vertex::new(339, 251),
        Vertex::new(374, 302),
    ];

    // Relleno
    framebuffer.set_current_color(Color::YELLOW);

    fill_polygon(
        &mut framebuffer,
        &polygon2,
    );

    // Borde
    framebuffer.set_current_color(Color::BLACK);

    draw_polygon(
        &mut framebuffer,
        &polygon2,
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
    // POLIGONO 4
    // =========================================================

    let polygon4 = vec![
        Vertex::new(413, 177),
        Vertex::new(448, 159),
        Vertex::new(502, 88),
        Vertex::new(553, 53),
        Vertex::new(535, 36),
        Vertex::new(676, 37),
        Vertex::new(660, 52),
        Vertex::new(750, 145),
        Vertex::new(761, 179),
        Vertex::new(672, 192),
        Vertex::new(659, 214),
        Vertex::new(615, 214),
        Vertex::new(632, 230),
        Vertex::new(580, 230),
        Vertex::new(597, 215),
        Vertex::new(552, 214),
        Vertex::new(517, 144),
        Vertex::new(466, 180),
    ];

    // =========================================================
    // POLIGONO 5 - AGUJERO DEL POLIGONO 4
    // =========================================================

    let polygon5 = vec![
        Vertex::new(682, 175),
        Vertex::new(708, 120),
        Vertex::new(735, 148),
        Vertex::new(739, 170),
    ];

    framebuffer.set_current_color(Color::PURPLE);

    fill_polygon_with_hole(
        &mut framebuffer,
        &polygon4,
        &polygon5,
    );

    framebuffer.set_current_color(Color::BLACK);

    draw_polygon(
        &mut framebuffer,
        &polygon4,
    );

    draw_polygon(
        &mut framebuffer,
        &polygon5,
    );

    // =========================================================
    // GUARDAR RESULTADO
    // =========================================================

    framebuffer.render_to_file("out.bmp");

    println!("Imagen generada correctamente en out.bmp");
}