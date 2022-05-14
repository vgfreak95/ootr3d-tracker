mod citra_memory;

use std::process::{Command, Stdio};
use std::io::Write;
use faithe::process as ps;
use faithe::types::access_rights::PROCESS_ALL_ACCESS;
use std::thread;
use std::{fs::{self, File}, sync::Arc, io::Read};
use std::i64;
use citra_memory as citra;

use fltk::{app, prelude::*, window::Window, image::PngImage, frame::Frame, table::Table, group, button::Button};
// use fltk_flex::Flex;
use fltk_grid::Grid;

fn main() {

    let mut proc = ps::Processes::new()
        .unwrap_or_else(|err| panic!("Couldn't get list of processes {}", err))
        .find(|p| p.sz_exe_file == "citra-qt.exe")
        .expect("There was an error opening the citra process")
        .open(false, PROCESS_ALL_ACCESS)
        .unwrap_or_else(|err| panic!("Couldn't open process {}", err));

        
    let game_address = citra::read_citra_file();

    let i_game_address = i64::from_str_radix(&game_address, 16)
        .expect("Couldn't convert to decimal");
    
    println!("Address: {}", i_game_address);
    // assert_eq!(i_game_address, 1760616525904);

    println!("Bit Shift: {}", citra::bit_shift(0x0587A0E));

    let to_game = citra::memory_iter( i_game_address, citra::bit_shift(0x0587A0E), 8 );
    println!("To Game: {}", to_game);

    let read_game_addr = proc.read::<i64>(to_game as usize).unwrap();
    println!("Read Address: {}", read_game_addr);

    let swords = citra::memory_iter(read_game_addr, citra::move_ptr(0x0587A0E), 1);
    println!("{:08x}", swords);

    let value = proc.read::<i8>(swords as usize).unwrap();
        // .unwrap_or_else(|err| panic!("There was an error {}", err));
    
    println!("{:04b}", value);

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 400, "Hello from rust!");
    let mut grid = Grid::default_fill();

    grid.debug(true);
    grid.set_layout(3, 3);

    let mut sw1_frame = Frame::default().with_size(50, 50);
    let mut sw2_frame = Frame::default().with_size(50, 50);
    let mut sw3_frame = Frame::default().with_size(50, 50);

    let mut kokiri = PngImage::load("res/swords/kokiri_sword_dark.png").unwrap();
    let mut master = PngImage::load("res/swords/master_sword_dark.png").unwrap();
    let mut biggor = PngImage::load("res/swords/biggorons_sword_dark.png").unwrap();

    kokiri.scale(70, 70, true, true);
    master.scale(70, 70, true, true);
    biggor.scale(70, 70, true, true);
    // maste
    sw1_frame.set_image(Some(kokiri));
    sw2_frame.set_image(Some(master));
    sw3_frame.set_image(Some(biggor));

    grid.insert(&mut sw1_frame, 0, 0);
    grid.insert(&mut sw2_frame, 0, 1);
    grid.insert(&mut sw3_frame, 0, 2);


    
    
    wind.end();
    wind.show();
    app.run().unwrap();

    // println!("{}", game_address);
    // println!("{}", converted);
    // let handle = thread::spawn(move || {
    //     loop {
    //         let sword = proc.read::<i8>(0x2164a8e4a4e).unwrap();
    //         println!("{}", sword);
    //     }
    // });

    // handle.join().expect("the thread panicked");
    
}
