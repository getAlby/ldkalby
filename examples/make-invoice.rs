use std::fs;

use ldkalby_bindings::{new_blocking_ldk_alby_client, MakeInvoiceRequest};

fn main() {
    let mnemonic = std::env::var("MNEMONIC").unwrap();
    let work_dir = "./ldk";
    fs::remove_dir_all(work_dir).unwrap();

    let client = new_blocking_ldk_alby_client(mnemonic, String::from(work_dir)).unwrap();

    let result = client
        .make_invoice(MakeInvoiceRequest { amount_msat: 1000 })
        .unwrap();

    println!("Result: {:?}", result);
}
