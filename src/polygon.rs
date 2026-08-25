use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::vertex::Vertex;

pub fn draw_polygon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vertex],
) {
    if vertices.len() < 2 {
        return;
    }

    for i in 0..vertices.len() {
        let start = vertices[i];

        let end = vertices[(i + 1) % vertices.len()];

        line(framebuffer, start, end);
    }
}

pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vertex],
) {
    if vertices.len() < 3 {
        return;
    }

    // Encontrar el valor mínimo y máximo de Y
    let min_y = vertices.iter().map(|v| v.y).min().unwrap();
    let max_y = vertices.iter().map(|v| v.y).max().unwrap();

    // Recorrer cada línea horizontal del polígono
    for y in min_y..=max_y {
        let mut intersections: Vec<i32> = Vec::new();

        // Revisar cada arista
        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];

            // Verificar si la línea horizontal cruza esta arista
            if (v1.y <= y && v2.y > y)
                || (v2.y <= y && v1.y > y)
            {
                let x = v1.x
                    + (y - v1.y) * (v2.x - v1.x)
                        / (v2.y - v1.y);

                intersections.push(x);
            }
        }

        // Ordenar las intersecciones de izquierda a derecha
        intersections.sort();

        // Rellenar entre pares de intersecciones
        let mut i = 0;

        while i + 1 < intersections.len() {
            let x_start = intersections[i];
            let x_end = intersections[i + 1];

            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }

            i += 2;
        }
    }
}