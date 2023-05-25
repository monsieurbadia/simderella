use core::arch::x86_64::*;

pub fn add(lhs: i32, rhs: i32) -> i32 {
  unsafe {
    let lhs = _mm_set1_epi32(lhs);
    let rhs = _mm_set1_epi32(rhs);
    let result = _mm_add_epi32(lhs, rhs);

    let mut out: i32 = 0;

    _mm_storeu_si128(&mut out as *mut i32 as *mut _, result);

    out
  }
}

pub fn sub(lhs: i32, rhs: i32) -> i32 {
  unsafe {
    let lhs = _mm_set1_epi32(lhs);
    let rhs = _mm_set1_epi32(rhs);
    let result = _mm_sub_epi32(lhs, rhs);

    let mut out: i32 = 0;

    _mm_storeu_si128(&mut out as *mut i32 as *mut _, result);

    out
  }
}

pub fn mul(lhs: i32, rhs: i32) -> i32 {
  unsafe {
    let lhs = _mm_set1_epi32(lhs);
    let rhs = _mm_set1_epi32(rhs);
    let result = _mm_mullo_epi32(lhs, rhs);

    let mut out: i32 = 0;

    _mm_storeu_si128(&mut out as *mut i32 as *mut _, result);

    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simd_add_x86_64() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-10, 5), -5);
    assert_eq!(add(0, 0), 0);
  }

  #[test]
  fn test_simd_sub_x86_64() {
    assert_eq!(sub(5, 2), 3);
    assert_eq!(sub(10, 5), 5);
    assert_eq!(sub(0, 0), 0);
  }

  #[test]
  fn test_simd_mul_x86_64() {
    assert_eq!(mul(2, 3), 6);
    assert_eq!(mul(-5, 4), -20);
    assert_eq!(mul(0, 10), 0);
  }
}
