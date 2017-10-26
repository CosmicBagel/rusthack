extern crate user32; //for polling key states
extern crate winapi;  //for virtual key codes
extern crate kernel32; //for console drawing

use std::{time, thread}; //just for taking naps

use winapi::winuser::{VK_RIGHT, VK_LEFT, VK_DOWN, VK_UP, VK_ESCAPE};

struct KeyStates {
    upkey_state: i16,
    downkey_state: i16,
    leftkey_state: i16,
    rightkey_state: i16,
    esckey_state: i16,
}

fn main() {
    println!("it's a rust app yo!");

    let mut key_states: KeyStates = KeyStates {
        upkey_state: 0,
        downkey_state: 0,
        leftkey_state: 0,
        rightkey_state: 0,
        esckey_state: 0,
    };

    let mut screen_buffer: [[char; 6]; 6];
    screen_buffer = [['A', 'B', 'C', 'D', 'E', 'F'],
                     ['G', 'H', 'I', 'J', 'K', 'L'],
                     ['M', 'N', 'O', 'P', 'Q', 'R'],
                     ['S', 'T', 'U', 'V', 'W', 'X'],
                     ['Y', 'Z', 'a', 'b', 'c', 'd'],
                     ['e', 'f', 'g', 'h', 'i', 'j']];

    loop {
        update_key_states(&mut key_states);

        // draw scree
        draw_scene(screen_buffer);

        // if key_states.upkey_state != 0 {
        //     println!("Up key pressed");
        // }

        // if key_states.rightkey_state != 0 {
        //     println!("Right key pressed");
        // }

        // if key_states.leftkey_state != 0 {
        //     println!("Left key pressed");
        // }

        // if key_states.downkey_state != 0 {
        //     println!("Down key pressed");
        // }

        if key_states.esckey_state != 0 {
            break;
        }

        // main thread sleep in case the os needs to do a thing
        let insta_duration = time::Duration::from_millis(0);
        thread::sleep(insta_duration);
    }
}

fn draw_scene(screen_buffer: [[char; 6]; 6]) {
    // kernel32::WriteConsoleOutputW()
    // todo: make stuff draw in region fast-path style
    // todo: alternatively look into doing some drawing with gdi
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