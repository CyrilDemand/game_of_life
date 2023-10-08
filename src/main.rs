use std::time::Instant;
extern crate sdl2;
extern crate gl;
extern crate rand;
extern crate bit_vec;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use rand::random;

const TAB_LEN: usize=200;

fn main() {
    let start = Instant::now();

    let mut tab: [[u8; TAB_LEN]; TAB_LEN] = [[0; TAB_LEN]; TAB_LEN];
    let mut buffer: [[u8; TAB_LEN]; TAB_LEN] = [[0; TAB_LEN]; TAB_LEN];
    init_random_tab(&mut tab);


    let duration = start.elapsed();
    println!("Temps écoulé: {:?}", duration);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of life", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    main_iteration(&mut tab, &mut buffer, &mut Some(&mut canvas), &mut event_pump);

}

fn init_random_tab(tab: &mut [[u8; TAB_LEN]; TAB_LEN]) {
    for row in tab.iter_mut() {
        for cell in row.iter_mut() {
            *cell = if random::<f32>() < 0.5 { 0 } else { 1 };
        }
    }
}

fn afficher_tab_gl(tab: &[[u8; TAB_LEN];TAB_LEN], canvas: &mut WindowCanvas, event_pump: &mut EventPump, &mut paused: &mut bool){
    if !paused {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for (row_idx, row) in tab.iter().enumerate() {
            for (cell_idx, &cell) in row.iter().enumerate() {
                if cell==0 {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                }else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                canvas.fill_rect(Rect::new((cell_idx * 1000 / row.len()) as i32, (row_idx * 1000 / tab.len()) as i32, (1000 / tab.len()) as u32, (1000 / row.len()) as u32)).unwrap();

            }
        }

        canvas.present();
    }

    std::thread::sleep(std::time::Duration::new(0, 50_000_000));
}

fn main_iteration(tab: &mut [[u8;TAB_LEN];TAB_LEN], buffer: &mut [[u8;TAB_LEN];TAB_LEN],  canvas: &mut Option<&mut WindowCanvas>, event_pump: &mut sdl2::EventPump) {
    let mut paused:bool = false;
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;  // <- L'utiliser ici devrait fonctionner
                },
                _ => {}
            }
        }
        if !paused {
            for row in 0..tab.len() {
                for cell in 0..tab[row].len() {
                    buffer[row][cell] = calc_friend(tab, row, cell);
                }
            }
            match canvas {
                Some(ref mut can) => afficher_tab_gl(buffer, can, event_pump, &mut paused),
                None => afficher_tab_ascii(buffer),
            }

            // échange entre 'tab' et 'buffer'
            std::mem::swap(tab, buffer);
        }
    }
}


fn calc_friend(tab: &[[u8; TAB_LEN];TAB_LEN], row: usize, cell: usize) -> u8 {
    let mut friend: u8 = 0;

    for r in row.saturating_sub(1)..=row.saturating_add(1) {
        for c in cell.saturating_sub(1)..=cell.saturating_add(1) {
            if r < TAB_LEN && c < TAB_LEN && !(r == row && c == cell) && tab[r][c] == 1 {
                friend += 1;
            }
        }
    }

    if tab[row][cell] == 0 && friend == 3 {
        return 1;
    } else if tab[row][cell] == 1 && (friend == 3 || friend == 2) {
        return 1;
    }
    return 0;
}

fn afficher_tab_ascii(tab: &[[u8; TAB_LEN];TAB_LEN]) {
    for row in tab.iter() {
        for &cell in row {
            match cell {
                0 => print!("D "),
                1 => print!("L "),
                _ => {}
            }
        }
        println!();
    }
}
