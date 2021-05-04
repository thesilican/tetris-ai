use c4w_ai::model::consts::BOARD_WIDTH;
use c4w_ai::model::consts::PIECE_MAX_X_SHIFT;
use c4w_ai::model::consts::PIECE_NUM_ROTATION;
use c4w_ai::model::consts::PIECE_NUM_TYPES;
use c4w_ai::model::consts::PIECE_SHAPE_SIZE;
use c4w_ai::model::consts::PIECE_STARTING_COLUMN;
use c4w_ai::model::piece::Piece;
use c4w_ai::model::piece::PieceType;
use std::fmt::Write;

// Visually verify that PIECE_INFO is correct
fn main() {
    for p in 0..PIECE_NUM_TYPES {
        let piece = Piece::new(PieceType::from_i32(p).unwrap());
        println!(
            "\n\n\n========== Piece {} ==========",
            piece.piece_type.to_char()
        );
        for r in 0..PIECE_NUM_ROTATION {
            println!("===== {} rotation {} =====", piece.piece_type.to_char(), r);
            // Print shape
            let shape = piece.get_shape(Some(r));
            let mut shape_text = String::new();
            for j in (0..PIECE_SHAPE_SIZE).rev() {
                for i in 0..PIECE_SHAPE_SIZE {
                    if shape[i as usize][j as usize] {
                        write!(shape_text, "██").unwrap();
                    } else {
                        write!(shape_text, "░░").unwrap();
                    }
                }
                write!(shape_text, "\n").unwrap();
            }
            print!("{}", shape_text);
            // Print height map
            let height_map = piece.get_height_map(Some(r));
            let mut height_map_text = String::new();
            for j in 0..PIECE_SHAPE_SIZE {
                write!(height_map_text, "{: >2}", height_map[j as usize].0).unwrap();
            }
            write!(height_map_text, "\n").unwrap();
            for j in 0..PIECE_SHAPE_SIZE {
                write!(height_map_text, "{: >2}", height_map[j as usize].1).unwrap();
            }
            println!("{}", height_map_text);
            // Print shift bounds
            let shift_bounds = piece.get_shift_bounds(Some(r));
            println!(
                "{} <-> {}  ↓{}, {}↑",
                shift_bounds.0, shift_bounds.1, shift_bounds.2, shift_bounds.3
            );
            // Print bit shapes
            let mut bit_shapes_text = String::new();
            for row_index in (0..PIECE_SHAPE_SIZE).rev() {
                for s in -PIECE_MAX_X_SHIFT..=PIECE_MAX_X_SHIFT {
                    let row = piece.get_bit_shape(Some(r), Some(s + PIECE_STARTING_COLUMN))
                        [row_index as usize];
                    let mut text = String::new();
                    write!(text, "{:016b}", row).unwrap();
                    let text = text
                        .chars()
                        .rev()
                        .take(BOARD_WIDTH as usize)
                        .collect::<String>();
                    write!(bit_shapes_text, "{} ", text).unwrap();
                }
                write!(bit_shapes_text, "\n").unwrap();
            }
            println!("{}", bit_shapes_text);
            println!();
        }
    }
}
