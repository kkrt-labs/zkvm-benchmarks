#![no_std]
#![no_main]
// #![feature(lang_items)]        // lang_items の使用を有効にする
// #![feature(core_intrinsics)]   // intrinsics の使用を有効にする

// use core::intrinsics;

// // パニックハンドラ
// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     unsafe { intrinsics::abort() }
// }

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
// #[export_name = "fibonacci"]
#[no_mangle]
pub fn fibonacci(n: u32) -> u32 {
  let mut a = 0u32;
  let mut b = 1u32;
  for _ in 0..n {
      let c = a.wrapping_add(b);
      a = b;
      b = c;
  }
  b
}

// /// Fibonacci 関数
// #[no_mangle]
// pub extern "C" fn fibonacci(n: u32) -> (u32, u32)  {
//   let mut a = 0u32;
//   let mut b = 1u32;
//   for _ in 0..n {
//       let c = a.wrapping_add(b);
//       a = b;
//       b = c;
//   }
//   (a, b)
// }