use crate::framebuffer::Framebuffer;
use crate::line::line;
use crate::vertex::Vertex;

// =========================================================
// DIBUJAR EL CONTORNO DE UN POLIGONO
// =========================================================

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


// =========================================================
// CALCULAR INTERSECCIONES DE UNA SCANLINE
// =========================================================

fn add_intersections(
    vertices: &[Vertex],
    y: i32,
    intersections: &mut Vec<f64>,
) {
    for i in 0..vertices.len() {
        let v1 = vertices[i];
        let v2 = vertices[(i + 1) % vertices.len()];

        // La scanline debe cruzar la arista.
        //
        // Usamos:
        // ymin <= y < ymax
        //
        // Esto evita contar dos veces un mismo vertice.
        if (v1.y <= y && v2.y > y)
            || (v2.y <= y && v1.y > y)
        {
            let x1 = v1.x as f64;
            let y1 = v1.y as f64;

            let x2 = v2.x as f64;
            let y2 = v2.y as f64;

            let scan_y = y as f64;

            // Interseccion de la arista con la scanline
            let x = x1
                + (scan_y - y1)
                    * (x2 - x1)
                    / (y2 - y1);

            intersections.push(x);
        }
    }
}


// =========================================================
// RELLENAR UN POLIGONO
// =========================================================

pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vertex],
) {
    if vertices.len() < 3 {
        return;
    }

    let min_y = vertices
        .iter()
        .map(|v| v.y)
        .min()
        .unwrap();

    let max_y = vertices
        .iter()
        .map(|v| v.y)
        .max()
        .unwrap();

    // Recorremos horizontalmente el poligono
    for y in min_y..=max_y {
        let mut intersections: Vec<f64> = Vec::new();

        add_intersections(
            vertices,
            y,
            &mut intersections,
        );

        // Ordenar de izquierda a derecha
        intersections.sort_by(|a, b| {
            a.partial_cmp(b).unwrap()
        });

        // Rellenamos las intersecciones por pares
        let mut i = 0;

        while i + 1 < intersections.len() {
            // ceil:
            // primer pixel que realmente esta dentro
            let x_start = intersections[i].ceil() as i32;

            // floor:
            // ultimo pixel que realmente esta dentro
            let x_end = intersections[i + 1].floor() as i32;

            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }

            i += 2;
        }
    }
}


// =========================================================
// RELLENAR POLIGONO CON AGUJERO
// =========================================================

pub fn fill_polygon_with_hole(
    framebuffer: &mut Framebuffer,
    outer: &[Vertex],
    hole: &[Vertex],
) {
    if outer.len() < 3 {
        return;
    }

    let min_y = outer
        .iter()
        .map(|v| v.y)
        .min()
        .unwrap();

    let max_y = outer
        .iter()
        .map(|v| v.y)
        .max()
        .unwrap();

    for y in min_y..=max_y {
        let mut intersections: Vec<f64> = Vec::new();

        // Intersecciones con el contorno exterior
        add_intersections(
            outer,
            y,
            &mut intersections,
        );

        // Intersecciones con el agujero
        add_intersections(
            hole,
            y,
            &mut intersections,
        );

        // Ordenamos todas las intersecciones
        intersections.sort_by(|a, b| {
            a.partial_cmp(b).unwrap()
        });

        // Regla Even-Odd
        let mut i = 0;

        while i + 1 < intersections.len() {
            let x_start = intersections[i].ceil() as i32;
            let x_end = intersections[i + 1].floor() as i32;

            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }

            i += 2;
        }
    }
}