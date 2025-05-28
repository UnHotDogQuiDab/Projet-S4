use rustfft::{FftPlanner, num_complex::Complex, num_traits::Zero};

pub fn time_stretch(samples: &[f64], factor: f64) -> Vec<f64> {
    let window_size = 1024;
    let hop_in = (window_size as f64 / 4.0).round() as usize;
    let hop_out = (hop_in as f64 * factor).round() as usize;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);
    let ifft = planner.plan_fft_inverse(window_size);

    let mut output = vec![0.0; samples.len() * 2];
    let mut window = vec![0.0; window_size];
    for i in 0..window_size {
        window[i] = (std::f64::consts::PI * i as f64 / (window_size - 1) as f64).sin().powi(2);
    }

    let mut pos_in = 0;
    let mut pos_out = 0;

    while pos_in + window_size <= samples.len() {
        let frame: Vec<Complex<f64>> = samples[pos_in..pos_in + window_size]
            .iter()
            .zip(&window)
            .map(|(&s, &w)| Complex::new(s * w, 0.0))
            .collect();

        let mut spectrum = frame.clone();
        fft.process(&mut spectrum);

        ifft.process(&mut spectrum);

        for (j, c) in spectrum.iter().enumerate() {
            if pos_out + j < output.len() {
                output[pos_out + j] += (c.re / window_size as f64) * window[j];
            }
        }

        pos_in += hop_in;
        pos_out += hop_out;
    }

    output.truncate(pos_out + window_size);
    output
}
