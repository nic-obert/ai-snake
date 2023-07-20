use crate::consts::*;
use crate::map::{SubmapMatrix, Block};
use crate::snake::Direction;

use rand::Rng;
use serde_derive::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize)]
struct Layer {

    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,

}


impl Layer {

    pub fn new(input_size: usize, layer_size: usize) -> Self {

        let mut rand = rand::thread_rng();
        
        let mut biases: Vec<f64> = Vec::with_capacity(layer_size);
        let mut weights: Vec<Vec<f64>> = Vec::with_capacity(layer_size);

        for _ in 0..layer_size {
            biases.push(rand.gen_range(-1.0..1.0));

            let mut weights_row: Vec<f64> = Vec::with_capacity(input_size);
            for _ in 0..input_size {
                weights_row.push(rand.gen_range(-1.0..1.0));
            }

            weights.push(weights_row);
        }

        Self {
            weights,
            biases,
        }
    }


    pub fn forward(&self, input: &Vec<f64>) -> Vec<f64> {
        
        let mut output: Vec<f64> = Vec::with_capacity(self.weights.len());

        for i in 0..self.weights.len() {
            let mut sum = 0.0;
            for j in 0..self.weights[i].len() {
                sum += self.weights[i][j] * input[j];
            }
            sum += self.biases[i];
            output.push(sum);
        }

        output
    }


    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..self.weights.len() {

            if rng.gen_range(0.0..1.0) < MUTATION_CHANCE {
                self.biases[i] += rng.gen_range(-1.0..1.0) * MAX_MUTATION;
            }

            for j in 0..self.weights[i].len() {
                if rng.gen_range(0.0..1.0) < MUTATION_CHANCE {
                    self.weights[i][j] += rng.gen_range(-1.0..1.0) * MAX_MUTATION;
                }
            }
        }

    }

}


#[derive(Clone, Serialize, Deserialize)]
pub struct Brain {

    input_layer: Layer,
    output_layer: Layer,

}


impl Brain {

    pub fn mutate(&mut self) {
        
        self.input_layer.mutate();
        self.output_layer.mutate();
        
    }

    pub fn new() -> Self {
        Brain {
            input_layer: Layer::new(SIGHT_INPUT_SIZE, SIGHT_INPUT_SIZE),
            output_layer: Layer::new(SIGHT_INPUT_SIZE, 4),
        }
    }


    pub fn think(&mut self, input: &SubmapMatrix) -> Direction {

        let input: Vec<f64> = input.to_vec().iter().flatten().map(|block| 
            match block {
                Block::Void => 0.0,
                Block::Wall => -1.0,
                Block::SnakeTail => -1.0,
                Block::SnakeHead => -1.0,
                Block::Apple => 1.0,
            }).collect();

        let out = self.input_layer.forward(&input);
        
        let out = self.output_layer.forward(&out);

        let mut max = 0.0;
        let mut max_index = 0;
        for i in 0..out.len() {
            if out[i] > max {
                max = out[i];
                max_index = i;
            }
        }

        match max_index {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid direction index"),
        }

    }

}

