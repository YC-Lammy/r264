#[cfg(feature = "simd")]
#[inline]
fn nal_escape_part(dst: &mut [u8], src: &[u8]) -> usize {
    let mut i = 2;

    for b in src {
        let b = *b;

        if b <= 0x03 && dst[i - 2] == 0 && dst[i - 1] == 0 {
            dst[i] = 0x03;
            i += 1;
        }
        dst[i] = b;
        i += 1;
    }

    return i;
}

#[cfg(feature = "simd")]
#[inline]
fn nal_escape_simd(dst: &mut [u8], src: &[u8]) {
    use core::simd::Simd;
    use core::simd::{cmp::SimdPartialOrd, num::SimdUint};

    const SIMD_LEN: usize = 64;

    let mut dst_pos;
    let mut src_pos = 2;

    if let Some(b) = src.get(0) {
        dst[0] = *b;
    }
    if let Some(b) = src.get(1) {
        dst[1] = *b;
    }
    if src.len() < 3 {
        return;
    }
    let (a, b, c) = src[2..].as_simd::<SIMD_LEN>();

    dst_pos = nal_escape_part(dst, a);
    src_pos += a.len();

    for s in b {
        let le_mask = s.simd_le(Simd::from_array([0x03; SIMD_LEN]));

        // slow path
        if le_mask.any() {
            let has_zero = s.reduce_min() == 0;

            // only iterate when there is any zero
            if has_zero {
                // first byte in simd
                if le_mask.test(0) && dst[dst_pos - 2] == 0 && dst[dst_pos - 1] == 0 {
                    dst[dst_pos] = 0x03;
                    dst_pos += 1;
                }

                dst[dst_pos] = src[src_pos];
                dst_pos += 1;
                src_pos += 1;

                // second byte in simd
                if le_mask.test(1) && dst[dst_pos - 2] == 0 && dst[dst_pos - 1] == 0 {
                    dst[dst_pos] = 0x03;
                    dst_pos += 1;
                }

                dst[dst_pos] = src[src_pos];
                dst_pos += 1;
                src_pos += 1;

                for i in 0..(SIMD_LEN - 2) {
                    if le_mask.test(i + 2) && dst[dst_pos - 2] == 0 && dst[dst_pos - 1] == 0 {
                        dst[dst_pos] = 0x03;
                        dst_pos += 1;
                    }

                    dst[dst_pos] = src[src_pos];
                    dst_pos += 1;
                    src_pos += 1;
                }
            } else {
                // only check for the previous two zeros
                if le_mask.test(0) && dst[dst_pos - 2] == 0 && dst[dst_pos - 1] == 0 {
                    dst[dst_pos] = 0x03;
                    dst_pos += 1;
                }

                //s.copy_to_slice(&mut dst[dst_pos..]);
                unsafe {
                    let tmp = s;
                    core::ptr::copy_nonoverlapping(
                        tmp.as_array(),
                        dst.as_mut_ptr().add(dst_pos) as _,
                        1,
                    )
                }

                dst_pos += SIMD_LEN;
                src_pos += SIMD_LEN;
            }
        } else {
            // fast path, no value is less then 0x03
            //s.copy_to_slice(&mut dst[dst_pos..]);
            unsafe {
                let tmp = s;
                core::ptr::copy_nonoverlapping(
                    tmp.as_array(),
                    dst.as_mut_ptr().add(dst_pos) as _,
                    1,
                )
            }

            dst_pos += SIMD_LEN;
            src_pos += SIMD_LEN;
        }
    }

    debug_assert!(src.len() - src_pos == c.len());
    nal_escape_part(&mut dst[(dst_pos - 2)..], c);
}

#[allow(unused)]
#[inline]
fn nal_escape_no_simd(dst: &mut [u8], src: &[u8]) {
    let mut dst_pos = 2;

    if let Some(b) = src.get(0) {
        dst[0] = *b;
    }
    if let Some(b) = src.get(1) {
        dst[1] = *b;
    }
    if src.len() < 3 {
        return;
    }

    for b in &src[2..] {
        if *b <= 0x03 && dst[dst_pos - 2] == 0 && dst[dst_pos - 1] == 0 {
            dst[dst_pos] = 0x03;
            dst_pos += 1;
        }

        dst[dst_pos] = *b;
        dst_pos += 1;
    }
}

pub fn nal_escape(dst: &mut [u8], src: &[u8]) {
    #[cfg(feature = "simd")]
    nal_escape_simd(dst, src);
    #[cfg(not(feature = "simd"))]
    nal_escape_no_simd(dst, src);
}

#[test]
fn test() {
    use rand::Rng;

    let mut t1 = 0;
    let mut t2 = 0;

    for _ in 0..10000 {
        let mut dst: [u8; 4320] = [0; 4320];
        let mut dst_1: [u8; 4320] = [0; 4320];

        let mut src: [u8; 4096] = [0; 4096];
        rand::thread_rng().fill(&mut src);

        let inst = std::time::Instant::now();
        nal_escape_no_simd(&mut dst_1, &src);
        let e = inst.elapsed().as_nanos();
        t1 += e;

        let inst = std::time::Instant::now();
        nal_escape_simd(&mut dst, &src);
        let e = inst.elapsed().as_nanos();
        t2 += e;

        assert!(dst == dst_1)
    }

    println!("no simd: {}ns", t1 / 10000);
    println!("simd: {}ns", t2 / 10000);
}
