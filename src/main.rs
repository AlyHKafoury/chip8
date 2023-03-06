use chip8::Chip8;
use std::{thread, time};

mod chip8;


fn main() {
    let mut mychip8: Chip8 = Chip8::new();
    setup_graphics();
    setup_input();
    
    mychip8.init();
    mychip8.load_game("./games/tictac");

    loop {
        mychip8.emulate();
        mychip8.set_keys();
        thread::sleep(time::Duration::from_millis(50));
        if !(mychip8.draw_flag()) {
            continue;
        }
        mychip8.debug_render();
    }
}

fn draw_graphics() -> () {
    
}

fn setup_input() -> () {
    
}

fn setup_graphics() -> () {
    
}
