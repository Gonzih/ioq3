pub type DCTELEM = i32;
pub type float_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: *mut f32,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
/* 16 or 32 bits is fine */
pub type forward_DCT_method_ptr = Option<
    unsafe extern "C" fn(
        _: *mut crate::jdct_h::DCTELEM,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jmorecfg_h::JDIMENSION,
    ) -> (),
>;
/* 16 bits is OK, use short if faster */

/* fractional bits in scale factors */
pub type FLOAT_MULT_TYPE = f32;
/* short or int, whichever is faster */
pub type IFAST_MULT_TYPE = i32;
/*
 * An inverse DCT routine is given a pointer to the input JBLOCK and a pointer
 * to an output sample array.  The routine must dequantize the input data as
 * well as perform the IDCT; for dequantization, it uses the multiplier table
 * pointed to by compptr->dct_table.  The output data is to be placed into the
 * sample array starting at a specified column.  (Any row offset needed will
 * be applied to the array pointer before it is passed to the IDCT code.)
 * Note that the number of samples emitted by the IDCT routine is
 * DCT_h_scaled_size * DCT_v_scaled_size.
 */

/* typedef inverse_DCT_method_ptr is declared in jpegint.h */

/*
 * Each IDCT routine has its own ideas about the best dct_table element type.
 */
pub type ISLOW_MULT_TYPE = i32;
