use std::process::Command;
use anyhow::{Result, anyhow};

/// Returns the output of the compiled c file
pub fn build_c(target: &str, output: &str) -> Result<()> {
    let mut child = Command::new("gcc")
        .args([
            "-O3",
            "-o",
            output,
            target,
        ])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| anyhow!("Error spawning `gcc` process: {}", e))?;

    child.wait().map(|e| anyhow!("Error waiting on `gcc` process: {}", e))?;    
    
    Ok(())
}