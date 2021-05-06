use c4w_ai::ai::computed::StateTransitions;
use c4w_ai::ai::computed::C4W_TRANSITION_INFO;
use std::collections::HashMap;
use std::fmt::Write;

fn main() {
    let mut text = String::new();
    // Write center info
    {
        writeln!(text, "===== Center =====").unwrap();
        let mut counter = 1;
        let mut state_map = HashMap::new();
        for (state, _) in &C4W_TRANSITION_INFO.center {
            state_map.insert(state, counter);
            writeln!(text, "{}", counter).unwrap();
            counter += 1;
            for j in (0..4).rev() {
                write!(text, "  ").unwrap();
                for i in 0..4 {
                    if state >> (i + j * 4) & 1 == 1 {
                        write!(text, "██").unwrap();
                    } else {
                        write!(text, "░░").unwrap();
                    }
                }
                writeln!(text).unwrap();
            }
            writeln!(text).unwrap();
        }
        for (state, state_transitions) in &C4W_TRANSITION_INFO.center {
            let state_num = *state_map.get(state).unwrap();
            writeln!(text, "State {}", state_num).unwrap();
            for (piece_type, piece_transitions) in state_transitions.iter() {
                writeln!(text, "  Piece: {}", piece_type).unwrap();
                for (child_state, moves) in piece_transitions.iter() {
                    let child_state_num = *state_map.get(child_state).unwrap();
                    let mut moves_text = String::new();
                    for piece_move in moves {
                        write!(moves_text, "{} ", piece_move).unwrap();
                    }
                    writeln!(text, "    {} {}", child_state_num, moves_text.trim()).unwrap();
                }
            }
        }
        writeln!(text).unwrap();
    }
    // Write left and right info
    {
        fn write_transitions(text: &mut String, transitions: &StateTransitions<(i8, i8, i8)>) {
            for (state, state_transitions) in transitions {
                writeln!(text, "State ({},{},{})", state.0, state.1, state.2).unwrap();
                for (piece_type, piece_transitions) in state_transitions.iter() {
                    writeln!(text, "  Piece: {}", piece_type).unwrap();
                    for (child_state, moves) in piece_transitions.iter() {
                        let mut moves_text = String::new();
                        for piece_move in moves {
                            write!(moves_text, "{} ", piece_move).unwrap();
                        }
                        writeln!(
                            text,
                            "    ({},{},{}) {}",
                            child_state.0,
                            child_state.1,
                            child_state.2,
                            moves_text.trim()
                        )
                        .unwrap();
                    }
                }
            }
            writeln!(text).unwrap();
        }
        writeln!(text, "===== Left =====").unwrap();
        write_transitions(&mut text, &C4W_TRANSITION_INFO.left);
        writeln!(text, "===== Right =====").unwrap();
        write_transitions(&mut text, &C4W_TRANSITION_INFO.right);
    }
    println!("{}", text.trim());
}
