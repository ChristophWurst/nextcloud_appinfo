extern crate nextcloud_appinfo;

use nextcloud_appinfo::get_appinfo;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    if env::args().len() < 2 {
        println!("no path given");
        process::exit(1);
    }

    let param = env::args().nth(1).unwrap();
    let path = Path::new(&param);
    match get_appinfo(path) {
        Ok(info) => println!("Appinfo parsed: {:?}", info),
        Err(e) => {
            println!("Could not load appinfo: {}", e);
            process::exit(2);
        }
    };
}
