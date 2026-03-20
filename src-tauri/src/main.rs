#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use mimalloc::MiMalloc;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

fn main() {
  kanji_lib::run();
}
