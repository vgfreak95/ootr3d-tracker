use std::{fs::{self, File}, sync::Arc, io::Read};


pub fn read_citra_file() -> String {
    let mut file = File::open("C:/Users/VGFreak/citra/address.txt")
        .expect("Something went wrong opening the file");

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("Couldn't read the file");

    return buffer.strip_suffix("\r\n").unwrap().to_string(); 
}

pub fn memory_iter(memory_addr: i64, mut index: i64, amt: i64) -> i64 {

    let mut is_odd = false;
    let mut offset: i64 = 0;

    if index % 2 != 0 {
        is_odd = true;
        index = index - 1;
    } 

    if is_odd {
        offset = offset + (index * amt);
        offset = offset + amt;
    } else {
        offset = offset + (index * amt);
    }

    return memory_addr + offset;

}

pub fn bit_shift(vaddr: i64) -> i64 {
    
    let return_val: i64 = vaddr >> 12;
    // println!("Return from bit shift {}", return_val);
    return_val
}

pub fn move_ptr(pointer: i64) -> i64 {
    pointer & 0xFFF
}

