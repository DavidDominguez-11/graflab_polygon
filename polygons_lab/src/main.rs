mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;

fn main() {
    let width = 1000;
    let height = 1000;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));
    framebuffer.clear();

    let poligono1 = vec![
    (165, 380), (185, 360), (180, 330), (207, 345), 
    (233, 330), (230, 360), (250, 380), (220, 385), 
    (205, 410), (193, 383)
    ];

    let poligono2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    let poligono3 = vec![
        (377, 249), (411, 197), (436, 249)
    ];

    let poligono4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), 
        (535, 36), (676, 37), (660, 52), (750, 145), 
        (761, 179), (672, 192), (659, 214), (615, 214), 
        (632, 230), (580, 230), (597, 215), (552, 214), 
        (517, 144), (466, 180)
    ];

    let poligono5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    dibujar_poligono_con_agujero(&poligono4, &poligono5, &mut framebuffer, Color::GREEN);

    let poligono3 = vec![
        (377, 249), (411, 197), (436, 249)
    ];
    dibujar_poligono_relleno(&poligono3, &mut framebuffer, Color::RED);

    let poligono2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];
    dibujar_poligono_relleno(&poligono2, &mut framebuffer, Color::BLUE);

    let poligono1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), 
        (233, 330), (230, 360), (250, 380), (220, 385), 
        (205, 410), (193, 383)
    ];
    dibujar_poligono_relleno(&poligono1, &mut framebuffer, Color::YELLOW);
    framebuffer.render_to_file("out.bmp");


}

fn dibujar_poligono_relleno(poligono: &[(i32, i32)], framebuffer: &mut Framebuffer, fill_color: Color) {

    framebuffer.set_current_color(Color::WHITE);
    dibujar_contorno(poligono, framebuffer);

    let min_y = poligono.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = poligono.iter().map(|&(_, y)| y).max().unwrap();

    framebuffer.set_current_color(fill_color);

    for y in min_y..=max_y {
        let mut intersecciones = Vec::new();

        for i in 0..poligono.len() {
            let (x1, y1) = poligono[i];
            let (x2, y2) = poligono[(i + 1) % poligono.len()];

            if y1 == y2 {
                continue;
            }

            if (y >= y1 && y < y2) || (y >= y2 && y < y1) {
                let x = x1 + ((y - y1) as f32 * (x2 - x1) as f32 / (y2 - y1) as f32).round() as i32;
                intersecciones.push(x);
            }
        }
        intersecciones.sort();

        for i in (0..intersecciones.len()).step_by(2) {
            if i + 1 >= intersecciones.len() {
                break;
            }
            let x_start = intersecciones[i];
            let x_end = intersecciones[i + 1];
            for x in x_start..=x_end {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }

    framebuffer.set_current_color(Color::WHITE);
    dibujar_contorno(poligono, framebuffer);
}


fn dibujar_contorno(poligono: &[(i32, i32)], framebuffer: &mut Framebuffer) {
    for i in 0..poligono.len() {
        let (x1, y1) = poligono[i];
        let (x2, y2) = poligono[(i + 1) % poligono.len()];
        
        line(
            framebuffer,
            Vector2::new(x1 as f32, y1 as f32),
            Vector2::new(x2 as f32, y2 as f32),
        );
    }
}

fn dibujar_poligono_con_agujero(
    poligono1: &[(i32, i32)],
    poligono2: &[(i32, i32)],
    framebuffer: &mut Framebuffer,
    fill_color: Color,
) {
    framebuffer.set_current_color(fill_color);
    let min_y = poligono1.iter()
        .chain(poligono2.iter())
        .map(|&(_, y)| y)
        .min()
        .unwrap();
    let max_y = poligono1.iter()
        .chain(poligono2.iter())
        .map(|&(_, y)| y)
        .max()
        .unwrap();
    for y in min_y..=max_y {
        let mut intersecciones = Vec::new();

        intersecciones.extend(scanline_intersecciones(y, poligono1));
        intersecciones.extend(scanline_intersecciones(y, poligono2));

        intersecciones.sort();

        for i in (0..intersecciones.len()).step_by(2) {
            if i + 1 >= intersecciones.len() {
                break;
            }
            let x_start = intersecciones[i];
            let x_end = intersecciones[i + 1];

            for x in x_start..=x_end {
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }

    framebuffer.set_current_color(Color::WHITE);
    dibujar_contorno(poligono1, framebuffer);
    dibujar_contorno(poligono2, framebuffer);
}


fn scanline_intersecciones(y: i32, poligono: &[(i32, i32)]) -> Vec<i32> {
    let mut intersecciones = Vec::new();

    for i in 0..poligono.len() {
        let (x1, y1) = poligono[i];
        let (x2, y2) = poligono[(i + 1) % poligono.len()];

        if y1 == y2 {
            continue;
        }

        if (y >= y1 && y < y2) || (y >= y2 && y < y1) {
            let x = x1 + ((y - y1) as f32 * (x2 - x1) as f32 / (y2 - y1) as f32).round() as i32;
            intersecciones.push(x);
        }
    }

    intersecciones
}

