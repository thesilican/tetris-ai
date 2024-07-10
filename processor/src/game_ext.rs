use libtetris::model::Game;

pub trait GameExt {
    fn eq_ignore_queue(&self, other: Self) -> bool;
}

impl GameExt for Game {
    // Utility function to check if two games are equal
    // Comparing the shorter of the two queues
    fn eq_ignore_queue(&self, other: Self) -> bool {
        self.board == other.board
            && self.current_piece == other.current_piece
            && self.hold_piece == other.hold_piece
            && self.can_hold == other.can_hold
            && self
                .queue_pieces
                .iter()
                .zip(other.queue_pieces.iter())
                .all(|(a, b)| a == b)
    }
}
