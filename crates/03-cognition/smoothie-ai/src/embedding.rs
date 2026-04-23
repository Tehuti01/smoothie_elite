/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x45d11d63 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/embedding.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::boxed::Box;

///
/// velocity values, timing/position, and other musical attributes.
use alloc::vec;
use alloc::vec::Vec;
// use smoothie_core::math::FloatExt;

/// Technical implementation of the Embedding structure.
pub struct Embedding {
    pub weights: Vec<f32>,
    pub num_embeddings: usize,
    pub embedding_dim: usize,
}

impl Embedding {
    /// Initializes a new instance of the associated type.
    pub fn new(num_embeddings: usize, embedding_dim: usize) -> Self {
        let scale = 0.1 / (embedding_dim as f32).sqrt();

        let weights = (0..num_embeddings * embedding_dim)
            .map(|i| if i % 2 == 0 { scale } else { -scale })
            .collect();

        Self {
            weights,
            num_embeddings,
            embedding_dim,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, indices: &[usize], output: &mut [f32]) {
        let _output_len = indices.len() * self.embedding_dim;

        for (i, &idx) in indices.iter().enumerate() {
            if idx < self.num_embeddings {
                for d in 0..self.embedding_dim {
                    let out_idx = i * self.embedding_dim + d;
                    if out_idx < output.len() {
                        output[out_idx] = self.weights[idx * self.embedding_dim + d];
                    }
                }
            }
        }
    }

    /// Technical implementation of the get_embedding logic.
    pub fn get_embedding(&self, idx: usize) -> Option<&[f32]> {
        if idx < self.num_embeddings {
            let start = idx * self.embedding_dim;
            let end = start + self.embedding_dim;
            Some(&self.weights[start..end])
        } else {
            None
        }
    }

    /// Technical implementation of the get_embedding_mut logic.
    pub fn get_embedding_mut(&mut self, idx: usize) -> Option<&mut [f32]> {
        if idx < self.num_embeddings {
            let start = idx * self.embedding_dim;
            let end = start + self.embedding_dim;
            Some(&mut self.weights[start..end])
        } else {
            None
        }
    }

    /// Technical implementation of the load_pretrained logic.
    pub fn load_pretrained(&mut self, weights: &[f32]) -> Result<(), &'static str> {
        if weights.len() != self.weights.len() {
            return Err("Embedding weights size mismatch");
        }
        self.weights.copy_from_slice(weights);
        Ok(())
    }
}

/// Technical implementation of the PositionalEmbedding structure.
pub struct PositionalEmbedding {
    pub weights: Vec<f32>,
    pub max_len: usize,
    pub embedding_dim: usize,
}

impl PositionalEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(max_len: usize, embedding_dim: usize) -> Self {
        let mut weights = vec![0.0; max_len * embedding_dim];

        for pos in 0..max_len {
            for d in 0..embedding_dim {
                let angle = pos as f32 / 10000.0_f32.powf(2.0 * d as f32 / embedding_dim as f32);
                weights[pos * embedding_dim + d] =
                    if d % 2 == 0 { angle.sin() } else { angle.cos() };
            }
        }

        Self {
            weights,
            max_len,
            embedding_dim,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, positions: &[usize], output: &mut [f32]) {
        for (i, &pos) in positions.iter().enumerate() {
            let pos = pos.min(self.max_len - 1);
            for d in 0..self.embedding_dim {
                let out_idx = i * self.embedding_dim + d;
                if out_idx < output.len() {
                    output[out_idx] = self.weights[pos * self.embedding_dim + d];
                }
            }
        }
    }
}

/// Technical implementation of the NoteEmbedding structure.
pub struct NoteEmbedding {
    pub pitch: Embedding,
    pub velocity: Embedding,
    pub duration: Embedding,
    pub channel: Embedding,
}

