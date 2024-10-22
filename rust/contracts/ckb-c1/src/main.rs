#![cfg_attr(not(feature = "simulator"), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "simulator", test))]
extern crate alloc;

#[cfg(not(any(feature = "simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "simulator", test)))]
ckb_std::default_alloc!();

fn test_a(a: i32, b: i32) {
    let c = a + b;
    ckb_std::debug!("-- c {} --", c);
}

pub fn program_entry() -> i8 {
    ckb_std::debug!("This is a sample contract!");
    test_a(10, 13);

    ckb_std::debug!("Exit");
    0
}
