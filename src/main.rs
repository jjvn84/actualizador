// Todo: - Agregar soporte para crates de Rust instaladas directamente con cargo install.

use clap::Parser;
use colored::Colorize;
use std::io;
use std::process::{Command, ExitStatus};

/// Herramienta de mantenimiento para sistemas Debian.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Si se incluye, ejecuta 'apt autoremove' después de actualizar
    #[arg(short, long)]
    autoremove: bool,
}

/// Verifica si un comando existe en el sistema
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn run_command(program: &str, args: &[&str]) -> io::Result<ExitStatus> {
    println!("\n==> Ejecutando: {} {}", program, args.join(" "));

    let mut child = Command::new(program).args(args).spawn()?;

    child.wait()
}

fn main() {
    let args = Args::parse();

    println!(
        "{}",
        "--- Gestor de Mantenimiento y Actualización (Debian) ---".cyan()
    );

    // 1. APT: Siempre se asume presente en Debian
    println!(
        "\n{}",
        "[1] Actualizando paquetes del sistema (APT)...".cyan()
    );
    let _ = run_command("sudo", &["apt", "update"]);
    let _ = run_command("sudo", &["apt", "upgrade", "-y"]);

    if args.autoremove {
        println!("--> Ejecutando limpieza profunda de APT (autoremove)...");
        let _ = run_command("sudo", &["apt", "autoremove", "-y"]);
    }

    // 2. Flatpaks
    if command_exists("flatpak") {
        println!("\n{}", "[2] Gestionando Flatpaks...".cyan());
        let _ = run_command("flatpak", &["update", "-y"]);
        let _ = run_command("flatpak", &["uninstall", "--unused", "-y"]);
    } else {
        println!(
            "\n{}",
            "[2] Saltando Flatpak: No se encontró el binario.".red()
        );
    }

    // 3. Snaps
    if command_exists("snap") {
        println!("\n{}", "[3] Gestionando Snaps...".cyan());
        let _ = run_command("sudo", &["snap", "refresh"]);
    } else {
        println!(
            "\n{}",
            "[3] Saltando Snap: No se encontró el binario.".red()
        );
    }

    // 4. Rustup
    if command_exists("rustup") {
        println!("\n{}", "[4] Actualizando Rust...".cyan());
        let _ = run_command("rustup", &["update"]);
    } else {
        println!(
            "\n{}",
            "[4] Saltando Rustup: No se encontró el binario.".red()
        );
    }

    // 5. NPM Global
    if command_exists("npm") {
        println!(
            "\n{}",
            "[5] Actualizando paquetes globales de NPM...".cyan()
        );
        let _ = run_command("sudo", &["npm","update", "-g"]);
    } else {
        println!("\n{}", "[5] Saltando NPM: No se encontró el binario.".red());
    }

    println!("\n{}\n", "--- ¡Proceso finalizado! ---".cyan());
}
