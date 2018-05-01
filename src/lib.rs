//! Simple crate for resampling an f64 signal

/// Calculates the amount of samples needed in the output buffer for desired
pub fn calc_out_samples(in_sample_count: u32, in_sample_rate: u32, out_sample_rate: u32) -> u32 {
    (in_sample_count * out_sample_rate) / in_sample_rate
}

/// Simple linear resampling, not great quality but get things going
pub fn linear_resample(in_pcm: &Vec<f64>, out_pcm: &mut Vec<f64>) {
    let in_sample_count: u32 = in_pcm.capacity() as u32;
    let out_sample_count: u32 = out_pcm.capacity() as u32;
    let sample_ratio: f64 = f64::from(out_sample_count - 1) / f64::from(in_sample_count - 1);
    let mut out_pos_rel_in: f64;
    let mut near_in_low: u32;
    let mut near_in_high: u32;
    let mut a_lin: f64;
    let mut b_lin: f64;
    for out_pos in 0..out_sample_count {
        out_pos_rel_in = f64::from(out_pos) / sample_ratio;
        near_in_low = out_pos_rel_in as u32;
        if f64::from(near_in_low) == out_pos_rel_in {
            // Value is the same
            out_pcm.push(in_pcm[near_in_low as usize]);
        } else {
            // Value needs to get calculated
            near_in_high = near_in_low + 1u32;
            // No need to divide for a, (x2 - x1) is always equal to 1
            a_lin = in_pcm[near_in_high as usize] - in_pcm[near_in_low as usize];
            b_lin = in_pcm[near_in_low as usize] - (f64::from(near_in_low) * a_lin);
            let out_sample = (a_lin * out_pos_rel_in) + b_lin;
            out_pcm.push(out_sample);
        }
    }
}
