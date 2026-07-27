use rand::Rng;
use serde::{Deserialize, Serialize};

use super::example::TrainingExample;

pub const INPUT_SIZE: usize = 12;
pub const HIDDEN_SIZE: usize = 24;
pub const OUTPUT_SIZE: usize = 4;

const DEFAULT_LEARNING_RATE: f32 = 0.01;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheseusModel {
    input_hidden_weights: Vec<Vec<f32>>,
    hidden_biases: Vec<f32>,

    hidden_output_weights: Vec<Vec<f32>>,
    output_biases: Vec<f32>,

    learning_rate: f32,
    examples_trained: u128,
}

#[derive(Debug, Clone, Copy)]
pub struct TrainingResult {
    pub average_loss: f32,
    pub accuracy: f32,
    pub correct_predictions: usize,
    pub example_count: usize,
}

impl TheseusModel {
    /// Creates a new model with small random weights.
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let input_hidden_weights =
            create_weight_matrix(
                HIDDEN_SIZE,
                INPUT_SIZE,
                &mut rng,
            );

        let hidden_output_weights =
            create_weight_matrix(
                OUTPUT_SIZE,
                HIDDEN_SIZE,
                &mut rng,
            );

        Self {
            input_hidden_weights,
            hidden_biases: vec![0.0; HIDDEN_SIZE],

            hidden_output_weights,
            output_biases: vec![0.0; OUTPUT_SIZE],

            learning_rate: DEFAULT_LEARNING_RATE,
            examples_trained: 0,
        }
    }

    pub fn learning_rate(&self) -> f32 {
        self.learning_rate
    }

    pub fn examples_trained(&self) -> u128 {
        self.examples_trained
    }

    /// Trains the model once on every supplied example.
    ///
    /// One call represents one epoch over the examples generated
    /// from the current teacher path.
    pub fn train(
        &mut self,
        examples: &[TrainingExample],
    ) -> TrainingResult {
        if examples.is_empty() {
            return TrainingResult {
                average_loss: 0.0,
                accuracy: 0.0,
                correct_predictions: 0,
                example_count: 0,
            };
        }

        let mut total_loss = 0.0;
        let mut correct_predictions = 0;

        for example in examples {
            let input = example.input_values();
            let target = example.target_index();

            let result = self.train_one(&input, target);

            total_loss += result.loss;

            if result.prediction == target {
                correct_predictions += 1;
            }

            self.examples_trained += 1;
        }

        let example_count = examples.len();

        TrainingResult {
            average_loss: total_loss / example_count as f32,
            accuracy:
                correct_predictions as f32
                    / example_count as f32,
            correct_predictions,
            example_count,
        }
    }

    /// Returns the most likely direction class:
    ///
    /// 0 = North
    /// 1 = East
    /// 2 = South
    /// 3 = West
    pub fn predict(
        &self,
        input: &[f32; INPUT_SIZE],
    ) -> usize {
        let probabilities = self.predict_probabilities(input);

        index_of_largest(&probabilities)
    }

    /// Returns one probability for each direction.
    ///
    /// Order:
    /// [North, East, South, West]
    pub fn predict_probabilities(
        &self,
        input: &[f32; INPUT_SIZE],
    ) -> [f32; OUTPUT_SIZE] {
        let hidden_values = self.hidden_values(input);
        let output_scores = self.output_scores(&hidden_values);

        softmax(&output_scores)
    }

    fn train_one(
        &mut self,
        input: &[f32; INPUT_SIZE],
        target: usize,
    ) -> SingleTrainingResult {
        let hidden_pre_activations =
            self.hidden_pre_activations(input);

        let mut hidden_values = [0.0; HIDDEN_SIZE];

        for hidden_index in 0..HIDDEN_SIZE {
            hidden_values[hidden_index] =
                relu(
                    hidden_pre_activations[hidden_index],
                );
        }

        let output_scores =
            self.output_scores(&hidden_values);

        let probabilities = softmax(&output_scores);

        let prediction = index_of_largest(&probabilities);

        let target_probability =
            probabilities[target].max(f32::EPSILON);

        let loss = -target_probability.ln();

        /*
            For softmax combined with cross-entropy:

                output_gradient =
                    predicted_probability - target

            The target value is 1.0 for the teacher's direction
            and 0.0 for every other direction.
        */
        let mut output_gradients = probabilities;

        output_gradients[target] -= 1.0;

        /*
            Calculate hidden gradients before updating the
            hidden-to-output weights. Otherwise, the backpropagation
            calculation would partially use already-updated weights.
        */
        let mut hidden_gradients = [0.0; HIDDEN_SIZE];

        for hidden_index in 0..HIDDEN_SIZE {
            let mut gradient = 0.0;

            for output_index in 0..OUTPUT_SIZE {
                gradient +=
                    self.hidden_output_weights
                        [output_index][hidden_index]
                        * output_gradients[output_index];
            }

            hidden_gradients[hidden_index] =
                gradient
                    * relu_derivative(
                        hidden_pre_activations[hidden_index],
                    );
        }

        for output_index in 0..OUTPUT_SIZE {
            for hidden_index in 0..HIDDEN_SIZE {
                let gradient =
                    output_gradients[output_index]
                        * hidden_values[hidden_index];

                self.hidden_output_weights
                    [output_index][hidden_index] -=
                    self.learning_rate * gradient;
            }

            self.output_biases[output_index] -=
                self.learning_rate
                    * output_gradients[output_index];
        }

        for hidden_index in 0..HIDDEN_SIZE {
            for input_index in 0..INPUT_SIZE {
                let gradient =
                    hidden_gradients[hidden_index]
                        * input[input_index];

                self.input_hidden_weights
                    [hidden_index][input_index] -=
                    self.learning_rate * gradient;
            }

            self.hidden_biases[hidden_index] -=
                self.learning_rate
                    * hidden_gradients[hidden_index];
        }

        SingleTrainingResult {
            loss,
            prediction,
        }
    }

    fn hidden_values(
        &self,
        input: &[f32; INPUT_SIZE],
    ) -> [f32; HIDDEN_SIZE] {
        let pre_activations =
            self.hidden_pre_activations(input);

        let mut values = [0.0; HIDDEN_SIZE];

        for hidden_index in 0..HIDDEN_SIZE {
            values[hidden_index] =
                relu(pre_activations[hidden_index]);
        }

        values
    }

    fn hidden_pre_activations(
        &self,
        input: &[f32; INPUT_SIZE],
    ) -> [f32; HIDDEN_SIZE] {
        let mut values = [0.0; HIDDEN_SIZE];

        for hidden_index in 0..HIDDEN_SIZE {
            let mut value =
                self.hidden_biases[hidden_index];

            for input_index in 0..INPUT_SIZE {
                value +=
                    self.input_hidden_weights
                        [hidden_index][input_index]
                        * input[input_index];
            }

            values[hidden_index] = value;
        }

        values
    }

    fn output_scores(
        &self,
        hidden_values: &[f32; HIDDEN_SIZE],
    ) -> [f32; OUTPUT_SIZE] {
        let mut scores = [0.0; OUTPUT_SIZE];

        for output_index in 0..OUTPUT_SIZE {
            let mut score =
                self.output_biases[output_index];

            for hidden_index in 0..HIDDEN_SIZE {
                score +=
                    self.hidden_output_weights
                        [output_index][hidden_index]
                        * hidden_values[hidden_index];
            }

            scores[output_index] = score;
        }

        scores
    }
}

