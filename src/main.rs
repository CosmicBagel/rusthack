//for virtual key codes
extern crate kernel32;
extern crate user32;
//for polling key states
extern crate winapi; //for console drawing

use std::{thread, time}; //just for taking naps

use winapi::winnt::WCHAR; //for CHAR_INFO struct
use winapi::wincon::{CHAR_INFO, COORD, SMALL_RECT}; //required for WriteConsoleOutputW
use winapi::winuser::{VK_DOWN, VK_ESCAPE, VK_LEFT, VK_RIGHT, VK_UP}; //specific virtual key codes
use winapi::winbase::STD_OUTPUT_HANDLE; //code number used to get the handle of the console window


struct KeyStates {
    upkey_state: i16,
    downkey_state: i16,
    leftkey_state: i16,
    rightkey_state: i16,
    esckey_state: i16,
}

fn main() {
    println!("It's a rust app yo!");

    let mut key_states: KeyStates = KeyStates {
        upkey_state: 0,
        downkey_state: 0,
        leftkey_state: 0,
        rightkey_state: 0,
        esckey_state: 0,
    };

    //just some dummy data for now while I work out how to draw stuff
    let screen_buffer: [[char; 6]; 6];
    screen_buffer = [
        ['A', 'B', 'C', 'D', 'E', 'F'],
        ['G', 'H', 'I', 'J', 'K', 'L'],
        ['M', 'N', 'O', 'P', 'Q', 'R'],
        ['S', 'T', 'U', 'V', 'W', 'X'],
        ['Y', 'Z', 'a', 'b', 'c', 'd'],
        ['e', 'f', 'g', 'h', 'i', 'j'],
    ];

    loop {
        update_key_states(&mut key_states);

        // draw screen
        draw_scene(screen_buffer);

        if key_states.upkey_state != 0 {
            println!("Up key pressed");
        }

        if key_states.rightkey_state != 0 {
            println!("Right key pressed");
        }

        if key_states.leftkey_state != 0 {
            println!("Left key pressed");
        }

        if key_states.downkey_state != 0 {
            println!("Down key pressed");
        }

        if key_states.esckey_state != 0 {
            break;
        }

        // main thread sleep in case the os needs to do a thing
        //even just allowing 1ms, I've noticed a significant 
        //reduction in cpu usage without any perceptable 
        //responsiveness loss
        //also cpu runs a bit cooler as a result
        let one_millis = time::Duration::from_millis(1);
        thread::sleep(one_millis);
    }
}

fn draw_scene(screen_buffer: [[char; 6]; 6]) {
    let handle;
    //UnicodeChar = WCHAR = w_chart = c_ushort = u16

    //struct for char_info taken from winapi crate
    //pub struct CHAR_INFO {
    //     pub UnicodeChar: WCHAR, (u16 so 2 bytes)
    //     pub Attributes: WORD, (remember WORD is 2 bytes)
    // }
    let char_array = screen_buffer_to_char_info_array(screen_buffer);
    let buffer_size: COORD = COORD {X: 6, Y:6};
    let buffer_coordinates: COORD = COORD {X:0, Y:0};
    let screen_write_region: *mut SMALL_RECT = &mut SMALL_RECT {
        Top:1, Left:0, Right:6, Bottom:6};
    // BOOL WINAPI WriteConsoleOutput(
    // _In_          HANDLE      hConsoleOutput,
    // _In_    const CHAR_INFO   *lpBuffer,
    // _In_          COORD       dwBufferSize,
    // _In_          COORD       dwBufferCoord,
    // _Inout_       PSMALL_RECT lpWriteRegion
    // );

    unsafe {
        handle = kernel32::GetStdHandle(STD_OUTPUT_HANDLE);
        kernel32::WriteConsoleOutputW(
            handle, char_array.as_ptr(), buffer_size, 
            buffer_coordinates, screen_write_region);
    }

    // todo: make stuff draw in region fast-path style
    // todo: alternatively look into doing some drawing with gdi
}

fn screen_buffer_to_char_info_array<'a>(buffer: [[char; 6]; 6]) -> [CHAR_INFO; 36] {
    let mut char_info: [CHAR_INFO; 36] = [CHAR_INFO {
        UnicodeChar: 0 as WCHAR,
        Attributes: 0,
    }; 36];
    let mut counter = 0;
    for row in 0..6 {
        for col in 0..6 {
            char_info[counter] = CHAR_INFO {
                UnicodeChar: buffer[row][col] as WCHAR,
                Attributes: 0x000f,
            };
            counter += 1;
        }
    }
    
    //returning feels weird without explicitly saying return :/
    char_info
}

fn update_key_states(states: &mut KeyStates) {
    // calling windows api is inherently unsafe :b
    unsafe {
        states.rightkey_state = user32::GetAsyncKeyState(VK_RIGHT);
        states.leftkey_state = user32::GetAsyncKeyState(VK_LEFT);
        states.downkey_state = user32::GetAsyncKeyState(VK_DOWN);
        states.upkey_state = user32::GetAsyncKeyState(VK_UP);
        states.esckey_state = user32::GetAsyncKeyState(VK_ESCAPE);
    }
}
