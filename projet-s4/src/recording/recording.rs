
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rodio::cpal;
use std::sync::Arc;
use std::sync::Mutex;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter, SampleFormat};



pub fn record_wav(output_path: &str, duration_secs: u64) -> Result<(), String> {
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or("No input device found")?;
    let config = device.default_input_config().map_err(|e| e.to_string())?;

    let spec = WavSpec {
        channels: config.channels(),
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let writer = WavWriter::create(output_path, spec).map_err(|e| e.to_string())?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    let (stop_tx, stop_rx) = mpsc::channel();

    let writer_clone = writer.clone();

    let stream = match config.sample_format() {
        cpal::SampleFormat::I16 => {
            device.build_input_stream(
                &config.into(),
                move |data: &[i16], _| {
                    if let Ok(mut writer_lock) = writer_clone.lock() {
                        if let Some(writer) = writer_lock.as_mut() {
                            for &sample in data {
                                let _ = writer.write_sample(sample);
                            }
                        }
                    }
                },
                move |err| {
                    eprintln!("Stream error: {}", err);
                },
            )
        }
        cpal::SampleFormat::F32 => {
            device.build_input_stream(
                &config.into(),
                move |data: &[f32], _| {
                    if let Ok(mut writer_lock) = writer_clone.lock() {
                        if let Some(writer) = writer_lock.as_mut() {
                            for &sample in data {
                                // Convertir f32 (-1.0..1.0) en i16
                                let s = (sample * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
                                let _ = writer.write_sample(s);
                            }
                        }
                    }
                },
                move |err| {
                    eprintln!("Stream error: {}", err);
                },
            )
        }
        _ => return Err("Unsupported sample format".into()),
    }
    .map_err(|e| e.to_string())?;

    println!("Recording for {} seconds...", duration_secs);
    stream.play().map_err(|e| e.to_string())?;

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(duration_secs));
        let _ = stop_tx.send(());
    });

    stop_rx.recv().unwrap();
    drop(stream);

    if let Ok(mut writer_lock) = writer.lock() {
        if let Some(writer) = writer_lock.take() {
            writer.finalize().map_err(|e| e.to_string())?;
        }
    }

    println!("Recording saved to {}", output_path);
    Ok(())
}