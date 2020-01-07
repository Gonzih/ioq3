use ::libc;

pub mod macros_h {
    #[inline]

    pub unsafe extern "C" fn silk_CLZ32(
        mut in32: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if in32 != 0 {
            (32 as libc::c_int)
                - (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                    * 8 as libc::c_int
                    - (in32 as libc::c_uint).leading_zeros() as i32)
        } else {
            32 as libc::c_int
        };
    }

    /* SILK_MACROS_H */
    /* Column based */
    /* Row based */
}

pub mod Inlines_h {
    /* **********************************************************************
    Copyright (c) 2006-2011, Skype Limited. All rights reserved.
    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions
    are met:
    - Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.
    - Redistributions in binary form must reproduce the above copyright
    notice, this list of conditions and the following disclaimer in the
    documentation and/or other materials provided with the distribution.
    - Neither the name of Internet Society, IETF or IETF Trust, nor the
    names of specific contributors, may be used to endorse or promote
    products derived from this software without specific prior written
    permission.
    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
    IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
    ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
    LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
    CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
    SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
    INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
    CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
    ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
    POSSIBILITY OF SUCH DAMAGE.
    ***********************************************************************/
    /* ! \file silk_Inlines.h
     *  \brief silk_Inlines.h defines OPUS_INLINE signal processing functions.
     */
    /* count leading zeros of opus_int64 */
    /* Search in the lower 32 bits */
    /* Search in the upper 32 bits */
    /* get number of leading zeros and fractional part (the bits right after the leading one */
    /* I  input                               */
    /* O  number of leading zeros             */
    /* O  the 7 bits right after the leading one */
    /* Approximation of square root                                          */
    /* Accuracy: < +/- 10%  for output values > 15                           */
    /*           < +/- 2.5% for output values > 120                          */
    /* 46214 = sqrt(2) * 32768 */
    /* get scaling right */
    /* increment using fractional part of input */
    /* Divide two int32 values and return result as int32 in a given Q-domain */
    /* O    returns a good approximation of "(a32 << Qres) / b32" */
    /* I    numerator (Q0)                  */
    /* I    denominator (Q0)                */
    /* I    Q-domain of result (>= 0)       */
    /* Compute number of bits head room and normalize inputs */
    /* Q: a_headrm                  */
    /* Q: b_headrm                  */
    /* Inverse of b32, with 14 bits of precision */
    /* Q: 29 + 16 - b_headrm        */
    /* First approximation */
    /* Q: 29 + a_headrm - b_headrm  */
    /* Compute residual by subtracting product of denominator and first approximation */
    /* It's OK to overflow because the final value of a32_nrm should always be small */
    /* Q: a_headrm   */
    /* Refinement */
    /* Q: 29 + a_headrm - b_headrm  */
    /* Convert to Qres domain */
    /* Avoid undefined result */
    /* Invert int32 value and return result as int32 in a given Q-domain */
    #[inline]