impl NoteEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(embedding_dim: usize) -> Self {
        Self {
            pitch: Embedding::new(128, embedding_dim),
            velocity: Embedding::new(128, embedding_dim),
            duration: Embedding::new(256, embedding_dim),
            channel: Embedding::new(16, embedding_dim),
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(
        &self,
        pitch: &[usize],
        velocity: &[usize],
        duration: &[usize],
        channel: &[usize],
        output: &mut [f32],
    ) {
        let seq_len = pitch.len();
        let embedding_dim = self.pitch.embedding_dim;

        let mut pitch_out = vec![0.0; seq_len * embedding_dim];
        self.pitch.forward(pitch, &mut pitch_out);

        let mut vel_out = vec![0.0; seq_len * embedding_dim];
        self.velocity.forward(velocity, &mut vel_out);

        let mut dur_out = vec![0.0; seq_len * embedding_dim];
        self.duration.forward(duration, &mut dur_out);

        let mut chan_out = vec![0.0; seq_len * embedding_dim];
        self.channel.forward(channel, &mut chan_out);

        for i in 0..seq_len * embedding_dim {
            output[i] = pitch_out[i] + vel_out[i] + dur_out[i] + chan_out[i];
        }
    }
}

/// Technical implementation of the TimingEmbedding structure.
pub struct TimingEmbedding {
    pub beat: Embedding,
    pub measure: Embedding,
    pub tempo: Embedding,
    pub time_signature: Embedding,
}

impl TimingEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(embedding_dim: usize) -> Self {
        Self {
            beat: Embedding::new(32, embedding_dim),
            measure: Embedding::new(128, embedding_dim),
            tempo: Embedding::new(256, embedding_dim),
            time_signature: Embedding::new(16, embedding_dim),
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(
        &self,
        beat: &[usize],
        measure: &[usize],
        tempo: &[usize],
        time_sig: &[usize],
        output: &mut [f32],
    ) {
        let seq_len = beat.len();
        let embedding_dim = self.beat.embedding_dim;

        let mut beat_out = vec![0.0; seq_len * embedding_dim];
        self.beat.forward(beat, &mut beat_out);

        let mut measure_out = vec![0.0; seq_len * embedding_dim];
        self.measure.forward(measure, &mut measure_out);

        let mut tempo_out = vec![0.0; seq_len * embedding_dim];
        self.tempo.forward(tempo, &mut tempo_out);

        let mut ts_out = vec![0.0; seq_len * embedding_dim];
        self.time_signature.forward(time_sig, &mut ts_out);

        for i in 0..seq_len * embedding_dim {
            output[i] = beat_out[i] + measure_out[i] + tempo_out[i] + ts_out[i];
        }
    }
}

/// Technical implementation of the MultiEmbedding structure.
pub struct MultiEmbedding {
    pub embeddings: Vec<Embedding>,
    pub embedding_dim: usize,
}

impl MultiEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(vocab_sizes: &[usize], embedding_dim: usize) -> Self {
        let embeddings = vocab_sizes
            .iter()
            .map(|&size| Embedding::new(size, embedding_dim))
            .collect();

        Self {
            embeddings,
            embedding_dim,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, indices: &[Vec<usize>], output: &mut [f32]) {
        if indices.is_empty() {
            return;
        }

        let seq_len = indices.len();

        for (emb_idx, emb) in self.embeddings.iter().enumerate() {
            let mut emb_out = vec![0.0; seq_len * self.embedding_dim];

            let field_indices: Vec<usize> = indices
                .iter()
                .map(|v| v.get(emb_idx).copied().unwrap_or(0))
                .collect();
            emb.forward(&field_indices, &mut emb_out);

            for (i, v) in emb_out.iter().enumerate().take(output.len()) {
                output[i] += v;
            }
        }
    }
}

/// Technical implementation of the FeatureEmbedding structure.
pub struct FeatureEmbedding {
    pub continuous_weights: Vec<f32>,
    pub discrete: Embedding,
    pub embedding_dim: usize,
    num_continuous: usize,
}

impl FeatureEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(num_continuous: usize, num_discrete: usize, embedding_dim: usize) -> Self {
        Self {
            continuous_weights: vec![1.0; num_continuous * embedding_dim],
            discrete: Embedding::new(num_discrete, embedding_dim),
            embedding_dim,
            num_continuous,
        }
    }

