use ::libc;

pub mod pitch_h {
    /* Copyright (c) 2007-2008 CSIRO
    Copyright (c) 2007-2009 Xiph.Org Foundation
    Written by Jean-Marc Valin */
    /* *
      @file pitch.h
      @brief Pitch analysis
    */
    /*
       Redistribution and use in source and binary forms, with or without
       modification, are permitted provided that the following conditions
       are met:

       - Redistributions of source code must retain the above copyright
       notice, this list of conditions and the following disclaimer.

       - Redistributions in binary form must reproduce the above copyright
       notice, this list of conditions and the following disclaimer in the
       documentation and/or other materials provided with the distribution.

       THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
       ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
       LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
       A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
       OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
       EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
       PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
       PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
       LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
       NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
       SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
    */
    /* OPT: This is the kernel you really want to optimize. It gets used a lot
    by the prefilter and by the PLC. */
    #[inline]

    pub unsafe extern "C" fn xcorr_kernel_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut sum: *mut crate::arch_h::opus_val32,
        mut len: libc::c_int,
    ) {
        let mut j: libc::c_int = 0; /* gcc doesn't realize that y_3 can't be used uninitialized */
        let mut y_0: crate::arch_h::opus_val16 = 0.;
        let mut y_1: crate::arch_h::opus_val16 = 0.;
        let mut y_2: crate::arch_h::opus_val16 = 0.;
        let mut y_3: crate::arch_h::opus_val16 = 0.;
        y_3 = 0 as libc::c_int as crate::arch_h::opus_val16;
        let fresh0 = y;
        y = y.offset(1);
        y_0 = *fresh0;
        let fresh1 = y;
        y = y.offset(1);
        y_1 = *fresh1;
        let fresh2 = y;
        y = y.offset(1);
        y_2 = *fresh2;
        j = 0 as libc::c_int;
        while j < len - 3 as libc::c_int {
            let mut tmp: crate::arch_h::opus_val16 = 0.;
            let fresh3 = x;
            x = x.offset(1);
            tmp = *fresh3;
            let fresh4 = y;
            y = y.offset(1);
            y_3 = *fresh4;
            *sum.offset(0 as libc::c_int as isize) =
                *sum.offset(0 as libc::c_int as isize) + tmp * y_0;
            *sum.offset(1 as libc::c_int as isize) =
                *sum.offset(1 as libc::c_int as isize) + tmp * y_1;
            *sum.offset(2 as libc::c_int as isize) =
                *sum.offset(2 as libc::c_int as isize) + tmp * y_2;
            *sum.offset(3 as libc::c_int as isize) =
                *sum.offset(3 as libc::c_int as isize) + tmp * y_3;
            let fresh5 = x;
            x = x.offset(1);
            tmp = *fresh5;
            let fresh6 = y;
            y = y.offset(1);
            y_0 = *fresh6;
            *sum.offset(0 as libc::c_int as isize) =
                *sum.offset(0 as libc::c_int as isize) + tmp * y_1;
            *sum.offset(1 as libc::c_int as isize) =
                *sum.offset(1 as libc::c_int as isize) + tmp * y_2;
            *sum.offset(2 as libc::c_int as isize) =
                *sum.offset(2 as libc::c_int as isize) + tmp * y_3;
            *sum.offset(3 as libc::c_int as isize) =
                *sum.offset(3 as libc::c_int as isize) + tmp * y_0;
            let fresh7 = x;
            x = x.offset(1);
            tmp = *fresh7;
            let fresh8 = y;
            y = y.offset(1);
            y_1 = *fresh8;
            *sum.offset(0 as libc::c_int as isize) =
                *sum.offset(0 as libc::c_int as isize) + tmp * y_2;
            *sum.offset(1 as libc::c_int as isize) =
                *sum.offset(1 as libc::c_int as isize) + tmp * y_3;
            *sum.offset(2 as libc::c_int as isize) =
                *sum.offset(2 as libc::c_int as isize) + tmp * y_0;
            *sum.offset(3 as libc::c_int as isize) =
                *sum.offset(3 as libc::c_int as isize) + tmp * y_1;
            let fresh9 = x;
            x = x.offset(1);
            tmp = *fresh9;
            let fresh10 = y;
            y = y.offset(1);
            y_2 = *fresh10;
            *sum.offset(0 as libc::c_int as isize) =
                *sum.offset(0 as libc::c_int as isize) + tmp * y_3;
            *sum.offset(1 as libc::c_int as isize) =
                *sum.offset(1 as libc::c_int as isize) + tmp * y_0;
            *sum.offset(2 as libc::c_int as isize) =
                *sum.offset(2 as libc::c_int as isize) + tmp * y_1;
            *sum.offset(3 as libc::c_int as isize) =
                *sum.offset(3 as libc::c_int as isize) + tmp * y_2;
            j += 4 as libc::c_int
        }
        let fresh11 = j;
        j = j + 1;
        if fresh11 < len {
            let fresh12 = x;
            x = x.offset(1);
            let mut tmp_0: crate::arch_h::opus_val16 = *fresh12;
            let fresh13 = y;
            y = y.offset(1);
            y_3 = *fresh13;
            *sum.offset(0 as libc::c_int as isize) =
                *sum.offset(0 as libc::c_int as isize) + tmp_0 * y_0;
            *sum.offset(1 as libc::c_int as isize) =
                *sum.offset(1 as libc::c_int as isize) + tmp_0 * y_1;
            *sum.offset(2 as libc::c_int as isize) =
                *sum.offset(2 as libc::c_int as isize) + tmp_0 * y_2;
            *sum.offset(3 as libc::c_int as isize) =
                *sum.offset(3 as libc::c_int as isize) + tmp_0 * y_3
        }
        let fresh14 = j;
        j = j + 1;
        if fresh14 < len {
            let fresh15 = x;
            x = x.offset(1);
            let mut tmp_1: crate::arch_h::opus_val16 = *fresh15;
            let fresh16 = y;
            y = y.offset(1);
            y_0 = *fresh16;
            *sum.offset(0 as libc::c_int as isize) =
                *sum.offset(0 as libc::c_int as isize) + tmp_1 * y_1;
            *sum.offset(1 as libc::c_int as isize) =
                *sum.offset(1 as libc::c_int as isize) + tmp_1 * y_2;
            *sum.offset(2 as libc::c_int as isize) =
                *sum.offset(2 as libc::c_int as isize) + tmp_1 * y_3;
            *sum.offset(3 as libc::c_int as isize) =
                *sum.offset(3 as libc::c_int as isize) + tmp_1 * y_0
        }
        if j < len {
            let fresh17 = x;
            x = x.offset(1);
            let mut tmp_2: crate::arch_h::opus_val16 = *fresh17;
            let fresh18 = y;
            y = y.offset(1);
            y_1 = *fresh18;
            *sum.offset(0 as libc::c_int as isize) =
                *sum.offset(0 as libc::c_int as isize) + tmp_2 * y_2;
            *sum.offset(1 as libc::c_int as isize) =
                *sum.offset(1 as libc::c_int as isize) + tmp_2 * y_3;
            *sum.offset(2 as libc::c_int as isize) =
                *sum.offset(2 as libc::c_int as isize) + tmp_2 * y_0;
            *sum.offset(3 as libc::c_int as isize) =
                *sum.offset(3 as libc::c_int as isize) + tmp_2 * y_1
        };
    }

    use crate::arch_h::opus_val16;
}
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::src::opus_1_2_1::celt::celt_lpc::pitch_h::xcorr_kernel_c;
pub use crate::src::opus_1_2_1::celt::pitch::celt_pitch_xcorr_c;
use crate::stdlib::memset;
/* Copyright (c) 2009-2010 Xiph.Org Foundation
Written by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* Copyright (c) 2009-2010 Xiph.Org Foundation
Written by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
#[no_mangle]

pub unsafe extern "C" fn _celt_lpc(
    mut _lpc: *mut crate::arch_h::opus_val16,
    mut ac: *const crate::arch_h::opus_val32,
    mut p: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut r: crate::arch_h::opus_val32 = 0.;
    let mut error: crate::arch_h::opus_val32 = *ac.offset(0 as libc::c_int as isize);
    let mut lpc: *mut libc::c_float = _lpc;
    crate::stdlib::memset(
        lpc as *mut libc::c_void,
        0 as libc::c_int,
        (p as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    if *ac.offset(0 as libc::c_int as isize) != 0 as libc::c_int as libc::c_float {
        i = 0 as libc::c_int;
        while i < p {
            /* Sum up this iteration's reflection coefficient */
            let mut rr: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
            j = 0 as libc::c_int;
            while j < i {
                rr += *lpc.offset(j as isize) * *ac.offset((i - j) as isize);
                j += 1
            }
            rr += *ac.offset((i + 1 as libc::c_int) as isize);
            r = -(rr / error);
            /*  Update LPC coefficients and total error */
            *lpc.offset(i as isize) = r;
            j = 0 as libc::c_int;
            while j < i + 1 as libc::c_int >> 1 as libc::c_int {
                let mut tmp1: crate::arch_h::opus_val32 = 0.;
                let mut tmp2: crate::arch_h::opus_val32 = 0.;
                tmp1 = *lpc.offset(j as isize);
                tmp2 = *lpc.offset((i - 1 as libc::c_int - j) as isize);
                *lpc.offset(j as isize) = tmp1 + r * tmp2;
                *lpc.offset((i - 1 as libc::c_int - j) as isize) = tmp2 + r * tmp1;
                j += 1
            }
            error = error - r * r * error;
            /* Bail out once we get 30 dB gain */
            if error < 0.001f32 * *ac.offset(0 as libc::c_int as isize) {
                break;
            }
            i += 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn celt_fir_c(
    mut x: *const crate::arch_h::opus_val16,
    mut num: *const crate::arch_h::opus_val16,
    mut y: *mut crate::arch_h::opus_val16,
    mut N: libc::c_int,
    mut ord: libc::c_int,
    mut arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rnum: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fresh19 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(ord as libc::c_ulong) as usize,
    );
    rnum = fresh19.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    i = 0 as libc::c_int;
    while i < ord {
        *rnum.offset(i as isize) = *num.offset((ord - i - 1 as libc::c_int) as isize);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < N - 3 as libc::c_int {
        let mut sum: [crate::arch_h::opus_val32; 4] = [0.; 4];
        sum[0 as libc::c_int as usize] = *x.offset(i as isize);
        sum[1 as libc::c_int as usize] = *x.offset((i + 1 as libc::c_int) as isize);
        sum[2 as libc::c_int as usize] = *x.offset((i + 2 as libc::c_int) as isize);
        sum[3 as libc::c_int as usize] = *x.offset((i + 3 as libc::c_int) as isize);
        xcorr_kernel_c(
            rnum,
            x.offset(i as isize).offset(-(ord as isize)),
            sum.as_mut_ptr(),
            ord,
        );
        *y.offset(i as isize) = sum[0 as libc::c_int as usize];
        *y.offset((i + 1 as libc::c_int) as isize) = sum[1 as libc::c_int as usize];
        *y.offset((i + 2 as libc::c_int) as isize) = sum[2 as libc::c_int as usize];
        *y.offset((i + 3 as libc::c_int) as isize) = sum[3 as libc::c_int as usize];
        i += 4 as libc::c_int
    }
    while i < N {
        let mut sum_0: crate::arch_h::opus_val32 = *x.offset(i as isize);
        j = 0 as libc::c_int;
        while j < ord {
            sum_0 = sum_0 + *rnum.offset(j as isize) * *x.offset((i + j - ord) as isize);
            j += 1
        }
        *y.offset(i as isize) = sum_0;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn celt_iir(
    mut _x: *const crate::arch_h::opus_val32,
    mut den: *const crate::arch_h::opus_val16,
    mut _y: *mut crate::arch_h::opus_val32,
    mut N: libc::c_int,
    mut ord: libc::c_int,
    mut mem: *mut crate::arch_h::opus_val16,
    mut arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rden: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut y: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fresh20 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(ord as libc::c_ulong) as usize,
    );
    rden = fresh20.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul((N + ord) as libc::c_ulong) as usize,
    );
    y = fresh21.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    i = 0 as libc::c_int;
    while i < ord {
        *rden.offset(i as isize) = *den.offset((ord - i - 1 as libc::c_int) as isize);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < ord {
        *y.offset(i as isize) = -*mem.offset((ord - i - 1 as libc::c_int) as isize);
        i += 1
    }
    while i < N + ord {
        *y.offset(i as isize) = 0 as libc::c_int as crate::arch_h::opus_val16;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < N - 3 as libc::c_int {
        /* Unroll by 4 as if it were an FIR filter */
        let mut sum: [crate::arch_h::opus_val32; 4] = [0.; 4];
        sum[0 as libc::c_int as usize] = *_x.offset(i as isize);
        sum[1 as libc::c_int as usize] = *_x.offset((i + 1 as libc::c_int) as isize);
        sum[2 as libc::c_int as usize] = *_x.offset((i + 2 as libc::c_int) as isize);
        sum[3 as libc::c_int as usize] = *_x.offset((i + 3 as libc::c_int) as isize);
        xcorr_kernel_c(rden, y.offset(i as isize), sum.as_mut_ptr(), ord);
        /* Patch up the result to compensate for the fact that this is an IIR */
        *y.offset((i + ord) as isize) = -sum[0 as libc::c_int as usize];
        *_y.offset(i as isize) = sum[0 as libc::c_int as usize];
        sum[1 as libc::c_int as usize] = sum[1 as libc::c_int as usize]
            + *y.offset((i + ord) as isize) * *den.offset(0 as libc::c_int as isize);
        *y.offset((i + ord + 1 as libc::c_int) as isize) = -sum[1 as libc::c_int as usize];
        *_y.offset((i + 1 as libc::c_int) as isize) = sum[1 as libc::c_int as usize];
        sum[2 as libc::c_int as usize] = sum[2 as libc::c_int as usize]
            + *y.offset((i + ord + 1 as libc::c_int) as isize)
                * *den.offset(0 as libc::c_int as isize);
        sum[2 as libc::c_int as usize] = sum[2 as libc::c_int as usize]
            + *y.offset((i + ord) as isize) * *den.offset(1 as libc::c_int as isize);
        *y.offset((i + ord + 2 as libc::c_int) as isize) = -sum[2 as libc::c_int as usize];
        *_y.offset((i + 2 as libc::c_int) as isize) = sum[2 as libc::c_int as usize];
        sum[3 as libc::c_int as usize] = sum[3 as libc::c_int as usize]
            + *y.offset((i + ord + 2 as libc::c_int) as isize)
                * *den.offset(0 as libc::c_int as isize);
        sum[3 as libc::c_int as usize] = sum[3 as libc::c_int as usize]
            + *y.offset((i + ord + 1 as libc::c_int) as isize)
                * *den.offset(1 as libc::c_int as isize);
        sum[3 as libc::c_int as usize] = sum[3 as libc::c_int as usize]
            + *y.offset((i + ord) as isize) * *den.offset(2 as libc::c_int as isize);
        *y.offset((i + ord + 3 as libc::c_int) as isize) = -sum[3 as libc::c_int as usize];
        *_y.offset((i + 3 as libc::c_int) as isize) = sum[3 as libc::c_int as usize];
        i += 4 as libc::c_int
    }
    while i < N {
        let mut sum_0: crate::arch_h::opus_val32 = *_x.offset(i as isize);
        j = 0 as libc::c_int;
        while j < ord {
            sum_0 -= *rden.offset(j as isize) * *y.offset((i + j) as isize);
            j += 1
        }
        *y.offset((i + ord) as isize) = sum_0;
        *_y.offset(i as isize) = sum_0;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < ord {
        *mem.offset(i as isize) = *_y.offset((N - i - 1 as libc::c_int) as isize);
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn _celt_autocorr(
    mut x: *const crate::arch_h::opus_val16,
    mut ac: *mut crate::arch_h::opus_val32,
    mut window: *const crate::arch_h::opus_val16,
    mut overlap: libc::c_int,
    mut lag: libc::c_int,
    mut n: libc::c_int,
    mut arch: libc::c_int,
) -> libc::c_int {
    let mut d: crate::arch_h::opus_val32 = 0.;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut fastN: libc::c_int = n - lag;
    let mut shift: libc::c_int = 0;
    let mut xptr: *const crate::arch_h::opus_val16 = 0 as *const crate::arch_h::opus_val16;
    let mut xx: *mut crate::arch_h::opus_val16 = 0 as *mut crate::arch_h::opus_val16;
    let mut fresh22 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::opus_val16>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong) as usize,
    );
    xx = fresh22.as_mut_ptr() as *mut crate::arch_h::opus_val16;
    if overlap == 0 as libc::c_int {
        xptr = x
    } else {
        i = 0 as libc::c_int;
        while i < n {
            *xx.offset(i as isize) = *x.offset(i as isize);
            i += 1
        }
        i = 0 as libc::c_int;
        while i < overlap {
            *xx.offset(i as isize) = *x.offset(i as isize) * *window.offset(i as isize);
            *xx.offset((n - i - 1 as libc::c_int) as isize) =
                *x.offset((n - i - 1 as libc::c_int) as isize) * *window.offset(i as isize);
            i += 1
        }
        xptr = xx
    }
    shift = 0 as libc::c_int;
    crate::src::opus_1_2_1::celt::pitch::celt_pitch_xcorr_c(
        xptr,
        xptr,
        ac,
        fastN,
        lag + 1 as libc::c_int,
        arch,
    );
    k = 0 as libc::c_int;
    while k <= lag {
        i = k + fastN;
        d = 0 as libc::c_int as crate::arch_h::opus_val32;
        while i < n {
            d = d + *xptr.offset(i as isize) * *xptr.offset((i - k) as isize);
            i += 1
        }
        let ref mut fresh23 = *ac.offset(k as isize);
        *fresh23 += d;
        k += 1
    }
    return shift;
}
