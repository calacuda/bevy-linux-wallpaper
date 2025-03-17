use x11rb::connection::Connection;

pub mod wallpaper_plugin;

pub fn get_screen_roots() -> Vec<u32> {
    let (conn, screen_num) = x11rb::connect(None).unwrap();
    // println!(
    //     "root window id (i think) {:?}",
    //     &conn.setup().roots[screen_num].root
    // );
    // println!("screen_num {screen_num}");

    vec![conn.setup().roots[screen_num].root]
}
