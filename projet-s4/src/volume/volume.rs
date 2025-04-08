use hound::{WavReader, WavSpec, WavWriter, SampleFormat};

pub fn generate_low_volume_wav(path: &str) {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec).unwrap();

    // 1 seconde de sinus faible
    for t in 0..44100 {
        let val = ((t as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin() * i16::MAX as f32 * 0.1) as i16;
        writer.write_sample(val).unwrap();
    }
}


pub fn adjust_volume(input_path: &str, output_path: &str, factor: f32) -> Result<(), String> {
    let mut reader = WavReader::open(input_path)
        .map_err(|_| "Failed to open WAV file".to_string())?;
    let spec = reader.spec();

    if spec.sample_format == SampleFormat::Int && spec.bits_per_sample == 16 {
        let samples: Vec<i16> = reader
            .into_samples::<i16>()
            .filter_map(Result::ok)
            .collect();

        let amplified: Vec<i16> = samples
            .iter()
            .map(|&s| {
                let val = s as f32 * factor;
                val.clamp(i16::MIN as f32, i16::MAX as f32).round() as i16
            })
            .collect();

        let mut writer = WavWriter::create(output_path, spec)
            .map_err(|_| "Failed to create WAV file".to_string())?;
        for s in amplified {
            writer.write_sample(s).map_err(|_| "Write error".to_string())?;
        }

        Ok(())
    } else {
        Err("Unsupported format (only 16-bit signed int supported)".to_string())
    }
}


