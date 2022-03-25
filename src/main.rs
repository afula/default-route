mod route;

use local_ip_address::local_ip;

fn main() {
    // let my_local_ip = local_ip().unwrap();
    //
    // println!("This is my local IP address: {:?}", my_local_ip);
    let interface = route::cmd::get_default_interface().unwrap();
    println!("This is my default network interface : {:?}", interface);
}