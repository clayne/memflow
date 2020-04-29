use std::{thread, time};

use log::Level;
use simple_logger;

use flow_win32::*;

pub fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();

    let mut mem = flow_qemu_procfs::Memory::new().unwrap();

    let win = Win32::try_with(&mut mem).unwrap();
    let offsets = Win32Offsets::try_with_guid(&win.kernel_guid()).unwrap();

    let kbd = Keyboard::with(&mut mem, &win, &offsets).unwrap();

    loop {
        let kbs = kbd.state(&mut mem).unwrap();
        println!("{:?}", kbs.down(win_key_codes::VK_SPACE).unwrap());
        thread::sleep(time::Duration::from_millis(1000));
    }
}