    /// Technical implementation of the forward_continuous logic.
    pub fn forward_continuous(&self, values: &[f32], output: &mut [f32]) {
        let seq_len = values.len() / self.num_continuous;

        for b in 0..seq_len {
            for d in 0..self.embedding_dim {
                let out_idx = b * self.embedding_dim + d;
                if out_idx < output.len() {
                    let cont_idx = b * self.num_continuous;
                    let scale = self.continuous_weights[d % self.continuous_weights.len()];
                    output[out_idx] = values[cont_idx] * scale;
                }
            }
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, continuous: &[f32], discrete: &[usize], output: &mut [f32]) {
        self.forward_continuous(continuous, output);

        let seq_len = discrete.len();
        let mut disc_out = vec![0.0; seq_len * self.embedding_dim];
        self.discrete.forward(discrete, &mut disc_out);

        for i in 0..output.len() {
            output[i] += disc_out.get(i).copied().unwrap_or(0.0);
        }
    }
}

/// Technical implementation of the SinusoidalEmbedding structure.
pub struct SinusoidalEmbedding {
    pub embedding_dim: usize,
}

impl SinusoidalEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(embedding_dim: usize) -> Self {
        Self { embedding_dim }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, x: &[f32], output: &mut [f32]) {
        let freq = 10000.0_f32;

        for (i, &val) in x.iter().enumerate() {
            for d in 0..self.embedding_dim {
                let idx = i * self.embedding_dim + d;
                if idx < output.len() {
                    let angle = val / freq.powf(2.0 * d as f32 / self.embedding_dim as f32);
                    output[idx] = if d % 2 == 0 { angle.sin() } else { angle.cos() };
                }
            }
        }
    }
}

/// Technical implementation of the LearnedPositionalEmbedding structure.
pub struct LearnedPositionalEmbedding {
    pub weights: Vec<f32>,
    pub max_len: usize,
    pub embedding_dim: usize,
}

impl LearnedPositionalEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(max_len: usize, embedding_dim: usize) -> Self {
        Self {
            weights: vec![0.0; max_len * embedding_dim],
            max_len,
            embedding_dim,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, positions: &[usize], output: &mut [f32]) {
        for (i, &pos) in positions.iter().enumerate() {
            let pos = pos.min(self.max_len - 1);
            for d in 0..self.embedding_dim {
                let out_idx = i * self.embedding_dim + d;
                if out_idx < output.len() {
                    output[out_idx] = self.weights[pos * self.embedding_dim + d];
                }
            }
        }
    }

    /// Technical implementation of the update logic.
    pub fn update(&mut self, positions: &[usize], grads: &[f32]) {
        for (i, &pos) in positions.iter().enumerate() {
            let pos = pos.min(self.max_len - 1);
            for d in 0..self.embedding_dim {
                let idx = pos * self.embedding_dim + d;
                let grad_idx = i * self.embedding_dim + d;
                if idx < self.weights.len() && grad_idx < grads.len() {
                    self.weights[idx] += grads[grad_idx] * 0.01;
                }
            }
        }
    }
}

/// Technical implementation of the ConcatEmbedding structure.
pub struct ConcatEmbedding {
    pub embeddings: Vec<Box<dyn EmbeddingTrait>>,
    pub output_dim: usize,
}

pub trait EmbeddingTrait: Send + Sync {
    /// Technical implementation of the forward logic.
    fn forward(&self, indices: &[usize], output: &mut [f32]);
    /// Technical implementation of the embedding_dim logic.
    fn embedding_dim(&self) -> usize;
}

impl EmbeddingTrait for Embedding {
    /// Technical implementation of the forward logic.
    fn forward(&self, indices: &[usize], output: &mut [f32]) {
        Embedding::forward(self, indices, output);
    }

    /// Technical implementation of the embedding_dim logic.
    fn embedding_dim(&self) -> usize {
        self.embedding_dim
    }
}

impl EmbeddingTrait for PositionalEmbedding {
    /// Technical implementation of the forward logic.
    fn forward(&self, indices: &[usize], output: &mut [f32]) {
        PositionalEmbedding::forward(self, indices, output);
    }

