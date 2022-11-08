use super::utils::{adc, mac, sbb};

#[inline]
pub const fn add(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (l0, c) = adc(a[0], b[0], 0);
    let (l1, c) = adc(a[1], b[1], c);
    let (l2, c) = adc(a[2], b[2], c);
    let (l3, _) = adc(a[3], b[3], c);

    sub([l0, l1, l2, l3], p, p)
}

#[inline]
pub const fn sub(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (l0, brw) = sbb(a[0], b[0], 0);
    let (l1, brw) = sbb(a[1], b[1], brw);
    let (l2, brw) = sbb(a[2], b[2], brw);
    let (l3, brw) = sbb(a[3], b[3], brw);

    let (l0, c) = adc(l0, p[0] & brw, 0);
    let (l1, c) = adc(l1, p[1] & brw, c);
    let (l2, c) = adc(l2, p[2] & brw, c);
    let (l3, _) = adc(l3, p[3] & brw, c);

    [l0, l1, l2, l3]
}

#[inline]
pub const fn double(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    add(a, a, p)
}

#[inline]
pub const fn mul(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let (l0, c) = mac(0, a[0], b[0], 0);
    let (l1, c) = mac(0, a[0], b[1], c);
    let (l2, c) = mac(0, a[0], b[2], c);
    let (l3, l4) = mac(0, a[0], b[3], c);

    let (l1, c) = mac(l1, a[1], b[0], 0);
    let (l2, c) = mac(l2, a[1], b[1], c);
    let (l3, c) = mac(l3, a[1], b[2], c);
    let (l4, l5) = mac(l4, a[1], b[3], c);

    let (l2, c) = mac(l2, a[2], b[0], 0);
    let (l3, c) = mac(l3, a[2], b[1], c);
    let (l4, c) = mac(l4, a[2], b[2], c);
    let (l5, l6) = mac(l5, a[2], b[3], c);

    let (l3, c) = mac(l3, a[3], b[0], 0);
    let (l4, c) = mac(l4, a[3], b[1], c);
    let (l5, c) = mac(l5, a[3], b[2], c);
    let (l6, l7) = mac(l6, a[3], b[3], c);

    mont([l0, l1, l2, l3, l4, l5, l6, l7], p, inv)
}

#[inline]
pub const fn square(a: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    mul(a, a, p, inv)
}

#[inline]
pub fn neg(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    if a == [0; 4] {
        a
    } else {
        sub(p, a, p)
    }
}

#[inline]
pub const fn mont(a: [u64; 8], p: [u64; 4], inv: u64) -> [u64; 4] {
    let rhs = a[0].wrapping_mul(inv);

    let (_, d) = mac(a[0], rhs, p[0], 0); // a + (b * c) + d = 4 + INV * MOD[0] + 0;
    let (l1, d) = mac(a[1], rhs, p[1], d); //
    let (l2, d) = mac(a[2], rhs, p[2], d);
    let (l3, d) = mac(a[3], rhs, p[3], d);
    let (l4, e) = adc(a[4], 0, d);

    let rhs = l1.wrapping_mul(inv);

    let (_, d) = mac(l1, rhs, p[0], 0);
    let (l2, d) = mac(l2, rhs, p[1], d);
    let (l3, d) = mac(l3, rhs, p[2], d);
    let (l4, d) = mac(l4, rhs, p[3], d);
    let (l5, e) = adc(a[5], e, d);

    let rhs = l2.wrapping_mul(inv);
    let (_, d) = mac(l2, rhs, p[0], 0);
    let (l3, d) = mac(l3, rhs, p[1], d);
    let (l4, d) = mac(l4, rhs, p[2], d);
    let (l5, d) = mac(l5, rhs, p[3], d);
    let (l6, e) = adc(a[6], e, d);

    let rhs = l3.wrapping_mul(inv);
    let (_, d) = mac(l3, rhs, p[0], 0);
    let (l4, d) = mac(l4, rhs, p[1], d);
    let (l5, d) = mac(l5, rhs, p[2], d);
    let (l6, d) = mac(l6, rhs, p[3], d);
    let (l7, _) = adc(a[7], e, d);

    sub([l4, l5, l6, l7], p, p)
}

