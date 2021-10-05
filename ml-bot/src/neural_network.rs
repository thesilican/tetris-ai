use common::{
    misc::GenericErr,
    model::{Board, BOARD_HEIGHT, BOARD_WIDTH},
};
use nalgebra::{DMatrix, DVector, RowDVector};
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};

const LAYERS: [usize; 5] = [240, 100, 10, 20, 1];
const NUM_LAYERS: usize = LAYERS.len();

#[derive(Debug, Clone, Deserialize)]
struct NeuralNetworkSer {
    weights: Vec<Vec<Vec<f32>>>,
    biases: Vec<Vec<[f32; 1]>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "NeuralNetworkSer")]
pub struct NeuralNetwork {
    weights: [DMatrix<f32>; NUM_LAYERS - 1],
    biases: [DVector<f32>; NUM_LAYERS - 1],
}
impl NeuralNetwork {
    pub fn load() -> Self {
        let model_str = include_str!("../model/model.json");
        serde_json::from_str::<NeuralNetwork>(model_str).unwrap()
    }
    pub fn run_board(&self, board: &Board) -> f32 {
        let mut vec = Vec::new();
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
    fn run(&self, input: DVector<f32>) -> DVector<f32> {
        assert_eq!(input.shape(), (LAYERS[0], 1));
        let mut layer = input;
        for (weight, bias) in self.weights.iter().zip(self.biases.iter()) {
            layer = (weight * &layer) + bias;
            layer = layer.map(|x| 1. / (1. + (-x).exp()));
        }
        layer
    }
}
impl TryFrom<NeuralNetworkSer> for NeuralNetwork {
    type Error = GenericErr;

    fn try_from(value: NeuralNetworkSer) -> Result<Self, Self::Error> {
        assert_eq!(value.weights.len(), NUM_LAYERS - 1);
        let weights = value
            .weights
            .into_iter()
            .enumerate()
            .map(|(i, w)| {
                assert_eq!(w.len(), LAYERS[i + 1]);
                for row in w.iter() {
                    assert_eq!(row.len(), LAYERS[i]);
                }
                let rows = w
                    .into_iter()
                    .map(|row| RowDVector::from_vec(row))
                    .collect::<Vec<_>>();
                DMatrix::from_rows(&rows)
            })
            .collect::<Vec<_>>();
        assert_eq!(value.biases.len(), NUM_LAYERS - 1);
        let biases = value
            .biases
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                assert_eq!(b.len(), LAYERS[i + 1]);
                let col = b.into_iter().map(|row| row[0]).collect::<Vec<_>>();
                DVector::from_vec(col)
            })
            .collect::<Vec<_>>();
        Ok(NeuralNetwork {
            weights: weights.try_into().unwrap(),
            biases: biases.try_into().unwrap(),
        })
    }
}
