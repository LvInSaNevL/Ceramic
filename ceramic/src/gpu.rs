use core::ops::Deref;

const xMax: usize = 7;
const yMax: usize = 7;
static mut vram: [[char; xMax]; yMax] = [['\0'; xMax]; yMax];

pub fn clear() {
    for x in 0..xMax {
        for y in 0..yMax {
            unsafe {
                vram[x][y] = 'f';
            }
        }
    }
}

pub fn render() {
    print!("{}[2J", 27 as char);
    unsafe {
        for x in 0..xMax {
            let out_string: String = vram[x].iter().collect();
            println!("{}", out_string);
        }
    }
}

pub fn update(input: char, x: usize, y: usize) {
    unsafe {
        vram[x][y] = input;
    }
}

pub fn str_update(input: &str, x: usize, y: usize) {
    let form_char: Vec<char> = input.chars().collect();
    for (i, c) in form_char.iter().enumerate() {
        update(*(c), y, (x + i));
    }
}