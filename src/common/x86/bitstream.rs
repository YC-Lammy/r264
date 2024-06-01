use std::arch::global_asm;


global_asm!(include_str!("./bitstream-a.asm"));

extern "C" {
    pub fn x264_nal_escape_mmx2(dst: *mut u8, src: *const u8, end: *const u8) -> *mut u8;
    pub fn x264_nal_escape_sse2(dst: *mut u8, src: *mut u8, end: *mut u8) -> *mut u8;
    pub fn u8x264_nal_escape_avx2(dst: *mut u8, src: *mut u8, end: *mut u8) -> *mut u8;
    pub fn x264_cabac_block_residual_rd_internal_sse2(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_rd_internal_lzcnt(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_rd_internal_ssse3(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_rd_internal_ssse3_lzcnt(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_rd_internal_avx512(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_8x8_rd_internal_sse2(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_8x8_rd_internal_lzcnt(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_8x8_rd_internal_ssse3(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_8x8_rd_internal_ssse3_lzcnt(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_8x8_rd_internal_avx512(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_internal_sse2(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_internal_lzcnt(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_internal_avx2(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );
    pub fn x264_cabac_block_residual_internal_avx512(
        l: *mut dctcoef,
        b_interlaced: i32,
        ctx_block_cat: isize,
        cb: *mut x264_cabac_t,
    );

}
