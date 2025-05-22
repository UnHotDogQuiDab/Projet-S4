use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Convertit un fichier audio en .wav à l'aide de ffmpeg
pub fn convert_to_wav(input_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !input_file.exists() {
        return Err(format!("Fichier introuvable : {:?}", input_file).into());
    }

    // Génère le chemin de sortie avec l'extension .wav
    let output_file = input_file.with_extension("wav");

    // Exécute la commande ffmpeg
    let status = Command::new("ffmpeg")
        .args(["-y", "-i"])
        .arg(input_file)
        .arg(&output_file)
        .status()?;

    if !status.success() {
        return Err(format!("Échec de la conversion : {:?}", input_file).into());
    }

    println!("✅ Conversion réussie : {:?} → {:?}", input_file, output_file);
    Ok(())
}