impl Default for TheseusModel {
    fn default() -> Self {
        Self::new()
    }
}

struct SingleTrainingResult {
    loss: f32,
    prediction: usize,
}

fn create_weight_matrix(
    rows: usize,
    columns: usize,
    rng: &mut impl Rng,
) -> Vec<Vec<f32>> {
    let scale = (2.0 / columns as f32).sqrt();

    let mut matrix = Vec::with_capacity(rows);

    for _ in 0..rows {
        let mut row = Vec::with_capacity(columns);

        for _ in 0..columns {
            row.push(
                rng.random_range(-scale..=scale),
            );
        }

        matrix.push(row);
    }

    matrix
}

fn relu(value: f32) -> f32 {
    value.max(0.0)
}

fn relu_derivative(value: f32) -> f32 {
    if value > 0.0 {
        1.0
    } else {
        0.0
    }
}

fn softmax(
    scores: &[f32; OUTPUT_SIZE],
) -> [f32; OUTPUT_SIZE] {
    /*
        Subtracting the largest score prevents exponentiation
        from overflowing without changing the probabilities.
    */
    let largest_score = scores
        .iter()
        .copied()
        .fold(f32::NEG_INFINITY, f32::max);

    let mut exponentials = [0.0; OUTPUT_SIZE];
    let mut exponential_sum = 0.0;

    for output_index in 0..OUTPUT_SIZE {
        let exponential =
            (scores[output_index] - largest_score).exp();

        exponentials[output_index] = exponential;
        exponential_sum += exponential;
    }

    if !exponential_sum.is_finite()
        || exponential_sum <= 0.0
    {
        return [0.25; OUTPUT_SIZE];
    }

    let mut probabilities = [0.0; OUTPUT_SIZE];

    for output_index in 0..OUTPUT_SIZE {
        probabilities[output_index] =
            exponentials[output_index]
                / exponential_sum;
    }

    probabilities
}

fn index_of_largest(
    values: &[f32; OUTPUT_SIZE],
) -> usize {
    let mut largest_index = 0;
    let mut largest_value = values[0];

    for (index, value) in
        values.iter().copied().enumerate().skip(1)
    {
        if value > largest_value {
            largest_value = value;
            largest_index = index;
        }
    }

    largest_index
}