    /// Technical implementation of the embedding_dim logic.
    fn embedding_dim(&self) -> usize {
        self.embedding_dim
    }
}

impl ConcatEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(embeddings: Vec<Box<dyn EmbeddingTrait>>) -> Self {
        let output_dim = embeddings.iter().map(|e| e.embedding_dim()).sum();
        Self {
            embeddings,
            output_dim,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, indices: &[Vec<usize>], output: &mut [f32]) {
        if indices.is_empty() || self.embeddings.is_empty() {
            return;
        }

        let seq_len = indices.len();

        for (emb_idx, emb) in self.embeddings.iter().enumerate() {
            let field_indices: Vec<usize> = indices
                .iter()
                .map(|v| v.get(emb_idx).copied().unwrap_or(0))
                .collect();

            let emb_dim = emb.embedding_dim();
            let mut emb_out = vec![0.0; seq_len * emb_dim];
            emb.forward(&field_indices, &mut emb_out);

            let offset = self.embeddings[..emb_idx]
                .iter()
                .map(|e| e.embedding_dim())
                .sum::<usize>();

            for (i, v) in emb_out
                .iter()
                .enumerate()
                .take(output.len().saturating_sub(offset))
            {
                output[offset + i] += v;
            }
        }
    }
}

/// Technical implementation of the QuantizedEmbedding structure.
pub struct QuantizedEmbedding {
    pub codebook: Vec<f32>,
    pub num_embeddings: usize,
    pub embedding_dim: usize,
}

impl QuantizedEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(num_embeddings: usize, embedding_dim: usize) -> Self {
        let codebook = (0..num_embeddings * embedding_dim)
            .map(|i| (i as f32 % 256.0 / 256.0 - 0.5) * 0.1)
            .collect();

        Self {
            codebook,
            num_embeddings,
            embedding_dim,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, indices: &[usize], output: &mut [f32]) {
        for (i, &idx) in indices.iter().enumerate() {
            if idx < self.num_embeddings {
                for d in 0..self.embedding_dim {
                    let out_idx = i * self.embedding_dim + d;
                    if out_idx < output.len() {
                        output[out_idx] = self.codebook[idx * self.embedding_dim + d];
                    }
                }
            }
        }
    }

    /// Technical implementation of the quantize logic.
    pub fn quantize(&self, continuous: &[f32]) -> Vec<usize> {
        let num_vectors = continuous.len() / self.embedding_dim;
        let mut indices = Vec::with_capacity(num_vectors);

        for i in 0..num_vectors {
            let z = &continuous[i * self.embedding_dim..(i + 1) * self.embedding_dim];

            let mut min_dist = f32::INFINITY;
            let mut best_idx = 0;

            for j in 0..self.num_embeddings {
                let mut dist = 0.0f32;
                for d in 0..self.embedding_dim {
                    let diff = z[d] - self.codebook[j * self.embedding_dim + d];
                    dist += diff * diff;
                }
                if dist < min_dist {
                    min_dist = dist;
                    best_idx = j;
                }
            }

            indices.push(best_idx);
        }

        indices
    }

    /// Technical implementation of the decode logic.
    pub fn decode(&self, indices: &[usize], output: &mut [f32]) {
        self.forward(indices, output);
    }
}

/// Technical implementation of the CategoricalEmbedding structure.
pub struct CategoricalEmbedding {
    pub embeddings: Vec<Embedding>,
    pub num_categories: usize,
}

impl CategoricalEmbedding {
    /// Initializes a new instance of the associated type.
    pub fn new(num_categories: usize, embedding_dim: usize) -> Self {
        let embeddings = (0..num_categories)
            .map(|_| Embedding::new(256, embedding_dim))
            .collect();

        Self {
            embeddings,
            num_categories,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, categories: &[usize], output: &mut [f32]) {
        for (cat_idx, &cat) in categories.iter().enumerate() {
            if cat < self.num_categories {
                let emb_dim = self.embeddings[0].embedding_dim;
                for d in 0..emb_dim {
                    let out_idx = cat_idx * emb_dim + d;
                    if out_idx < output.len() {
                        let idx = cat.min(self.embeddings[cat].num_embeddings - 1);
                        output[out_idx] = self.embeddings[cat].weights[idx * emb_dim + d];
                    }
                }
            }
        }
    }
}