    pub unsafe extern "C" fn silk_INVERSE32_varQ(
        b32: crate::opus_types_h::opus_int32,
        Qres: libc::c_int,
    ) -> crate::opus_types_h::opus_int32
/* I    Q-domain of result (> 0)        */ {
        let mut b_headrm: libc::c_int = 0;
        let mut lshift: libc::c_int = 0;
        let mut b32_inv: crate::opus_types_h::opus_int32 = 0;
        let mut b32_nrm: crate::opus_types_h::opus_int32 = 0;
        let mut err_Q32: crate::opus_types_h::opus_int32 = 0;
        let mut result: crate::opus_types_h::opus_int32 = 0;
        /* Compute number of bits head room and normalize input */
        b_headrm = silk_CLZ32((if b32 > 0 as libc::c_int { b32 } else { -b32 })) - 1 as libc::c_int; /* Q: b_headrm                */
        b32_nrm = ((b32 as crate::opus_types_h::opus_uint32) << b_headrm)
            as crate::opus_types_h::opus_int32;
        /* Inverse of b32, with 14 bits of precision */
        b32_inv = (0x7fffffff as libc::c_int >> 2 as libc::c_int) / (b32_nrm >> 16 as libc::c_int); /* Q: 29 + 16 - b_headrm    */
        /* First approximation */
        result = ((b32_inv as crate::opus_types_h::opus_uint32) << 16 as libc::c_int)
            as crate::opus_types_h::opus_int32; /* Q: 61 - b_headrm            */
        /* Compute residual by subtracting product of denominator and first approximation from one */
        err_Q32 = (((((1 as libc::c_int) << 29 as libc::c_int)
            - (b32_nrm as libc::c_longlong
                * b32_inv as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int) as crate::opus_types_h::opus_int32)
            as crate::opus_types_h::opus_uint32)
            << 3 as libc::c_int) as crate::opus_types_h::opus_int32; /* Q32                        */
        /* Refinement */
        result = (result as libc::c_longlong
            + (err_Q32 as libc::c_longlong * b32_inv as libc::c_longlong >> 16 as libc::c_int))
            as crate::opus_types_h::opus_int32; /* Q: 61 - b_headrm            */
        /* Convert to Qres domain */
        lshift = 61 as libc::c_int - b_headrm - Qres;
        if lshift <= 0 as libc::c_int {
            return (((if 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32 >> -lshift
                > 0x7fffffff as libc::c_int >> -lshift
            {
                (if result
                    > 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32 >> -lshift
                {
                    (0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32) >> -lshift
                } else {
                    (if result < 0x7fffffff as libc::c_int >> -lshift {
                        (0x7fffffff as libc::c_int) >> -lshift
                    } else {
                        result
                    })
                })
            } else {
                (if result > 0x7fffffff as libc::c_int >> -lshift {
                    (0x7fffffff as libc::c_int) >> -lshift
                } else {
                    (if result
                        < 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32 >> -lshift
                    {
                        (0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32) >> -lshift
                    } else {
                        result
                    })
                })
            }) as crate::opus_types_h::opus_uint32)
                << -lshift) as crate::opus_types_h::opus_int32;
        } else if lshift < 32 as libc::c_int {
            return result >> lshift;
        } else {
            /* Avoid undefined result */
            return 0 as libc::c_int;
        };
    }
    use crate::opus_types_h::opus_int16;
    use crate::opus_types_h::opus_int32;
    use crate::opus_types_h::opus_uint32;
    use crate::src::opus_1_2_1::silk::LPC_inv_pred_gain::macros_h::silk_CLZ32;
    /* SILK_FIX_INLINES_H */
}

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::silk::LPC_inv_pred_gain::macros_h::silk_CLZ32;
pub use crate::src::opus_1_2_1::silk::LPC_inv_pred_gain::Inlines_h::silk_INVERSE32_varQ;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;
/* Compute inverse of LPC prediction gain, and                          */
/* test if LPC coefficients are stable (all poles within unit circle)   */

