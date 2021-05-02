use serde::{Deserialize, Serialize};

pub const BOARD_WIDTH: i32 = 10;
pub const BOARD_HEIGHT: i32 = 20;

#[derive(Deserialize)]
struct CLIInput {
    matrix: Vec<Vec<bool>>,
    queue: Vec<i32>,
    current: i32,
    hold: Option<i32>,
}

#[derive(Serialize)]
struct CLIOutput {
    moves: Vec<String>,
    score: Option<f64>,
}

#[derive(Debug)]
pub struct APIInput {
    pub current: i32,
    pub hold: Option<i32>,
    pub queue: Vec<i32>,
    pub matrix: [u16; BOARD_HEIGHT as usize],
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
pub struct APIOutput {
    pub score: Option<f64>,
    pub moves: Vec<APIMove>,
}

#[derive(Debug)]
pub enum APIError {
    IO(std::io::Error),
    Serde(serde_json::Error),
    Other(String),
}
impl From<serde_json::Error> for APIError {
    fn from(err: serde_json::Error) -> Self {
        APIError::Serde(err)
    }
}
impl From<std::io::Error> for APIError {
    fn from(err: std::io::Error) -> Self {
        APIError::IO(err)
    }
}

pub fn api_read() -> Result<APIInput, APIError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let input = serde_json::from_str::<CLIInput>(&input)?;

    let mut matrix = [0; BOARD_HEIGHT as usize];
    for (i, col) in input.matrix.iter().enumerate() {
        for (j, tile) in col.iter().enumerate() {
            if *tile {
                matrix[j as usize] |= 1 << i;
            }
        }
    }

    Ok(APIInput {
        current: input.current,
        hold: input.hold,
        queue: input.queue,
        matrix,
    })
}

pub fn api_write(options: APIOutput) -> Result<(), serde_json::Error> {
    let output = CLIOutput {
        moves: options
            .moves
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        score: options.score,
    };
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}
