use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io::stdin;
use rodio::cpal;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter, SampleFormat};

pub fn record_wav(output_path: &str, duration_secs: u64) -> Result<(), String> {
    let host = cpal::default_host();
    let devices: Vec<_> = host.input_devices().map_err(|e| e.to_string())?.collect();

    if devices.is_empty() {
        return Err("Aucun périphérique d'entrée audio trouvé.".into());
    }

    println!("Périphériques d'enregistrement disponibles :");
    for (i, device) in devices.iter().enumerate() {
        println!("{}: {}", i, device.name().unwrap_or("(inconnu)".into()));
    }

    println!("Entrez le numéro du périphérique à utiliser :");
    let mut input = String::new();
    stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    let index: usize = input.trim().parse().map_err(|_| "Entrée invalide.".to_string())?;

    let device = devices.get(index).ok_or("Indice de périphérique invalide.")?;
    let config = device.default_input_config().map_err(|e| e.to_string())?;

    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;
    let spec = WavSpec {
        channels: channels as u16,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let writer = WavWriter::create(output_path, spec).map_err(|e| e.to_string())?;
    let writer = std::sync::Arc::new(std::sync::Mutex::new(Some(writer)));

    let (sender, receiver) = mpsc::channel();
    let writer_clone = writer.clone();

    let stream = match config.sample_format() {
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                for frame in data.chunks(channels) {
                    let sample = frame[0]; // mono only
                    if let Ok(mut guard) = writer_clone.lock() {
                        if let Some(ref mut w) = *guard {
                            let _ = w.write_sample(sample);
                        }
                    }
                }
            },
            move |err| {
                eprintln!("Stream error: {}", err);
            }
        ),
        _ => return Err("Unsupported sample format".into()),
    }.map_err(|e| e.to_string())?;

    println!("Recording for {} seconds...", duration_secs);
    stream.play().map_err(|e| e.to_string())?;
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(duration_secs));
        sender.send(()).unwrap();
    });

    receiver.recv().unwrap();
    drop(stream);
    if let Ok(mut guard) = writer.lock() {
        if let Some(w) = guard.take() {
            w.finalize().map_err(|e| e.to_string())?;
        }
    }
    println!("Recording saved to {}", output_path);
    Ok(())
}