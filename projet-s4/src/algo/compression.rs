use hound;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FftPlanner;
use std::fs::File;
use std::io::Write;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn load(filename: &str) -> (u32, Vec<f64>) 
{
    let mut reader = hound::WavReader::open(filename).expect("Impossible d'ouvrir le fichier WAV");
    let sample_rate = reader.spec().sample_rate;
    let samples = reader.samples::<i16>()
        .filter_map(Result::ok) 
        .map(|s| s as f64)
        .collect::<Vec<f64>>(); 
    (sample_rate, samples)
}

pub fn apply_fft(samples: &[f64]) -> Vec<Complex<f64>> 
{
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());
    let mut buffer: Vec<Complex<f64>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    fft.process(&mut buffer);
    buffer
}

fn filter(spectrum: &mut Vec<Complex<f64>>, keep_ratio: f64) 
{
    let len = spectrum.len();
    let keep_count = (keep_ratio * len as f64) as usize;
    let bass_preserve = (0.02 * len as f64) as usize;

    for i in keep_count..len 
    {
        if i > bass_preserve {  
            spectrum[i] = Complex::zero();
        }
    }

}

pub fn save_compressed(filename: &str, sample_rate: u32, spectrum: &[Complex<f64>]) 
{
    let mut byte_data = Vec::new();
    byte_data.extend_from_slice(&sample_rate.to_le_bytes());

    for coeff in spectrum 
    {
        byte_data.push((coeff.re as f32).to_le_bytes()[0]);
        byte_data.push((coeff.re as f32).to_le_bytes()[1]);
        byte_data.push((coeff.re as f32).to_le_bytes()[2]);
        byte_data.push((coeff.re as f32).to_le_bytes()[3]);

        byte_data.push((coeff.im as f32).to_le_bytes()[0]);
        byte_data.push((coeff.im as f32).to_le_bytes()[1]);
        byte_data.push((coeff.im as f32).to_le_bytes()[2]);
        byte_data.push((coeff.im as f32).to_le_bytes()[3]);
    }

    let file = File::create(filename).expect("Erreur lors de la création du fichier");
    let mut encoder = GzEncoder::new(file, Compression::best());
    encoder.write_all(&byte_data).unwrap();
    encoder.finish().unwrap();
}

pub fn compression(input_file: &str, output_file: &str) 
{
    let (sample_rate, samples) = load(input_file);
    let mut spectrum = apply_fft(&samples);
    filter(&mut spectrum, 0.01);
    save_compressed(output_file, sample_rate, &spectrum);
}
