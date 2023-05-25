#[cfg(target_arch = "aarch64")]
fn eval_simd_aarch64() {
  use simderella::simd::aarch64;

  println!("{}", aarch64::add(4, 8));
  println!("{}", aarch64::sub(10, 5));
  println!("{}", aarch64::mul(1024, 1024));
}

#[cfg(target_arch = "x86_64")]
fn eval_simd_x86_64() {
  use simderella::simd::x86_64;

  println!("{}", x86_64::add(4, 8));
  println!("{}", x86_64::sub(10, 5));
  println!("{}", x86_64::mul(1024, 1024));
}

fn main() {
  #[cfg(target_arch = "aarch64")]
  eval_simd_aarch64();
  #[cfg(target_arch = "x86_64")]
  eval_simd_x86_64();
}
