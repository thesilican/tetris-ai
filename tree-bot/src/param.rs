use libtetris::{Board, LockInfo};

#[derive(Debug, Clone, Copy)]
pub struct Params {
    // Node
    pub max_height: f32,
    pub bumpiness: f32,
    pub holes: f32,
    // Edge
    pub normal_clear: [f32; 5],
    pub tspin_clear: [f32; 4],
}

pub const PARAMS_DIM: usize = 12;

impl Params {
    pub fn eval_node(&self, board: &Board) -> f32 {
        // Board height
        let max_height = board.max_height().pow(2);

        // Board Bumpiness
        let mut bumpiness = 0;
        let height_map = board.height_map();
        for x in height_map.windows(2) {
            bumpiness += (x[0] as i32 - x[1] as i32).pow(2);
        }

        // Board holes
        let mut holes = 0;
        for x in board.holes() {
            holes += x as i32;
        }

        (self.max_height * max_height as f32)
            + (self.bumpiness * bumpiness as f32)
            + (self.holes * holes as f32)
    }

    pub fn eval_edge(&self, lock_info: &LockInfo) -> f32 {
        if lock_info.top_out {
            f32::NEG_INFINITY
        } else if lock_info.tspin {
            *self
                .tspin_clear
                .get(lock_info.lines_cleared as usize)
                .unwrap_or(&0.)
        } else {
            *self
                .normal_clear
                .get(lock_info.lines_cleared as usize)
                .unwrap_or(&0.)
        }
    }

    pub fn from_vec(vec: [f32; PARAMS_DIM]) -> Self {
        Params {
            max_height: vec[0],
            bumpiness: vec[1],
            holes: vec[2],
            normal_clear: [vec[3], vec[4], vec[5], vec[6], vec[7]],
            tspin_clear: [vec[8], vec[9], vec[10], vec[11]],
        }
    }

    pub fn to_vec(&self) -> [f32; PARAMS_DIM] {
        [
            self.max_height,
            self.bumpiness,
            self.holes,
            self.normal_clear[0],
            self.normal_clear[1],
            self.normal_clear[2],
            self.normal_clear[3],
            self.normal_clear[4],
            self.tspin_clear[0],
            self.tspin_clear[1],
            self.tspin_clear[2],
            self.tspin_clear[3],
        ]
    }
}

// Found using optimizer search
pub static DEFAULT_SCORE_PARAMS: Params = Params {
    max_height: -4.8412785e-5,
    bumpiness: -0.13946371,
    holes: -1.4413329,
    normal_clear: [-0.5015993, -1.2004814, 1.1999483, -0.120569, -0.7330629],
    tspin_clear: [-0.20565611, -0.44592947, 0.4023606, 0.8722384],
};
