use std::fs;

use ldkalby_bindings::new_blocking_ldk_alby_client;

fn main() {
    let mnemonic = std::env::var("MNEMONIC").unwrap();
    let work_dir = "./ldk";
    fs::remove_dir_all(work_dir).unwrap();

    let client = new_blocking_ldk_alby_client(mnemonic, String::from(work_dir)).unwrap();
    let result = client.get_info().unwrap();

    println!("Result: {:?}", result);
}