unsafe extern "C" fn LPC_inverse_pred_gain_QA_c(
    mut A_QA: *mut crate::opus_types_h::opus_int32,
    order: libc::c_int,
) -> crate::opus_types_h::opus_int32
/* I   Prediction order                                         */ {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mult2Q: libc::c_int = 0;
    let mut invGain_Q30: crate::opus_types_h::opus_int32 = 0;
    let mut rc_Q31: crate::opus_types_h::opus_int32 = 0;
    let mut rc_mult1_Q30: crate::opus_types_h::opus_int32 = 0;
    let mut rc_mult2: crate::opus_types_h::opus_int32 = 0;
    let mut tmp1: crate::opus_types_h::opus_int32 = 0;
    let mut tmp2: crate::opus_types_h::opus_int32 = 0;
    invGain_Q30 = ((1 as libc::c_int as libc::c_longlong
        * ((1 as libc::c_int as libc::c_longlong) << 30 as libc::c_int))
        as libc::c_double
        + 0.5f64) as crate::opus_types_h::opus_int32;
    k = order - 1 as libc::c_int;
    while k > 0 as libc::c_int {
        /* Check for stability */
        if *A_QA.offset(k as isize)
            > (0.99975f64
                * ((1 as libc::c_int as libc::c_longlong) << 24 as libc::c_int) as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
            || *A_QA.offset(k as isize)
                < -((0.99975f64
                    * ((1 as libc::c_int as libc::c_longlong) << 24 as libc::c_int)
                        as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32)
        {
            return 0 as libc::c_int;
        }
        /* Set RC equal to negated AR coef */
        rc_Q31 = -(((*A_QA.offset(k as isize) as crate::opus_types_h::opus_uint32)
            << 31 as libc::c_int - 24 as libc::c_int)
            as crate::opus_types_h::opus_int32);
        /* rc_mult1_Q30 range: [ 1 : 2^30 ] */
        rc_mult1_Q30 = ((1 as libc::c_int as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 30 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
            - (rc_Q31 as libc::c_longlong * rc_Q31 as libc::c_longlong >> 32 as libc::c_int)
                as crate::opus_types_h::opus_int32;
        /* reduce A_LIMIT if fails */
        /* Update inverse gain */
        /* invGain_Q30 range: [ 0 : 2^30 ] */
        invGain_Q30 = (((invGain_Q30 as libc::c_longlong * rc_mult1_Q30 as libc::c_longlong
            >> 32 as libc::c_int) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_uint32)
            << 2 as libc::c_int) as crate::opus_types_h::opus_int32;
        if invGain_Q30
            < ((1.0f32 / 1e4f32
                * ((1 as libc::c_int as libc::c_longlong) << 30 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
        {
            return 0 as libc::c_int;
        }
        mult2Q = 32 as libc::c_int
            - silk_CLZ32(
                (if rc_mult1_Q30 > 0 as libc::c_int {
                    rc_mult1_Q30
                } else {
                    -rc_mult1_Q30
                }),
            );
        rc_mult2 = silk_INVERSE32_varQ(rc_mult1_Q30, mult2Q + 30 as libc::c_int);
        n = 0 as libc::c_int;
        while n < k + 1 as libc::c_int >> 1 as libc::c_int {
            let mut tmp64: libc::c_longlong = 0;
            tmp1 = *A_QA.offset(n as isize);
            tmp2 = *A_QA.offset((k - n - 1 as libc::c_int) as isize);
            tmp64 = if mult2Q == 1 as libc::c_int {
                ((if (tmp1 as crate::opus_types_h::opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong >> 1 as libc::c_int)
                            + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                & 1 as libc::c_int as libc::c_longlong)
                    } else {
                        ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_longlong)
                            >> 1 as libc::c_int
                    }) as crate::opus_types_h::opus_int32
                        as crate::opus_types_h::opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    (if tmp1 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                    } else {
                        (tmp1)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                } else {
                    (if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        (tmp1)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                }) as libc::c_longlong
                    * rc_mult2 as libc::c_longlong
                    >> 1 as libc::c_int)
                    + ((if (tmp1 as crate::opus_types_h::opus_uint32).wrapping_sub(
                        (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as crate::opus_types_h::opus_uint32,
                    ) & 0x80000000 as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        (if tmp1 as libc::c_uint
                            & ((if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                                as libc::c_uint
                                ^ 0x80000000 as libc::c_uint)
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                        } else {
                            (tmp1)
                                - (if 31 as libc::c_int == 1 as libc::c_int {
                                    (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 1 as libc::c_int)
                                        + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                            & 1 as libc::c_int as libc::c_longlong)
                                } else {
                                    ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 31 as libc::c_int - 1 as libc::c_int)
                                        + 1 as libc::c_int as libc::c_longlong)
                                        >> 1 as libc::c_int
                                })
                                    as crate::opus_types_h::opus_int32
                        })
                    } else {
                        (if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                            & (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                                as libc::c_uint
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x7fffffff as libc::c_int
                        } else {
                            (tmp1)
                                - (if 31 as libc::c_int == 1 as libc::c_int {
                                    (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 1 as libc::c_int)
                                        + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                            & 1 as libc::c_int as libc::c_longlong)
                                } else {
                                    ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 31 as libc::c_int - 1 as libc::c_int)
                                        + 1 as libc::c_int as libc::c_longlong)
                                        >> 1 as libc::c_int
                                })
                                    as crate::opus_types_h::opus_int32
                        })
                    }) as libc::c_longlong
                        * rc_mult2 as libc::c_longlong
                        & 1 as libc::c_int as libc::c_longlong)
            } else {
                (((if (tmp1 as crate::opus_types_h::opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong >> 1 as libc::c_int)
                            + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                & 1 as libc::c_int as libc::c_longlong)
                    } else {
                        ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_longlong)
                            >> 1 as libc::c_int
                    }) as crate::opus_types_h::opus_int32
                        as crate::opus_types_h::opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    (if tmp1 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                    } else {
                        (tmp1)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                } else {
                    (if (tmp1 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        (tmp1)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp2 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                }) as libc::c_longlong
                    * rc_mult2 as libc::c_longlong
                    >> mult2Q - 1 as libc::c_int)
                    + 1 as libc::c_int as libc::c_longlong)
                    >> 1 as libc::c_int
            };
            if tmp64 > 0x7fffffff as libc::c_int as libc::c_longlong
                || tmp64
                    < 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                        as libc::c_longlong
            {
                return 0 as libc::c_int;
            }
            *A_QA.offset(n as isize) = tmp64 as crate::opus_types_h::opus_int32;
            tmp64 = if mult2Q == 1 as libc::c_int {
                ((if (tmp2 as crate::opus_types_h::opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong >> 1 as libc::c_int)
                            + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                & 1 as libc::c_int as libc::c_longlong)
                    } else {
                        ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_longlong)
                            >> 1 as libc::c_int
                    }) as crate::opus_types_h::opus_int32
                        as crate::opus_types_h::opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    (if tmp2 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                    } else {
                        (tmp2)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                } else {
                    (if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        (tmp2)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                }) as libc::c_longlong
                    * rc_mult2 as libc::c_longlong
                    >> 1 as libc::c_int)
                    + ((if (tmp2 as crate::opus_types_h::opus_uint32).wrapping_sub(
                        (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as crate::opus_types_h::opus_uint32,
                    ) & 0x80000000 as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        (if tmp2 as libc::c_uint
                            & ((if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                                as libc::c_uint
                                ^ 0x80000000 as libc::c_uint)
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                        } else {
                            (tmp2)
                                - (if 31 as libc::c_int == 1 as libc::c_int {
                                    (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 1 as libc::c_int)
                                        + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                            & 1 as libc::c_int as libc::c_longlong)
                                } else {
                                    ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 31 as libc::c_int - 1 as libc::c_int)
                                        + 1 as libc::c_int as libc::c_longlong)
                                        >> 1 as libc::c_int
                                })
                                    as crate::opus_types_h::opus_int32
                        })
                    } else {
                        (if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                            & (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                                as libc::c_uint
                            & 0x80000000 as libc::c_uint
                            != 0
                        {
                            0x7fffffff as libc::c_int
                        } else {
                            (tmp2)
                                - (if 31 as libc::c_int == 1 as libc::c_int {
                                    (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 1 as libc::c_int)
                                        + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                            & 1 as libc::c_int as libc::c_longlong)
                                } else {
                                    ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        >> 31 as libc::c_int - 1 as libc::c_int)
                                        + 1 as libc::c_int as libc::c_longlong)
                                        >> 1 as libc::c_int
                                })
                                    as crate::opus_types_h::opus_int32
                        })
                    }) as libc::c_longlong
                        * rc_mult2 as libc::c_longlong
                        & 1 as libc::c_int as libc::c_longlong)
            } else {
                (((if (tmp2 as crate::opus_types_h::opus_uint32).wrapping_sub(
                    (if 31 as libc::c_int == 1 as libc::c_int {
                        (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong >> 1 as libc::c_int)
                            + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                & 1 as libc::c_int as libc::c_longlong)
                    } else {
                        ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                            >> 31 as libc::c_int - 1 as libc::c_int)
                            + 1 as libc::c_int as libc::c_longlong)
                            >> 1 as libc::c_int
                    }) as crate::opus_types_h::opus_int32
                        as crate::opus_types_h::opus_uint32,
                ) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    (if tmp2 as libc::c_uint
                        & ((if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                            ^ 0x80000000 as libc::c_uint)
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                    } else {
                        (tmp2)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                } else {
                    (if (tmp2 as libc::c_uint ^ 0x80000000 as libc::c_uint)
                        & (if 31 as libc::c_int == 1 as libc::c_int {
                            (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 1 as libc::c_int)
                                + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    & 1 as libc::c_int as libc::c_longlong)
                        } else {
                            ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                >> 31 as libc::c_int - 1 as libc::c_int)
                                + 1 as libc::c_int as libc::c_longlong)
                                >> 1 as libc::c_int
                        }) as crate::opus_types_h::opus_int32
                            as libc::c_uint
                        & 0x80000000 as libc::c_uint
                        != 0
                    {
                        0x7fffffff as libc::c_int
                    } else {
                        (tmp2)
                            - (if 31 as libc::c_int == 1 as libc::c_int {
                                (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 1 as libc::c_int)
                                    + (tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                        & 1 as libc::c_int as libc::c_longlong)
                            } else {
                                ((tmp1 as libc::c_longlong * rc_Q31 as libc::c_longlong
                                    >> 31 as libc::c_int - 1 as libc::c_int)
                                    + 1 as libc::c_int as libc::c_longlong)
                                    >> 1 as libc::c_int
                            }) as crate::opus_types_h::opus_int32
                    })
                }) as libc::c_longlong
                    * rc_mult2 as libc::c_longlong
                    >> mult2Q - 1 as libc::c_int)
                    + 1 as libc::c_int as libc::c_longlong)
                    >> 1 as libc::c_int
            };
            if tmp64 > 0x7fffffff as libc::c_int as libc::c_longlong
                || tmp64
                    < 0x80000000 as libc::c_uint as crate::opus_types_h::opus_int32
                        as libc::c_longlong
            {
                return 0 as libc::c_int;
            }
            *A_QA.offset((k - n - 1 as libc::c_int) as isize) =
                tmp64 as crate::opus_types_h::opus_int32;
            n += 1
        }
        k -= 1
    }
    /* rc_mult2 range: [ 2^30 : silk_int32_MAX ] */
    /* Update AR coefficient */
    /* Check for stability */
    if *A_QA.offset(k as isize)
        > (0.99975f64
            * ((1 as libc::c_int as libc::c_longlong) << 24 as libc::c_int) as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
        || *A_QA.offset(k as isize)
            < -((0.99975f64
                * ((1 as libc::c_int as libc::c_longlong) << 24 as libc::c_int) as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32)
    {
        return 0 as libc::c_int;
    }
    /* Set RC equal to negated AR coef */
    rc_Q31 = -(((*A_QA.offset(0 as libc::c_int as isize) as crate::opus_types_h::opus_uint32)
        << 31 as libc::c_int - 24 as libc::c_int)
        as crate::opus_types_h::opus_int32);
    /* Range: [ 1 : 2^30 ] */
    rc_mult1_Q30 = ((1 as libc::c_int as libc::c_longlong
        * ((1 as libc::c_int as libc::c_longlong) << 30 as libc::c_int))
        as libc::c_double
        + 0.5f64) as crate::opus_types_h::opus_int32
        - (rc_Q31 as libc::c_longlong * rc_Q31 as libc::c_longlong >> 32 as libc::c_int)
            as crate::opus_types_h::opus_int32;
    /* Update inverse gain */
    /* Range: [ 0 : 2^30 ] */
    invGain_Q30 = (((invGain_Q30 as libc::c_longlong * rc_mult1_Q30 as libc::c_longlong
        >> 32 as libc::c_int) as crate::opus_types_h::opus_int32
        as crate::opus_types_h::opus_uint32)
        << 2 as libc::c_int) as crate::opus_types_h::opus_int32;
    if invGain_Q30
        < ((1.0f32 / 1e4f32
            * ((1 as libc::c_int as libc::c_longlong) << 30 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
    {
        return 0 as libc::c_int;
    }
    return invGain_Q30;
}
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/*#define silk_MACRO_COUNT */
/* Used to enable WMOPS counting */
/* max order of the LPC analysis in schur() and k2a() */
/* for memset(), memcpy(), memmove() */
/* *******************************************************************/
/*                    SIGNAL PROCESSING FUNCTIONS                   */
/* *******************************************************************/
/* !
 * Initialize/reset the resampler state for a given pair of input/output sampling rates
*/
/* I/O  Resampler state                                             */
/* I    Input sampling rate (Hz)                                    */
/* I    Output sampling rate (Hz)                                   */
/* I    If 1: encoder; if 0: decoder                                */
/* !
 * Resampler: convert from one sampling rate to another
 */
/* I/O  Resampler state                                             */
/* O    Output signal                                               */
/* I    Input signal                                                */
/* I    Number of input samples                                     */
/* !
* Downsample 2x, mediocre quality
*/
/* I/O  State vector [ 2 ]                                          */
/* O    Output signal [ len ]                                       */
/* I    Input signal [ floor(len/2) ]                               */
/* I    Number of input samples                                     */
/* !
 * Downsample by a factor 2/3, low quality
*/
/* I/O  State vector [ 6 ]                                          */
/* O    Output signal [ floor(2*inLen/3) ]                          */
/* I    Input signal [ inLen ]                                      */
/* I    Number of input samples                                     */
/* !
 * second order ARMA filter;
 * slower than biquad() but uses more precise coefficients
 * can handle (slowly) varying coefficients
 */
/* I     input signal                                               */
/* I     MA coefficients [3]                                        */
/* I     AR coefficients [2]                                        */
/* I/O   State vector [2]                                           */
/* O     output signal                                              */
/* I     signal length (must be even)                               */
/* I     input signal                                               */
/* I     MA coefficients [3]                                        */
/* I     AR coefficients [2]                                        */
/* I/O   State vector [4]                                           */
/* O     output signal                                              */
/* I     signal length (must be even)                               */
/* Variable order MA prediction error filter. */
/* O    Output signal                                               */
/* I    Input signal                                                */
/* I    MA prediction coefficients, Q12 [order]                     */
/* I    Signal length                                               */
/* I    Filter order                                                */
/* I    Run-time architecture                                       */
/* Chirp (bandwidth expand) LP AR filter */
/* I/O  AR filter to be expanded (without leading 1)                */
/* I    Length of ar                                                */
/* I    Chirp factor (typically in the range 0 to 1)                */
/* Chirp (bandwidth expand) LP AR filter */
/* I/O  AR filter to be expanded (without leading 1)                */
/* I    Length of ar                                                */
/* I    Chirp factor in Q16                                         */
/* Compute inverse of LPC prediction gain, and                           */
/* test if LPC coefficients are stable (all poles within unit circle)    */
/* For input in Q12 domain */
#[no_mangle]

pub unsafe extern "C" fn silk_LPC_inverse_pred_gain_c(
    mut A_Q12: *const crate::opus_types_h::opus_int16,
    order: libc::c_int,
) -> crate::opus_types_h::opus_int32
/* I   Prediction order                                             */ {
    let mut k: libc::c_int = 0;
    let mut Atmp_QA: [crate::opus_types_h::opus_int32; 24] = [0; 24];
    let mut DC_resp: crate::opus_types_h::opus_int32 = 0 as libc::c_int;
    /* Increase Q domain of the AR coefficients */
    k = 0 as libc::c_int;
    while k < order {
        DC_resp += *A_Q12.offset(k as isize) as crate::opus_types_h::opus_int32;
        Atmp_QA[k as usize] = ((*A_Q12.offset(k as isize) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_uint32)
            << 24 as libc::c_int - 12 as libc::c_int)
            as crate::opus_types_h::opus_int32;
        k += 1
    }
    /* If the DC is unstable, we don't even need to do the full calculations */
    if DC_resp >= 4096 as libc::c_int {
        return 0 as libc::c_int;
    }
    return LPC_inverse_pred_gain_QA_c(Atmp_QA.as_mut_ptr(), order);
}
