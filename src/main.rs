mod framebuffer;
mod gameoflife;

use framebuffer::Framebuffer;
use gameoflife::GameOfLife;
use raylib::prelude::*;
use raylib::consts::TextureFilter;

use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};


// ======== STILL LIFES ========
const BLOCK: &[(i32,i32)] = &[(0,0),(1,0),(0,1),(1,1)];

const BEEHIVE: &[(i32,i32)] = &[
    (1,0),(2,0),
    (0,1),        (3,1),
    (1,2),(2,2)
];

const LOAF: &[(i32,i32)] = &[
    (1,0),(2,0),
    (0,1),        (3,1),
    (1,2),        (3,2),
           (2,3)
];

const BOAT: &[(i32,i32)] = &[(0,0),(1,0),(0,1),(2,1),(1,2)];

const TUB: &[(i32,i32)] = &[(1,0),(0,1),(2,1),(1,2)];

// ======== OSCILLATORS ========
const BLINKER: &[(i32,i32)] = &[(0,-1),(0,0),(0,1)];
const TOAD: &[(i32,i32)] = &[(1,0),(2,0),(3,0),(0,1),(1,1),(2,1)];
const BEACON: &[(i32,i32)] = &[(0,0),(1,0),(0,1),(3,2),(2,3),(3,3)];

// ======== SPACESHIPS ========
const GLIDER: &[(i32,i32)] = &[(1,0),(2,1),(0,2),(1,2),(2,2)];

const LWSS: &[(i32,i32)] = &[
    (1,0),(2,0),(3,0),(4,0),
    (0,1),(4,1),
    (4,2),
    (0,3),(3,3)
];

const PULSAR: &[(i32,i32)] = &[
    // fila 0
    (4,0),(10,0),
    // fila 1
    (4,1),(10,1),
    // fila 2
    (4,2),(5,2),(9,2),(10,2),
    // fila 3 vacia
    // fila 4
    (0,4),(1,4),(2,4),(5,4),(6,4),(8,4),(9,4),(12,4),(13,4),(14,4),
    // fila 5
    (2,5),(4,5),(6,5),(8,5),(10,5),(12,5),
    // fila 6
    (4,6),(5,6),(9,6),(10,6),
    
    // fila 7 vacia
    
    // fila 8
    (4,8),(5,8),(9,8),(10,8),
    // fila 9
    (2,9),(4,9),(6,9),(8,9),(10,9),(12,9),
    // fila 10
    (0,10),(1,10),(2,10),(5,10),(6,10),(8,10),(9,10),(12,10),(13,10),(14,10),
    // filla 11 vacia
    // fila 12
    (4,12),(5,12),(9,12),(10,12),
    // fila 13
    (4,13),(10,13),
    // fila 14
    (4,14),(10,14),
];

// ========== SPACESHIPS ==========

// Middle‑weight spaceship (MWSS) – bounding box 6×4
// Desplazamiento: +X cada 4 generaciones
const MWSS: &[(i32,i32)] = &[
    // fila 0
    (1,0),(2,0),(3,0),(4,0),(5,0),
    // fila 1
    (0,1),                (5,1),
    // fila 2
                            (5,2),
    // fila 3
    (0,3),        (4,3),
];

// Heavy‑weight spaceship (HWSS) – bounding box 7×4
// Desplazamiento: +X cada 4 generaciones
const HWSS: &[(i32,i32)] = &[
    // fila 0
    (1,0),(2,0),(3,0),(4,0),(5,0),(6,0),
    // fila 1
    (0,1),                        (6,1),
    // fila 2
                                  (6,2),
    // fila 3
    (0,3),                (5,3),
];


const MODCATHLON: &[(i32,i32)] = &[
    // fila 0
    (1,0),
    // fila 1
    (0,1),(1,1),(2,1),
    // fila 2 vacia
    // fila 3 vacia
    // fila 4
    (0,4),(1,4),(2,4),
    // fila 5
    (0,5),(2,5),
    // fila 6
    (0,6),(2,6),
    // fila 7 vacia
    // fila 8
    (0,8),(1,8),(2,8),
    // fila 9 vacia
    // fila 10 vacia
    // fila 11
    (0,11),(1,11),(2,11),
    // fila 12
    (1,12)
];

