use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;

pub fn line(
    framebuffer: &mut Framebuffer,
    start: Vertex,
    end: Vertex,
) {
    let mut x0 = start.x;
    let mut y0 = start.y;

    let x1 = end.x;
    let y1 = end.y;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };

    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        framebuffer.set_pixel(x0, y0);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * error;

        if e2 >= dy {
            error += dy;
            x0 += sx;
        }

        if e2 <= dx {
            error += dx;
            y0 += sy;
        }
    }
}