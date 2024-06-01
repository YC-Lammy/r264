pub const fn fix8(f: f32) -> i32 {
    (f * 256. + 0.5) as i32
}

pub enum Profile {
    Baseline = 66,
    Main = 77,
    High = 100,
    High10 = 110,
    High422 = 122,
    High444Predictive = 244,
}

pub enum ChromaFormat {
    Chroma400 = 0,
    Chroma420 = 1,
    Chroma422 = 2,
    Chroma444 = 3,
}

pub enum SliceType {
    P = 0,
    B = 1,
    I = 2,
}

pub enum SeiPayloadType {
    BufferingPeriod = 0,
    PicTiming = 1,
    PanScanRect = 2,
    Filler = 3,
    UserDataRegistered = 4,
    UserDataUnregistered = 5,
    RecoveryPoint = 6,
    DecRefPicMarking = 7,
    FramePacking = 45,
    MasteringDisplay = 137,
    ContentLightLevel = 144,
    AlternativeTransfer = 147,
}

pub const BFRAME_MAC: usize = 16;
pub const BFRAME_MAX: usize = 16;
pub const REF_MAX: usize = 16;
pub const X264_THREAD_MAX: usize = 128;
pub const X264_LOOKAHEAD_THREAD_MAX: usize = 16;
pub const X264_LOOKAHEAD_MAX: usize = 250;

/// number of pixels (per thread) in progress at any given time.
/// 16 for the macroblock in progress + 3 for deblocking + 3 for motion compensation filter + 2 for extra safety
pub const THREAD_HEIGHT: usize = 24;

pub const SCAN8: [u8; 16 * 3 + 3] = [
    4 + 1 * 8,
    5 + 1 * 8,
    4 + 2 * 8,
    5 + 2 * 8,
    6 + 1 * 8,
    7 + 1 * 8,
    6 + 2 * 8,
    7 + 2 * 8,
    4 + 3 * 8,
    5 + 3 * 8,
    4 + 4 * 8,
    5 + 4 * 8,
    6 + 3 * 8,
    7 + 3 * 8,
    6 + 4 * 8,
    7 + 4 * 8,
    4 + 6 * 8,
    5 + 6 * 8,
    4 + 7 * 8,
    5 + 7 * 8,
    6 + 6 * 8,
    7 + 6 * 8,
    6 + 7 * 8,
    7 + 7 * 8,
    4 + 8 * 8,
    5 + 8 * 8,
    4 + 9 * 8,
    5 + 9 * 8,
    6 + 8 * 8,
    7 + 8 * 8,
    6 + 9 * 8,
    7 + 9 * 8,
    4 + 11 * 8,
    5 + 11 * 8,
    4 + 12 * 8,
    5 + 12 * 8,
    6 + 11 * 8,
    7 + 11 * 8,
    6 + 12 * 8,
    7 + 12 * 8,
    4 + 13 * 8,
    5 + 13 * 8,
    4 + 14 * 8,
    5 + 14 * 8,
    6 + 13 * 8,
    7 + 13 * 8,
    6 + 14 * 8,
    7 + 14 * 8,
    0 + 0 * 8,
    0 + 5 * 8,
    0 + 10 * 8,
];
