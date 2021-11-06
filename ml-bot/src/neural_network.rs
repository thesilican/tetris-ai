use common::{
    misc::GenericErr,
    model::{Board, BOARD_HEIGHT, BOARD_WIDTH},
};
use nalgebra::{DMatrix, DVector, RowDVector};
use rand::{distributions::Distribution, rngs::StdRng, SeedableRng};
use rand_distr::Normal;
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fs::File,
};

const LAYERS: [usize; 5] = [240, 100, 10, 20, 1];
const NUM_LAYERS: usize = LAYERS.len();

fn sigmoid(x: f32) -> f32 {
    1. / (1. + (-x).exp())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "NeuralNetworkSer")]
#[serde(into = "NeuralNetworkSer")]
pub struct NeuralNetwork {
    weights: [DMatrix<f32>; NUM_LAYERS - 1],
    biases: [DVector<f32>; NUM_LAYERS - 1],
    epoch: i32,
}
impl NeuralNetwork {
    /// Create a new neural network with random weights
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let distr = Normal::new(0., 1.).unwrap();
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        for i in 0..(NUM_LAYERS - 1) {
            // Generate weights matrix
            let mut weight: DMatrix<f32> = DMatrix::zeros(LAYERS[i], LAYERS[i + 1]);
            for x in 0..LAYERS[i] {
                for y in 0..LAYERS[i + 1] {
                    weight[(x, y)] = distr.sample(&mut rng);
                }
            }
            weights.push(weight);

            let mut bias: DVector<f32> = DVector::zeros(LAYERS[i + 1]);
            for x in 0..LAYERS[i + 1] {
                bias[(x, 0)] = distr.sample(&mut rng);
            }
            biases.push(bias);
        }
        // Convert from vec to array
        let weights = weights.try_into().unwrap();
        let biases = biases.try_into().unwrap();

        NeuralNetwork {
            weights,
            biases,
            epoch: 0,
        }
    }
    /// Load a serialized model from file
    pub fn load(path: &str) -> Result<Self, GenericErr> {
        let file = File::open(path)?;
        let res: NeuralNetwork = serde_json::from_reader(file)?;
        Ok(res)
    }
    /// Run the neural network on a given input
    fn run(&self, input: DVector<f32>) -> DVector<f32> {
        assert_eq!(input.shape(), (LAYERS[0], 1));
        let mut layer = input;
        for (weight, bias) in self.weights.iter().zip(self.biases.iter()) {
            layer = (weight * &layer) + bias;
            layer = layer.map(sigmoid);
        }
        layer
    }
    /// Run the neural network on a board
    pub fn run_board(&self, board: &Board) -> f32 {
        let mut vec = Vec::with_capacity(BOARD_WIDTH * BOARD_HEIGHT);
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if board.get(i, j) {
                    vec.push(1.);
                } else {
                    vec.push(0.)
                }
            }
        }
        let res = self.run(DVector::from_vec(vec));
        res[(0, 0)]
    }
}

// Serde stuff
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NeuralNetworkSer {
    weights: Vec<Vec<Vec<f32>>>,
    biases: Vec<Vec<[f32; 1]>>,
    epoch: i32,
}
impl TryFrom<NeuralNetworkSer> for NeuralNetwork {
    type Error = GenericErr;

    fn try_from(value: NeuralNetworkSer) -> Result<Self, Self::Error> {
        // Process weights
        let mut weights = Vec::new();
        for (i, weight) in value.weights.into_iter().enumerate() {
            let rows = weight
                .into_iter()
                .map(|row| RowDVector::from_vec(row))
                .collect::<Vec<_>>();
            let matrix = DMatrix::from_rows(&rows);
            if matrix.shape().0 != LAYERS[i + 1] || matrix.shape().1 != LAYERS[i] {
                let msg = format!(
                    "Weights layer {} has invalid shape: {:?}",
                    i,
                    matrix.shape()
                );
                return Err(msg.into());
            }
            weights.push(matrix);
        }
        // Try to coerce into array
        let weights = match weights.try_into() {
            Ok(arr) => arr,
            Err(_) => return Err("Invalid number of weights matrixes".into()),
        };

        // Process biases
        let mut biases = Vec::new();
        for (i, bias) in value.biases.into_iter().enumerate() {
            let col = bias.into_iter().map(|row| row[0]).collect::<Vec<_>>();
            if col.len() != LAYERS[i + 1] {
                let msg = format!("Biases layer {} has invalid length: {}", i, col.len());
                return Err(msg.into());
            }
            let vector = DVector::from_vec(col);
            biases.push(vector);
        }
        // Try to coerce in to array
        let biases = match biases.try_into() {
            Ok(arr) => arr,
            Err(_) => return Err("Invalid number of bias matrixes".into()),
        };

        Ok(NeuralNetwork {
            weights,
            biases,
            epoch: value.epoch,
        })
    }
}
impl From<NeuralNetwork> for NeuralNetworkSer {
    fn from(value: NeuralNetwork) -> Self {
        // Weights array
        let mut weights = Vec::new();
        for weight in value.weights.iter() {
            let mut matrix = Vec::new();
            for row in weight.row_iter() {
                let row_vec = row.iter().map(|x| *x).collect::<Vec<_>>();
                matrix.push(row_vec);
            }
            weights.push(matrix);
        }

        // Biases array
        let mut biases = Vec::new();
        for bias in value.biases.iter() {
            let vector = bias.iter().map(|x| [*x]).collect::<Vec<_>>();
            biases.push(vector);
        }

        NeuralNetworkSer {
            weights,
            biases,
            epoch: value.epoch,
        }
    }
}
