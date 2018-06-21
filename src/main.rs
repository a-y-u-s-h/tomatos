//=============================================================
//                                                            ||
//                                                            ||
//                      T O M A T O S                         ||
//                                                            ||
//                                                            ||
//============================================================||
//                                                            ||
//               Author     :   Ayush Sharma                  ||
//               Github     :   a-y-u-s-h                     ||
//               Website    :   a-y-u-s-h.github.io           ||
//               Email      :   code.ayush@gmail.com          ||
//                                                            ||
//=============================================================


//-------------------------------------------------------------
//
// [1] 
// 
// DISABLE STANDARD LIBRARIES :-
// ==============================
//  
// Disable all standard library..
// because some of them are OS dependent,
// we can't use OS dependent stuff to make an OS.
// 
#![no_std]
// 
// SIDE EFFECTS OF THIS STEP :-
// ============================
// 
// This raises two errors during compilation. One of them can
// be fixed by a step mentioned in `Cargo.toml` and for the 
// other one we need to implement that function ourselves, 
// which is done in step [1] -> [2] in this same file.
// 
//-------------------------------------------------------------


//-------------------------------------------------------------
//
// [1] -> [2] 
// 
// ENABLE AN UNSTABLE FEATURE `panic_implementation` 
// ====================================================
// 
// `panic_implementation` feature which we'll use below is 
// unstable so we need to say "alright, I know this is unstable..
// I want to use it anyways" to the compiler so that an error
// goes away. Below line does exactly that. 
// 
#![feature(panic_implementation)]
//
// Now, we just need to implement this feature : 
// Steps : [1] -> [2] -> [3] does that. 
//-------------------------------------------------------------


//-------------------------------------------------------------
//
// [2] 
// 
// OVERWRITE THE ENTRY POINT : 
// ===========================
// 
// After finishing steps : ([1] -> [2] -> [3]) in `main.rs`
// and [1] in `main.rs` -> [2] in `Cargo.toml`, there'll be
// error : requires `start` lang_item. To fix this we need to
// overwrite all rust level entry points, i.e. we need to 
// disable the normal entry point, i.e. `main` function and 
// put this at the beginning of the file.
// 
#![no_main]
// 
// `main` function doesn't make sense without an underlying 
// runtime that calls it, so we'll now overwrite entry point,
// and define our own `_start` function. While doing so it's 
// important to disable "name mangling" so that compiler doesn't
// generate some cryptic symbol that linker won't recognize.
// `_start` is our entry point since the linker looks for a 
// function named `_start` by default.
//
#[no_mangle]
pub extern "C" fn _start() -> ! {
  print()
}
//
// We also need to specify that `_start` is supposed to use 
// C calling convention (instead of unspecified Rust calling
// convention). Return type is 'never' (!) because entry point
// is not called by any function, but invoked directly by OS or
// bootloader. In our case, shutting down the machine could be a 
// reasonable action, since there's nothing left to do if a 
// freestanding binary returns. For now, we fulfill the requirement
// by looping endlessly.
//   
// SIDE EFFECTS OF THIS STEP :-
// ============================
// 
// Ugly linker error after this step. Also this step is different
// for different operating systems. To fix this, we need to
// compile this project with, if target specification doesn't exist : 
// 
// `cargo rustc -- -Z pre-link-arg=-nostartfiles`
// 
// Via `cargo rustc` command we can pass linker attributes. It works
// exactly like `cargo build` but also allows to pass options to 
// `rustc`, underlying rust compiler. `rustc` has (unstable)
// `-Z pre-link-arg` flag which passes an argument to linker.
// 
// If target specification (x86_64-tomatos.json) exists, use this : 
// 
// `cargo xbuild --target x86_64-tomatos.json`
// 
// To install `xbuild` do this :
// 
// `cargo install cargo-xbuild && rustup component add rust-src` 
// 
//-------------------------------------------------------------


//-------------------------------------------------------------
// 
// [1] -> [2] -> [3]
//  
// DESCRIBE WHAT HAPPENS ON `PANIC` 
// ====================================
// 
// Function runs whenever panic happens. `PanicInfo` contains 
// file and line where panic happened and optional message.
// `!` is 'never' type, we don't want to return anything on 
// panic. So, we'll just loop infinitely.
// 
use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
  loop {}
}
// 
//-------------------------------------------------------------


static HELLO: &[u8] = b"Hello World!";
fn print() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}