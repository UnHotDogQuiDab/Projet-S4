use hound;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::fs::File;
use std::io::{BufReader, Read};
use flate2::read::GzDecoder;

fn load_compressed(filename: &str) -> (u32, Vec<Complex<f64>>) 
{
    let file = File::open(filename).expect("Erreur lors de l'ouverture du fichier compressé");
    let decoder = GzDecoder::new(file);
    let mut reader = BufReader::new(decoder);
    
    let mut sample_rate_bytes = [0u8; 4];
    reader.read_exact(&mut sample_rate_bytes).unwrap();
    let sample_rate = u32::from_le_bytes(sample_rate_bytes);
    
    let mut spectrum = Vec::new();
    let mut buf = [0u8; 4];
    while reader.read_exact(&mut buf).is_ok() 
    {
        let re = f32::from_le_bytes(buf) as f64;
        reader.read_exact(&mut buf).unwrap();
        let im = f32::from_le_bytes(buf) as f64;
        spectrum.push(Complex::new(re, im));
    }
    (sample_rate, spectrum)
}

fn apply_ifft(spectrum: &[Complex<f64>]) -> Vec<f64> 
{
    let mut planner = FftPlanner::new();
    let ifft = planner.plan_fft_inverse(spectrum.len());
    let mut buffer = spectrum.to_vec();
    ifft.process(&mut buffer);
    
    let scale = (buffer.len() as f64).sqrt();
    buffer.iter().map(|c| c.re / scale).collect()
}

fn save(filename: &str, sample_rate: u32, samples: &[f64]) 
{
    let max_val = samples.iter().copied().fold(f64::MIN, f64::max).max(1.0);
    let factor = 1.0 / max_val;

    let spec = hound::WavSpec 
    {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    
    for &sample in samples 
    {
        let sample_i16 = (sample * factor * i16::MAX as f64) as i16;
        writer.write_sample(sample_i16).unwrap();
    }
}

fn resample(samples: &[f64], speed: f64) -> Vec<f64> {
    let new_len = (samples.len() as f64 / speed).round() as usize;
    (0..new_len)
        .map(|i| {
            let index = i as f64 * speed;
            let i0 = index.floor() as usize;
            let i1 = (i0 + 1).min(samples.len() - 1);
            let t = index - i0 as f64;
            (1.0 - t) * samples[i0] + t * samples[i1]
        })
        .collect()
}


pub fn decompression(input_file: &str, output_file: &str, speed: f64)
{
    let samples = apply_ifft(&spectrum);
    let resampled = resample(&samples, speed);
    save(output_file, sample_rate, &resampled);

    save(output_file, sample_rate, &samples);
}
