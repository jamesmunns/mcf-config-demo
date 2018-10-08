extern crate mcf_config_demo_lib as demo;

fn main() {
    for (i, b) in demo::BUFFER.iter().enumerate() {
        println!("{:03}: 0x{:02x}", i, b);
    }

    demo::greets();
}
