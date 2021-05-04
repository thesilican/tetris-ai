use serde::{Deserialize, Serialize};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

#[derive(Debug)]
pub struct APIRequest {
    pub current: i32,
    pub hold: Option<i32>,
    pub queue: Vec<i32>,
    pub matrix: [u16; BOARD_HEIGHT as usize],
}

#[derive(Debug)]
pub struct APIResponse {
    pub score: Option<f64>,
    pub moves: Vec<APIMove>,
}

#[derive(Debug)]
pub struct APIError(pub String);
impl From<()> for APIError {
    fn from(_: ()) -> Self{
        APIError(String::from("Unknown error"))
    }
}
impl From<&str> for APIError {
    fn from(err: &str) -> Self{
        APIError(String::from(err))
    }
}

#[derive(Debug)]
pub enum APIMove {
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    Rotate180,
    Hold,
    SoftDrop,
    HardDrop,
}
impl APIMove {
    fn to_string(&self) -> String {
        String::from(match self {
            APIMove::ShiftLeft => "shiftLeft",
            APIMove::ShiftRight => "shiftRight",
            APIMove::RotateLeft => "rotateLeft",
            APIMove::RotateRight => "rotateRight",
            APIMove::Rotate180 => "rotate180",
            APIMove::Hold => "hold",
            APIMove::SoftDrop => "softDrop",
            APIMove::HardDrop => "hardDrop",
        })
    }
}
impl std::fmt::Display for APIMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.to_string();
        write!(f, "{}", text)
    }
}

#[derive(Debug)]
pub enum JSONError {
    Serde(serde_json::Error),
    Other(String),
}
impl From<serde_json::Error> for JSONError {
    fn from(err: serde_json::Error) -> Self {
        JSONError::Serde(err)
    }
}
#[derive(Deserialize)]
struct JSONInput {
    matrix: Vec<Vec<bool>>,
    queue: Vec<i32>,
    current: i32,
    hold: Option<i32>,
}

#[derive(Serialize)]
struct JSONOutput {
    moves: Vec<String>,
    score: Option<f64>,
}

pub trait TetrisAI {
    fn evaluate(&mut self, req: APIRequest) -> Result<APIResponse, APIError>;
}

pub fn parse(req: String) -> Result<APIRequest, JSONError> {
    let input = serde_json::from_str::<JSONInput>(&req)?;

    if input.matrix.len() != BOARD_WIDTH || input.matrix[0].len() != BOARD_HEIGHT {
        return Err(JSONError::Other(String::from("Invalid matrix size")));
    }

    let mut matrix = [0; BOARD_HEIGHT];
    for (i, col) in input.matrix.iter().enumerate() {
        for (j, tile) in col.iter().enumerate() {
            if *tile {
                matrix[j as usize] |= 1 << i;
            }
        }
    }

    Ok(APIRequest {
        current: input.current,
        hold: input.hold,
        queue: input.queue,
        matrix,
    })
}

pub fn stringify(res: APIResponse) -> Result<String, JSONError> {
    let output = JSONOutput {
        moves: res
            .moves
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        score: res.score,
    };
    Ok(serde_json::to_string(&output)?)
}