const PENTADECATHLON: &[(i32,i32)] = &[
    // fila 0
    (1,0),
    // fila 1
    (0,1),(1,1),(2,1),
    // fila 2 vacia
    // fila 3 vacia
    // fila 4
    (0,4),(1,4),(2,4),
    // fila 5 vacia
    // fila 6
    (0,6),(2,6),
    // fila 7
    (0,7),(2,7),
    // fila 8 vacia
    // fila 9
    (0,9),(1,9),(2,9),
    // fila 10 vacia
    // fila 11 vacia
    // fila 12
    (0,12),(1,12),(2,12),
    // fila 12
    (1,13)
];

fn main() {
    let (win_w, win_h) = (800, 800);
    let (fb_w,  fb_h)  = (100, 100);   // tablero pequeño, ventana grande
    let (mut rl, thread) = raylib::init()
        .size(win_w, win_h)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(10); // 10 “turnos” por segundo

    // Framebuffer y juego
    let mut fb   = Framebuffer::new(fb_w, fb_h, Color::BLACK);
    let mut life = GameOfLife::new(fb_w, fb_h);

    // ───────── CONFIGURACIÓN INICIAL FIJA ─────────
    seed_pattern(&mut life, 43, 43, PULSAR);            // centro

    // Izquierda
    seed_pattern(&mut life, 34, 43, PENTADECATHLON);
    seed_pattern(&mut life, 24, 43, PENTADECATHLON);
    seed_pattern(&mut life, 14, 43, PENTADECATHLON);

    // Derecha
    seed_pattern(&mut life, 67, 43, PENTADECATHLON);
    seed_pattern(&mut life, 77, 43, PENTADECATHLON);
    seed_pattern(&mut life, 87, 43, PENTADECATHLON);

    // Naves arriba
    seed_pattern(&mut life, 7, 17, MWSS);
    seed_pattern(&mut life, 17, 17, HWSS);
    seed_pattern(&mut life, 27, 17, LWSS);
    seed_pattern(&mut life, 37, 17, MWSS);
    seed_pattern(&mut life, 47, 17, HWSS); 
    seed_pattern(&mut life, 57, 17, MWSS);
    seed_pattern(&mut life, 67, 17, LWSS); 
    seed_pattern(&mut life, 77, 17, HWSS); 
    seed_pattern(&mut life, 87, 17, MWSS);

    // Naves abajo
    seed_pattern(&mut life, 7, 77, MWSS);
    seed_pattern(&mut life, 7, 77, MWSS);
    seed_pattern(&mut life, 17, 77, HWSS);
    seed_pattern(&mut life, 27, 77, LWSS);
    seed_pattern(&mut life, 37, 77, MWSS);
    seed_pattern(&mut life, 47, 77, HWSS); 
    seed_pattern(&mut life, 57, 77, MWSS);
    seed_pattern(&mut life, 67, 77, LWSS); 
    seed_pattern(&mut life, 77, 77, HWSS); 
    seed_pattern(&mut life, 87, 77, MWSS);

    //Nukes arriba
    seed_pattern(&mut life, 24, 27, MODCATHLON);
    seed_pattern(&mut life, 77, 27, MODCATHLON);

    let mut tex = rl.load_texture_from_image(&thread, &fb.color_buffer).unwrap();
    tex.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    while !rl.window_should_close() {
        life.step();
        life.draw(&mut fb);
        tex.update_texture(fb.as_bytes()).unwrap();   // <─ la clave

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let scale = (win_w as f32 / fb_w as f32).min(win_h as f32 / fb_h as f32);
        d.draw_texture_ex(&tex, Vector2::zero(), 0.0, scale, Color::WHITE);
    }
}

/// Un glider en (ox, oy)
fn seed_glider(game: &mut GameOfLife, ox: i32, oy: i32) {
    let pattern = &[(1,0),(2,1),(0,2),(1,2),(2,2)];
    for &(dx,dy) in pattern {
        game.set_alive(ox+dx, oy+dy, true);
    }
}

/// Agrega cualquier patrón definido por coordenadas relativas
fn seed_pattern(game: &mut GameOfLife, ox: i32, oy: i32, pattern: &[(i32,i32)]) {
    for &(dx,dy) in pattern {
        game.set_alive(ox + dx, oy + dy, true);
    }
}

