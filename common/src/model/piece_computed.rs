use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::PIECE_NUM_ROTATION;
use crate::model::consts::PIECE_NUM_TYPES;
use crate::model::consts::PIECE_SHAPE_SIZE;
use crate::model::consts::PIECE_SPAWN_COLUMN;
use crate::model::consts::{BOARD_WIDTH, PIECE_MAX_X_SHIFT};
// use std::lazy::SyncLazy;

// pub static PIECE_INFO: SyncLazy<PieceInfo> = SyncLazy::new(|| PieceInfo::new());

// Manually generated using PieceInfo::new()
pub const PIECE_INFO: PieceInfo = PieceInfo {
    spawn_locations: [
        (3, 20),
        (3, 19),
        (3, 20),
        (3, 20),
        (3, 20),
        (3, 20),
        (3, 20),
    ],
    shapes: [
        [
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
        ],
        [
            [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
        ],
        [
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        ],
        [
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            [
                [true, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        ],
        [
            [
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            [
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        ],
        [
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            [
                [true, false, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        ],
        [
            [
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [true, true, false, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            [
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
        ],
    ],
    bit_shapes: [
        [
            [
                [0, 1, 1, 0],
                [0, 3, 3, 0],
                [0, 6, 6, 0],
                [0, 12, 12, 0],
                [0, 24, 24, 0],
                [0, 48, 48, 0],
                [0, 96, 96, 0],
                [0, 192, 192, 0],
                [0, 384, 384, 0],
                [0, 768, 768, 0],
                [0, 512, 512, 0],
            ],
            [
                [0, 1, 1, 0],
                [0, 3, 3, 0],
                [0, 6, 6, 0],
                [0, 12, 12, 0],
                [0, 24, 24, 0],
                [0, 48, 48, 0],
                [0, 96, 96, 0],
                [0, 192, 192, 0],
                [0, 384, 384, 0],
                [0, 768, 768, 0],
                [0, 512, 512, 0],
            ],
            [
                [0, 1, 1, 0],
                [0, 3, 3, 0],
                [0, 6, 6, 0],
                [0, 12, 12, 0],
                [0, 24, 24, 0],
                [0, 48, 48, 0],
                [0, 96, 96, 0],
                [0, 192, 192, 0],
                [0, 384, 384, 0],
                [0, 768, 768, 0],
                [0, 512, 512, 0],
            ],
            [
                [0, 1, 1, 0],
                [0, 3, 3, 0],
                [0, 6, 6, 0],
                [0, 12, 12, 0],
                [0, 24, 24, 0],
                [0, 48, 48, 0],
                [0, 96, 96, 0],
                [0, 192, 192, 0],
                [0, 384, 384, 0],
                [0, 768, 768, 0],
                [0, 512, 512, 0],
            ],
        ],
        [
            [
                [0, 0, 3, 0],
                [0, 0, 7, 0],
                [0, 0, 15, 0],
                [0, 0, 30, 0],
                [0, 0, 60, 0],
                [0, 0, 120, 0],
                [0, 0, 240, 0],
                [0, 0, 480, 0],
                [0, 0, 960, 0],
                [0, 0, 896, 0],
                [0, 0, 768, 0],
            ],
            [
                [1, 1, 1, 1],
                [2, 2, 2, 2],
                [4, 4, 4, 4],
                [8, 8, 8, 8],
                [16, 16, 16, 16],
                [32, 32, 32, 32],
                [64, 64, 64, 64],
                [128, 128, 128, 128],
                [256, 256, 256, 256],
                [512, 512, 512, 512],
                [0, 0, 0, 0],
            ],
            [
                [0, 3, 0, 0],
                [0, 7, 0, 0],
                [0, 15, 0, 0],
                [0, 30, 0, 0],
                [0, 60, 0, 0],
                [0, 120, 0, 0],
                [0, 240, 0, 0],
                [0, 480, 0, 0],
                [0, 960, 0, 0],
                [0, 896, 0, 0],
                [0, 768, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [2, 2, 2, 2],
                [4, 4, 4, 4],
                [8, 8, 8, 8],
                [16, 16, 16, 16],
                [32, 32, 32, 32],
                [64, 64, 64, 64],
                [128, 128, 128, 128],
                [256, 256, 256, 256],
                [512, 512, 512, 512],
            ],
        ],
        [
            [
                [0, 1, 0, 0],
                [0, 3, 1, 0],
                [0, 7, 2, 0],
                [0, 14, 4, 0],
                [0, 28, 8, 0],
                [0, 56, 16, 0],
                [0, 112, 32, 0],
                [0, 224, 64, 0],
                [0, 448, 128, 0],
                [0, 896, 256, 0],
                [0, 768, 512, 0],
            ],
            [
                [0, 1, 0, 0],
                [1, 3, 1, 0],
                [2, 6, 2, 0],
                [4, 12, 4, 0],
                [8, 24, 8, 0],
                [16, 48, 16, 0],
                [32, 96, 32, 0],
                [64, 192, 64, 0],
                [128, 384, 128, 0],
                [256, 768, 256, 0],
                [512, 512, 512, 0],
            ],
            [
                [0, 1, 0, 0],
                [1, 3, 0, 0],
                [2, 7, 0, 0],
                [4, 14, 0, 0],
                [8, 28, 0, 0],
                [16, 56, 0, 0],
                [32, 112, 0, 0],
                [64, 224, 0, 0],
                [128, 448, 0, 0],
                [256, 896, 0, 0],
                [512, 768, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [1, 1, 1, 0],
                [2, 3, 2, 0],
                [4, 6, 4, 0],
                [8, 12, 8, 0],
                [16, 24, 16, 0],
                [32, 48, 32, 0],
                [64, 96, 64, 0],
                [128, 192, 128, 0],
                [256, 384, 256, 0],
                [512, 768, 512, 0],
            ],
        ],
        [
            [
                [0, 1, 1, 0],
                [0, 3, 2, 0],
                [0, 7, 4, 0],
                [0, 14, 8, 0],
                [0, 28, 16, 0],
                [0, 56, 32, 0],
                [0, 112, 64, 0],
                [0, 224, 128, 0],
                [0, 448, 256, 0],
                [0, 896, 512, 0],
                [0, 768, 0, 0],
            ],
            [
                [1, 0, 0, 0],
                [3, 1, 1, 0],
                [6, 2, 2, 0],
                [12, 4, 4, 0],
                [24, 8, 8, 0],
                [48, 16, 16, 0],
                [96, 32, 32, 0],
                [192, 64, 64, 0],
                [384, 128, 128, 0],
                [768, 256, 256, 0],
                [512, 512, 512, 0],
            ],
            [
                [0, 1, 0, 0],
                [0, 3, 0, 0],
                [1, 7, 0, 0],
                [2, 14, 0, 0],
                [4, 28, 0, 0],
                [8, 56, 0, 0],
                [16, 112, 0, 0],
                [32, 224, 0, 0],
                [64, 448, 0, 0],
                [128, 896, 0, 0],
                [256, 768, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [1, 1, 1, 0],
                [2, 2, 3, 0],
                [4, 4, 6, 0],
                [8, 8, 12, 0],
                [16, 16, 24, 0],
                [32, 32, 48, 0],
                [64, 64, 96, 0],
                [128, 128, 192, 0],
                [256, 256, 384, 0],
                [512, 512, 768, 0],
            ],
        ],
        [
            [
                [0, 1, 0, 0],
                [0, 3, 0, 0],
                [0, 7, 1, 0],
                [0, 14, 2, 0],
                [0, 28, 4, 0],
                [0, 56, 8, 0],
                [0, 112, 16, 0],
                [0, 224, 32, 0],
                [0, 448, 64, 0],
                [0, 896, 128, 0],
                [0, 768, 256, 0],
            ],
            [
                [0, 0, 1, 0],
                [1, 1, 3, 0],
                [2, 2, 6, 0],
                [4, 4, 12, 0],
                [8, 8, 24, 0],
                [16, 16, 48, 0],
                [32, 32, 96, 0],
                [64, 64, 192, 0],
                [128, 128, 384, 0],
                [256, 256, 768, 0],
                [512, 512, 512, 0],
            ],
            [
                [1, 1, 0, 0],
                [2, 3, 0, 0],
                [4, 7, 0, 0],
                [8, 14, 0, 0],
                [16, 28, 0, 0],
                [32, 56, 0, 0],
                [64, 112, 0, 0],
                [128, 224, 0, 0],
                [256, 448, 0, 0],
                [512, 896, 0, 0],
                [0, 768, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [1, 1, 1, 0],
                [3, 2, 2, 0],
                [6, 4, 4, 0],
                [12, 8, 8, 0],
                [24, 16, 16, 0],
                [48, 32, 32, 0],
                [96, 64, 64, 0],
                [192, 128, 128, 0],
                [384, 256, 256, 0],
                [768, 512, 512, 0],
            ],
        ],
        [
            [
                [0, 0, 1, 0],
                [0, 1, 3, 0],
                [0, 3, 6, 0],
                [0, 6, 12, 0],
                [0, 12, 24, 0],
                [0, 24, 48, 0],
                [0, 48, 96, 0],
                [0, 96, 192, 0],
                [0, 192, 384, 0],
                [0, 384, 768, 0],
                [0, 768, 512, 0],
            ],
            [
                [1, 1, 0, 0],
                [2, 3, 1, 0],
                [4, 6, 2, 0],
                [8, 12, 4, 0],
                [16, 24, 8, 0],
                [32, 48, 16, 0],
                [64, 96, 32, 0],
                [128, 192, 64, 0],
                [256, 384, 128, 0],
                [512, 768, 256, 0],
                [0, 512, 512, 0],
            ],
            [
                [0, 1, 0, 0],
                [1, 3, 0, 0],
                [3, 6, 0, 0],
                [6, 12, 0, 0],
                [12, 24, 0, 0],
                [24, 48, 0, 0],
                [48, 96, 0, 0],
                [96, 192, 0, 0],
                [192, 384, 0, 0],
                [384, 768, 0, 0],
                [768, 512, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [1, 1, 0, 0],
                [2, 3, 1, 0],
                [4, 6, 2, 0],
                [8, 12, 4, 0],
                [16, 24, 8, 0],
                [32, 48, 16, 0],
                [64, 96, 32, 0],
                [128, 192, 64, 0],
                [256, 384, 128, 0],
                [512, 768, 256, 0],
            ],
        ],
        [
            [
                [0, 1, 0, 0],
                [0, 3, 1, 0],
                [0, 6, 3, 0],
                [0, 12, 6, 0],
                [0, 24, 12, 0],
                [0, 48, 24, 0],
                [0, 96, 48, 0],
                [0, 192, 96, 0],
                [0, 384, 192, 0],
                [0, 768, 384, 0],
                [0, 512, 768, 0],
            ],
            [
                [0, 1, 1, 0],
                [1, 3, 2, 0],
                [2, 6, 4, 0],
                [4, 12, 8, 0],
                [8, 24, 16, 0],
                [16, 48, 32, 0],
                [32, 96, 64, 0],
                [64, 192, 128, 0],
                [128, 384, 256, 0],
                [256, 768, 512, 0],
                [512, 512, 0, 0],
            ],
            [
                [1, 0, 0, 0],
                [3, 1, 0, 0],
                [6, 3, 0, 0],
                [12, 6, 0, 0],
                [24, 12, 0, 0],
                [48, 24, 0, 0],
                [96, 48, 0, 0],
                [192, 96, 0, 0],
                [384, 192, 0, 0],
                [768, 384, 0, 0],
                [512, 768, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [1, 3, 2, 0],
                [2, 6, 4, 0],
                [4, 12, 8, 0],
                [8, 24, 16, 0],
                [16, 48, 32, 0],
                [32, 96, 64, 0],
                [64, 192, 128, 0],
                [128, 384, 256, 0],
                [256, 768, 512, 0],
            ],
        ],
    ],
    height_maps: [
        [
            [(-1, -1), (1, 2), (1, 2), (-1, -1)],
            [(-1, -1), (1, 2), (1, 2), (-1, -1)],
            [(-1, -1), (1, 2), (1, 2), (-1, -1)],
            [(-1, -1), (1, 2), (1, 2), (-1, -1)],
        ],
        [
            [(2, 1), (2, 1), (2, 1), (2, 1)],
            [(-1, -1), (-1, -1), (0, 4), (-1, -1)],
            [(1, 1), (1, 1), (1, 1), (1, 1)],
            [(-1, -1), (0, 4), (-1, -1), (-1, -1)],
        ],
        [
            [(1, 1), (1, 2), (1, 1), (-1, -1)],
            [(-1, -1), (0, 3), (1, 1), (-1, -1)],
            [(1, 1), (0, 2), (1, 1), (-1, -1)],
            [(1, 1), (0, 3), (-1, -1), (-1, -1)],
        ],
        [
            [(1, 1), (1, 1), (1, 2), (-1, -1)],
            [(-1, -1), (0, 3), (0, 1), (-1, -1)],
            [(0, 2), (1, 1), (1, 1), (-1, -1)],
            [(2, 1), (0, 3), (-1, -1), (-1, -1)],
        ],
        [
            [(1, 2), (1, 1), (1, 1), (-1, -1)],
            [(-1, -1), (0, 3), (2, 1), (-1, -1)],
            [(1, 1), (1, 1), (0, 2), (-1, -1)],
            [(0, 1), (0, 3), (-1, -1), (-1, -1)],
        ],
        [
            [(1, 1), (1, 2), (2, 1), (-1, -1)],
            [(-1, -1), (1, 2), (0, 2), (-1, -1)],
            [(0, 1), (0, 2), (1, 1), (-1, -1)],
            [(1, 2), (0, 2), (-1, -1), (-1, -1)],
        ],
        [
            [(2, 1), (1, 2), (1, 1), (-1, -1)],
            [(-1, -1), (0, 2), (1, 2), (-1, -1)],
            [(1, 1), (0, 2), (0, 1), (-1, -1)],
            [(0, 2), (1, 2), (-1, -1), (-1, -1)],
        ],
    ],
    location_bounds: [
        [
            (-1, 7, -1, 21),
            (-1, 7, -1, 21),
            (-1, 7, -1, 21),
            (-1, 7, -1, 21),
        ],
        [
            (0, 6, -2, 21),
            (-2, 7, 0, 20),
            (0, 6, -1, 22),
            (-1, 8, 0, 20),
        ],
        [(0, 7, -1, 21), (-1, 7, 0, 21), (0, 7, 0, 22), (0, 8, 0, 21)],
        [(0, 7, -1, 21), (-1, 7, 0, 21), (0, 7, 0, 22), (0, 8, 0, 21)],
        [(0, 7, -1, 21), (-1, 7, 0, 21), (0, 7, 0, 22), (0, 8, 0, 21)],
        [(0, 7, -1, 21), (-1, 7, 0, 21), (0, 7, 0, 22), (0, 8, 0, 21)],
        [(0, 7, -1, 21), (-1, 7, 0, 21), (0, 7, 0, 22), (0, 8, 0, 21)],
    ],
    shift_bounds: [
        [(4, 4), (4, 4), (4, 4), (4, 4)],
        [(3, 3), (5, 4), (3, 3), (4, 5)],
        [(3, 4), (4, 4), (3, 4), (3, 5)],
        [(3, 4), (4, 4), (3, 4), (3, 5)],
        [(3, 4), (4, 4), (3, 4), (3, 5)],
        [(3, 4), (4, 4), (3, 4), (3, 5)],
        [(3, 4), (4, 4), (3, 4), (3, 5)],
    ],
    kick_table: [
        [
            [
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
        ],
        [
            [
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
        ],
        [
            [
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
        ],
        [
            [
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
        ],
        [
            [
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
        ],
        [
            [
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
        ],
        [
            [
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
            [
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                },
            ],
            [
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 1,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
                KickSeq {
                    len: 5,
                    shifts: [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                },
                KickSeq {
                    len: 0,
                    shifts: [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
                },
            ],
        ],
    ],
};

#[derive(Debug, Clone, Copy)]
pub struct KickSeq {
    pub len: i8,
    pub shifts: [(i8, i8); 5],
}
impl IntoIterator for KickSeq {
    type Item = (i8, i8);
    type IntoIter = std::iter::Take<std::array::IntoIter<(i8, i8), 5>>;

    fn into_iter(self) -> Self::IntoIter {
        self.shifts.into_iter().take(self.len as usize)
    }
}

/// Precomputed constants for a piece
#[derive(Debug, Clone, Copy)]
pub struct PieceInfo {
    /// The spawn location for each piece
    pub spawn_locations: [(i8, i8); PIECE_NUM_TYPES],
    /// The shape of each piece, as a 2d array of bools
    pub shapes:
        [[[[bool; PIECE_SHAPE_SIZE]; PIECE_SHAPE_SIZE]; PIECE_NUM_ROTATION]; PIECE_NUM_TYPES],
    /// u16 bit maps of each shape
    /// If shifting out of bounds, the shape will be cut-off
    pub bit_shapes: [[[[u16; PIECE_SHAPE_SIZE]; (PIECE_MAX_X_SHIFT * 2) + 1]; PIECE_NUM_ROTATION];
        PIECE_NUM_TYPES],
    /// Lows and Heights (Height from bottom to first block, then height of blocks)
    /// Both fields are -1 if that column is empty
    pub height_maps: [[[(i8, i8); PIECE_SHAPE_SIZE]; PIECE_NUM_ROTATION]; PIECE_NUM_TYPES],
    /// Min/Max x/y positions for a piece (min x, max x, min y, max y)
    /// min/max are both inclusive
    pub location_bounds: [[(i8, i8, i8, i8); PIECE_NUM_ROTATION]; PIECE_NUM_TYPES],
    /// How much a piece can shift from its spawn position (left and right)
    pub shift_bounds: [[(i8, i8); PIECE_NUM_ROTATION]; PIECE_NUM_TYPES],
    /// (x, y) shifts when doing kicks
    pub kick_table: [[[KickSeq; PIECE_NUM_ROTATION]; PIECE_NUM_ROTATION]; PIECE_NUM_TYPES],
}

impl PieceInfo {
    pub fn new() -> Self {
        fn rotate_shape(
            arr: [[bool; PIECE_SHAPE_SIZE]; PIECE_SHAPE_SIZE],
            size: usize,
        ) -> [[bool; PIECE_SHAPE_SIZE]; PIECE_SHAPE_SIZE] {
            let mut new_arr = [[false; PIECE_SHAPE_SIZE]; PIECE_SHAPE_SIZE];
            for i in 0..size {
                for j in 0..size {
                    new_arr[j][size - i - 1] = arr[i][j];
                }
            }
            new_arr
        }

        let sizes = [2, 4, 3, 3, 3, 3, 3];
        let spawn_locations = [
            (PIECE_SPAWN_COLUMN as i8, 20),
            (PIECE_SPAWN_COLUMN as i8, 19),
            (PIECE_SPAWN_COLUMN as i8, 20),
            (PIECE_SPAWN_COLUMN as i8, 20),
            (PIECE_SPAWN_COLUMN as i8, 20),
            (PIECE_SPAWN_COLUMN as i8, 20),
            (PIECE_SPAWN_COLUMN as i8, 20),
        ];
        let base_shapes = [
            // O
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            // I
            [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            // T
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            // L
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            // J
            [
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            // S
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            // Z
            [
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ];
        let mut shapes: [[[[bool; PIECE_SHAPE_SIZE]; PIECE_SHAPE_SIZE]; PIECE_NUM_ROTATION];
            PIECE_NUM_TYPES] = Default::default();
        for p in 0..PIECE_NUM_TYPES {
            let size = sizes[p];
            let mut shape = base_shapes[p];
            for r in 0..PIECE_NUM_ROTATION {
                shapes[p][r] = shape;
                // Don't rotate O shape
                if p != 0 {
                    shape = rotate_shape(shape, size);
                }
            }
        }

        // You have to be careful when doing bit shapes, as they're kinda backwards
        // Typically the LSB is written on the right side, but in this case
        // Bit 0 represents the left-most bit of the matrix
        let mut bit_shapes: [[[[u16; PIECE_SHAPE_SIZE]; (PIECE_MAX_X_SHIFT * 2) + 1];
            PIECE_NUM_ROTATION]; PIECE_NUM_TYPES] = Default::default();
        for p in 0..PIECE_NUM_TYPES {
            for r in 0..PIECE_NUM_ROTATION {
                let shape = shapes[p][r];
                for s in 0..(PIECE_MAX_X_SHIFT * 2) + 1 {
                    let mut bit_shape = [0u16; PIECE_SHAPE_SIZE];
                    for i in 0..PIECE_SHAPE_SIZE {
                        for j in 0..PIECE_SHAPE_SIZE {
                            if !shape[i][j] {
                                continue;
                            }
                            let x = (s + i + PIECE_SPAWN_COLUMN) as i8 - PIECE_MAX_X_SHIFT as i8;
                            if x < 0 || x >= BOARD_WIDTH as i8 {
                                continue;
                            }
                            bit_shape[j] |= 1 << x;
                        }
                    }
                    bit_shapes[p][r][s] = bit_shape;
                }
            }
        }

        // Calculate height maps, location bounds, and shift bounds
        let mut height_maps: [[[(i8, i8); PIECE_SHAPE_SIZE]; PIECE_NUM_ROTATION]; PIECE_NUM_TYPES] =
            Default::default();
        let mut location_bounds: [[(i8, i8, i8, i8); PIECE_NUM_ROTATION]; PIECE_NUM_TYPES] =
            Default::default();
        let mut shift_bounds: [[(i8, i8); PIECE_NUM_ROTATION]; PIECE_NUM_TYPES] =
            Default::default();
        for p in 0..PIECE_NUM_TYPES {
            for r in 0..PIECE_NUM_ROTATION {
                let shape = shapes[p][r];
                let bit_shape = bit_shapes[p][r][PIECE_MAX_X_SHIFT];
                // Calculate height map
                let mut height_map = [(-1, -1); PIECE_SHAPE_SIZE];
                for i in 0..PIECE_SHAPE_SIZE {
                    for j in 0..PIECE_SHAPE_SIZE {
                        if shape[i][j] {
                            if height_map[i].0 == -1 {
                                height_map[i] = (j as i8, 1);
                            } else {
                                height_map[i].1 += 1;
                            }
                        }
                    }
                }
                height_maps[p][r] = height_map;

                // Calculate location bounds
                let mut left = 0;
                for i in 0..PIECE_SHAPE_SIZE {
                    if height_map[i].0 == -1 {
                        left -= 1
                    } else {
                        break;
                    }
                }
                let mut right = (BOARD_WIDTH - PIECE_SHAPE_SIZE) as i8;
                for i in (0..PIECE_SHAPE_SIZE).rev() {
                    if height_map[i].0 == -1 {
                        right += 1;
                    } else {
                        break;
                    }
                }
                let mut bottom = 0;
                for j in 0..PIECE_SHAPE_SIZE {
                    if bit_shape[j] == 0 {
                        bottom -= 1;
                    } else {
                        break;
                    }
                }
                let mut top = (BOARD_HEIGHT - PIECE_SHAPE_SIZE) as i8;
                for j in (0..PIECE_SHAPE_SIZE).rev() {
                    if bit_shape[j] == 0 {
                        top += 1;
                    } else {
                        break;
                    }
                }
                location_bounds[p][r] = (left, right, bottom, top);
                shift_bounds[p][r] = (
                    PIECE_SPAWN_COLUMN as i8 - left,
                    right - PIECE_SPAWN_COLUMN as i8,
                );
            }
        }
        fn kick_table(table: [[Vec<(i8, i8)>; 4]; 4]) -> [[KickSeq; 4]; 4] {
            table.map(|x| {
                x.map(|vec| {
                    assert!(vec.len() <= 5);
                    let len = vec.len() as i8;
                    let mut shifts = [(0, 0); 5];
                    for (i, val) in vec.into_iter().take(5).enumerate() {
                        shifts[i] = val;
                    }
                    KickSeq { len, shifts }
                })
            })
        }
        // Pain
        let o_kick_table = kick_table([
            [vec![], vec![(0, 0)], vec![(0, 0)], vec![(0, 0)]],
            [vec![(0, 0)], vec![], vec![(0, 0)], vec![(0, 0)]],
            [vec![(0, 0)], vec![(0, 0)], vec![], vec![(0, 0)]],
            [vec![(0, 0)], vec![(0, 0)], vec![(0, 0)], vec![]],
        ]);
        let i_kick_table = kick_table([
            [
                // 0 >> 0
                vec![],
                // 0 >> 1
                vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                // 0 >> 2
                vec![(0, 0)],
                // 0 >> 3
                vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
            ],
            [
                // 1 >> 0
                vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                // 1 >> 1
                vec![],
                // 1 >> 2
                vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                // 1 >> 3
                vec![(0, 0)],
            ],
            [
                // 2 >> 0
                vec![(0, 0)],
                // 2 >> 1
                vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                // 2 >> 2
                vec![],
                // 2 >> 3
                vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
            ],
            [
                // 3 >> 0
                vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                // 3 >> 1
                vec![(0, 0)],
                // 3 >> 2
                vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                // 3 >> 3
                vec![],
            ],
        ]);
        let tljsz_kick_table = kick_table([
            [
                // 0 >> 0
                vec![],
                // 0 >> 1
                vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                // 0 >> 2
                vec![(0, 0)],
                // 0 >> 3
                vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            ],
            [
                // 1 >> 0
                vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                // 1 >> 1
                vec![],
                // 1 >> 2
                vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                // 1 >> 3
                vec![(0, 0)],
            ],
            [
                // 2 >> 0
                vec![(0, 0)],
                // 2 >> 1
                vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                // 2 >> 2
                vec![],
                // 2 >> 3
                vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            ],
            [
                // 3 >> 0
                vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                // 3 >> 1
                vec![(0, 0)],
                // 3 >> 2
                vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                // 3 >> 3
                vec![],
            ],
        ]);
        let kick_table = [
            o_kick_table,
            i_kick_table,
            tljsz_kick_table,
            tljsz_kick_table,
            tljsz_kick_table,
            tljsz_kick_table,
            tljsz_kick_table,
        ];
        PieceInfo {
            spawn_locations,
            shapes,
            bit_shapes,
            height_maps,
            location_bounds,
            shift_bounds,
            kick_table,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PIECE_INFO;
    use crate::model::consts::BOARD_WIDTH;
    use crate::model::consts::PIECE_MAX_X_SHIFT;
    use crate::model::consts::PIECE_NUM_ROTATION;
    use crate::model::consts::PIECE_NUM_TYPES;
    use crate::model::consts::PIECE_SHAPE_SIZE;
    use crate::model::consts::PIECE_SPAWN_COLUMN;

    #[test]
    fn bit_shapes_match_shapes() {
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let shape = PIECE_INFO.shapes[piece][rotation];
                // Get the center shape
                let bit_shape = PIECE_INFO.bit_shapes[piece][rotation][PIECE_MAX_X_SHIFT];
                // Check all 16 bits
                for j in 0..PIECE_SHAPE_SIZE {
                    for i in 0..16 {
                        let bit = (bit_shape[j] >> i) & 1;
                        let x = i as i8 - PIECE_SPAWN_COLUMN as i8;
                        if x < 0 || x >= PIECE_SHAPE_SIZE as i8 {
                            assert_eq!(bit, 0);
                        } else {
                            if shape[x as usize][j] {
                                assert_eq!(bit, 1);
                            } else {
                                assert_eq!(bit, 0);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn bit_shapes_match() {
        let bit_mask = (1 << BOARD_WIDTH) - 1;
        for piece in 0..PIECE_NUM_TYPES {
            for rotation in 0..PIECE_NUM_ROTATION {
                let bit_shape_arr = PIECE_INFO.bit_shapes[piece][rotation];
                let center_shape = bit_shape_arr[PIECE_MAX_X_SHIFT];
                for shift in 1..PIECE_MAX_X_SHIFT {
                    let left_shape = bit_shape_arr[(PIECE_MAX_X_SHIFT - shift)];
                    let right_shape = bit_shape_arr[(PIECE_MAX_X_SHIFT + shift)];
                    for j in 0..PIECE_SHAPE_SIZE {
                        let center = center_shape[j];
                        let left = left_shape[j];
                        let right = right_shape[j];
                        assert_eq!(left, center >> shift);
                        assert_eq!(right, (center << shift) & bit_mask);
                    }
                }
            }
        }
    }

    // TODO: Maybe add other tests
}
