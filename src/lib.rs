#![cfg_attr(feature = "simd", feature(portable_simd))]
#![feature(const_fn_floating_point_arithmetic)]

pub mod common;

/// H.264 level restriction information
pub struct H264Level {
    pub level_idc: u8,
    pub mbps: u32,       /* max macroblock processing rate (macroblocks/sec) */
    pub frame_size: u32, /* max frame size (macroblocks) */
    pub dpb: u32,        /* max decoded picture buffer (mbs) */
    pub bitrate: u32,    /* max bitrate (kbit/sec) */
    pub cpb: u32,        /* max vbv buffer (kbit) */
    pub mv_range: u16,   /* max vertical mv component range (pixels) */
    pub mvs_per_2mb: u8, /* max mvs per 2 consecutive mbs. */
    pub slice_rate: u8,  /* ?? */
    pub mincr: u8,       /* min compression ratio */
    pub bipred8x8: u8,   /* limit bipred to >=8x8 */
    pub direct8x8: u8,   /* limit b_direct to >=8x8 */
    pub frame_only: u8,  /* forbid interlacing */
}

impl H264Level {
    pub const fn new(
        level_idc: u8,
        mbps: u32,       /* max macroblock processing rate (macroblocks/sec) */
        frame_size: u32, /* max frame size (macroblocks) */
        dpb: u32,        /* max decoded picture buffer (mbs) */
        bitrate: u32,    /* max bitrate (kbit/sec) */
        cpb: u32,        /* max vbv buffer (kbit) */
        mv_range: u16,   /* max vertical mv component range (pixels) */
        mvs_per_2mb: u8, /* max mvs per 2 consecutive mbs. */
        slice_rate: u8,  /* ?? */
        mincr: u8,       /* min compression ratio */
        bipred8x8: u8,   /* limit bipred to >=8x8 */
        direct8x8: u8,   /* limit b_direct to >=8x8 */
        frame_only: u8,  /* forbid interlacing */
    ) -> Self {
        Self {
            level_idc,
            mbps,
            frame_size,
            dpb,
            bitrate,
            cpb,
            mv_range,
            mvs_per_2mb,
            slice_rate,
            mincr,
            bipred8x8,
            direct8x8,
            frame_only,
        }
    }
}
