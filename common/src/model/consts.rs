/// The total number of different piece types
pub const PIECE_NUM_TYPES: i32 = 7;
/// The total number of piece rotations
pub const PIECE_NUM_ROTATION: i32 = 4;
/// The shape bounds of a piece
pub const PIECE_SHAPE_SIZE: i32 = 4;
/// All pieces spawn in this column
pub const PIECE_SPAWN_COLUMN: i32 = 3;
/// Max shift (max a piece can shift left or shift right)
pub const PIECE_MAX_X_SHIFT: i32 = 5;

/// Width of the board
pub const BOARD_WIDTH: i32 = 10;
/// Height of the board
pub const BOARD_HEIGHT: i32 = 24;
/// Visible height of the board
/// Any pieces placed above this is considered a top-out
pub const BOARD_VISIBLE_HEIGHT: i32 = 20;

/// Size of the bag queue
pub const BAG_LEN: usize = PIECE_NUM_TYPES as usize;

/// Maximum number of pieces in the queue
pub const GAME_MAX_QUEUE_LEN: usize = 8;
