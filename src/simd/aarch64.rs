use core::arch::aarch64::*;

pub type Int32x4 = int32x4_t;

pub fn add(lhs: i32, rhs: i32) -> i32 {
  unsafe {
    let lhs = vdupq_n_s32(lhs);
    let rhs = vdupq_n_s32(rhs);
    let result = vaddq_s32(lhs, rhs);

    let mut out: i32 = 0;

    vst1q_s32(&mut out as *mut i32, result);

    out
  }
}

pub fn sub(lhs: i32, rhs: i32) -> i32 {
  unsafe {
    let lhs = vdupq_n_s32(lhs);
    let rhs = vdupq_n_s32(rhs);
    let result = vsubq_s32(lhs, rhs);

    let mut out: i32 = 0;

    vst1q_s32(&mut out as *mut i32, result);

    out
  }
}

pub fn mul(lhs: i32, rhs: i32) -> i32 {
  unsafe {
    let lhs = vdupq_n_s32(lhs);
    let rhs = vdupq_n_s32(rhs);
    let result = vmulq_s32(lhs, rhs);

    let mut out: i32 = 0;

    vst1q_s32(&mut out as *mut i32, result);

    out
  }
}

pub fn add_vec4(lhs: [i32; 4], rhs: [i32; 4]) -> [i32; 4] {
  unsafe {
    let lhs = vld1q_s32(lhs.as_ptr() as *const _);
    let rhs = vld1q_s32(rhs.as_ptr() as *const _);
    let result = vaddq_s32(lhs, rhs);

    let mut out: [i32; 4] = [0; 4];

    vst1q_s32(out.as_mut_ptr() as *mut _, result);

    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simd_add_aarch64() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-10, 5), -5);
    assert_eq!(add(0, 0), 0);
  }

  #[test]
  fn test_simd_sub_aarch64() {
    assert_eq!(sub(5, 2), 3);
    assert_eq!(sub(10, 5), 5);
    assert_eq!(sub(0, 0), 0);
  }

  #[test]
  fn test_simd_mul_aarch64() {
    assert_eq!(mul(2, 3), 6);
    assert_eq!(mul(-5, 4), -20);
    assert_eq!(mul(0, 10), 0);
  }

  #[test]
  fn test_simd_add_vec4_simple_aarch64() {
    let lhs = [1, 2, 3, 4];
    let rhs = [5, 6, 7, 8];
    let expected_result = [6, 8, 10, 12];
    let result = add_vec4(lhs, rhs);

    assert_eq!(result, expected_result);
  }

  #[test]
  fn test_simd_add_vec4_complex_aarch64() {
    let lhs = [10, 20, 30, 40];
    let rhs = [15, -5, 25, -15];
    let expected_result = [25, 15, 55, 25];
    let result = add_vec4(lhs, rhs);

    assert_eq!(result, expected_result);
  }
}
