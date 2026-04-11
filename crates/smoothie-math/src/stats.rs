//! Statistical helpers for audio buffers.

/// RMS (root mean square) of a slice.
#[inline]
pub fn rms(buf: &[f32]) -> f32 {
    if buf.is_empty() { return 0.0; }
    let sum_sq: f32 = buf.iter().map(|&x| x * x).sum();
    (sum_sq / buf.len() as f32).sqrt()
}

/// Peak (max absolute value) of a slice.
#[inline]
pub fn peak(buf: &[f32]) -> f32 {
    buf.iter().fold(0.0_f32, |m, &x| m.max(x.abs()))
}

/// Crest factor = peak / rms.
#[inline]
pub fn crest_factor(buf: &[f32]) -> f32 {
    let r = rms(buf);
    if r == 0.0 { return 0.0; }
    peak(buf) / r
}

/// Mean value.
#[inline]
pub fn mean(buf: &[f32]) -> f32 {
    if buf.is_empty() { return 0.0; }
    buf.iter().sum::<f32>() / buf.len() as f32
}

/// Variance.
#[inline]
pub fn variance(buf: &[f32]) -> f32 {
    let m = mean(buf);
    buf.iter().map(|&x| { let d = x - m; d * d }).sum::<f32>() / buf.len() as f32
}

/// Standard deviation.
#[inline]
pub fn std_dev(buf: &[f32]) -> f32 { variance(buf).sqrt() }

/// Zero-crossing rate (per sample).
pub fn zcr(buf: &[f32]) -> f32 {
    if buf.len() < 2 { return 0.0; }
    let crossings = buf.windows(2).filter(|w| w[0].signum() != w[1].signum()).count();
    crossings as f32 / (buf.len() - 1) as f32
}