#[inline]
pub fn invert(a: [u64; 4], p: [u64; 4], inv: u64) -> Option<[u64; 4]> {
    let zero: [u64; 4] = [0, 0, 0, 0];
    if a == zero {
        return None;
    }

    let mut t1 = square(a, p, inv);
    let mut t0 = square(t1, p, inv);
    let mut t3 = mul(t0, t1, p, inv);
    let t6 = mul(t3, a, p, inv);
    let t7 = mul(t6, t1, p, inv);
    let t12 = mul(t7, t3, p, inv);
    let t13 = mul(t12, t0, p, inv);
    let t16 = mul(t12, t3, p, inv);
    let t2 = mul(t13, t3, p, inv);
    let t15 = mul(t16, t3, p, inv);
    let t19 = mul(t2, t0, p, inv);
    let t9 = mul(t15, t3, p, inv);
    let t18 = mul(t9, t3, p, inv);
    let t14 = mul(t18, t1, p, inv);
    let t4 = mul(t18, t0, p, inv);
    let t8 = mul(t18, t3, p, inv);
    let t17 = mul(t14, t3, p, inv);
    let t11 = mul(t8, t3, p, inv);
    t1 = mul(t17, t3, p, inv);
    let t5 = mul(t11, t3, p, inv);
    t3 = mul(t5, t0, p, inv);
    t0 = square(t5, p, inv);
    let t0 = multi_square(t0, p, inv, 5);
    let t0 = mul(t0, t3, p, inv);
    let t0 = multi_square(t0, p, inv, 6);
    let t0 = mul(t0, t8, p, inv);
    let t0 = multi_square(t0, p, inv, 7);
    let t0 = mul(t0, t19, p, inv);
    let t0 = multi_square(t0, p, inv, 6);
    let t0 = mul(t0, t13, p, inv);
    let t0 = multi_square(t0, p, inv, 8);
    let t0 = mul(t0, t14, p, inv);
    let t0 = multi_square(t0, p, inv, 6);
    let t0 = mul(t0, t18, p, inv);
    let t0 = multi_square(t0, p, inv, 7);
    let t0 = mul(t0, t17, p, inv);
    let t0 = multi_square(t0, p, inv, 5);
    let t0 = mul(t0, t16, p, inv);
    let t0 = multi_square(t0, p, inv, 3);
    let t0 = mul(t0, a, p, inv);
    let t0 = multi_square(t0, p, inv, 11);
    let t0 = mul(t0, t11, p, inv);
    let t0 = multi_square(t0, p, inv, 8);
    let t0 = mul(t0, t5, p, inv);
    let t0 = multi_square(t0, p, inv, 5);
    let t0 = mul(t0, t15, p, inv);
    let t0 = multi_square(t0, p, inv, 8);
    let t0 = mul(t0, a, p, inv);
    let t0 = multi_square(t0, p, inv, 12);
    let t0 = mul(t0, t13, p, inv);
    let t0 = multi_square(t0, p, inv, 7);
    let t0 = mul(t0, t9, p, inv);
    let t0 = multi_square(t0, p, inv, 5);
    let t0 = mul(t0, t15, p, inv);
    let t0 = multi_square(t0, p, inv, 14);
    let t0 = mul(t0, t14, p, inv);
    let t0 = multi_square(t0, p, inv, 5);
    let t0 = mul(t0, t13, p, inv);
    let t0 = multi_square(t0, p, inv, 2);
    let t0 = mul(t0, a, p, inv);
    let t0 = multi_square(t0, p, inv, 6);
    let t0 = mul(t0, a, p, inv);
    let t0 = multi_square(t0, p, inv, 9);
    let t0 = mul(t0, t7, p, inv);
    let t0 = multi_square(t0, p, inv, 6);
    let t0 = mul(t0, t12, p, inv);
    let t0 = multi_square(t0, p, inv, 8);
    let t0 = mul(t0, t11, p, inv);
    let t0 = multi_square(t0, p, inv, 3);
    let t0 = mul(t0, a, p, inv);
    let t0 = multi_square(t0, p, inv, 12);
    let t0 = mul(t0, t9, p, inv);
    let t0 = multi_square(t0, p, inv, 11);
    let t0 = mul(t0, t8, p, inv);
    let t0 = multi_square(t0, p, inv, 8);
    let t0 = mul(t0, t7, p, inv);
    let t0 = multi_square(t0, p, inv, 4);
    let t0 = mul(t0, t6, p, inv);
    let t0 = multi_square(t0, p, inv, 10);
    let t0 = mul(t0, t5, p, inv);
    let t0 = multi_square(t0, p, inv, 7);
    let t0 = mul(t0, t3, p, inv);
    let t0 = multi_square(t0, p, inv, 6);
    let t0 = mul(t0, t4, p, inv);
    let t0 = multi_square(t0, p, inv, 7);
    let t0 = mul(t0, t3, p, inv);
    let t0 = multi_square(t0, p, inv, 5);
    let t0 = mul(t0, t2, p, inv);
    let t0 = multi_square(t0, p, inv, 6);
    let t0 = mul(t0, t2, p, inv);
    let t0 = multi_square(t0, p, inv, 7);
    let t0 = mul(t0, t1, p, inv);

    Some(t0)
}

fn multi_square(a: [u64; 4], p: [u64; 4], inv: u64, num_times: usize) -> [u64; 4] {
    let mut sqrt = a.clone();
    for _ in 0..num_times {
        sqrt = square(sqrt, p, inv);
    }
    sqrt
}
