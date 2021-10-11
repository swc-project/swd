use anyhow::{anyhow, Context, Result};
use cargo_metadata::MetadataCommand;
use std::{
    env,
    path::{Path, PathBuf},
};
use tokio::task::spawn_blocking;

pub mod add;
pub mod upgrade;

pub async fn cargo_metadata(
    mut cmd: MetadataCommand,
    from: &Path,
) -> Result<cargo_metadata::Metadata> {
    let from = from.to_path_buf();
    spawn_blocking(move || {
        let result = cmd
            .current_dir(&from)
            .exec()
            .context("failed to execute `cargo metadata`")?;

        Ok(result)
    })
    .await
    .context("failed to join the task for `cargo metadata`")?
}

pub async fn swc_build_dir() -> Result<PathBuf> {
    let from = env::current_dir().context("failed to get current dir")?;

    let cargo_target = cargo_target_dir(&from).await?;

    Ok(cargo_target.join(".swc"))
}

pub async fn cargo_target_dir(from: &Path) -> Result<PathBuf> {
    let mut cmd = MetadataCommand::new();
    cmd.no_deps();
    let md = cargo_metadata(cmd, from).await?;

    Ok(md.target_directory.as_std_path().to_path_buf())
}

pub fn get_default_cargo_target_sync() -> Result<String> {
    use std::process::Command;

    let output = Command::new("rustc")
        .arg("-vV")
        .output()
        .context("Failed to run rustc to get the host target")?;
    let output =
        String::from_utf8(output.stdout).context("`rustc -vV` didn't return utf8 output")?;

    let field = "host: ";
    let host = output
        .lines()
        .find(|l| l.starts_with(field))
        .map(|l| &l[field.len()..])
        .ok_or_else(|| {
            anyhow!(
                "`rustc -vV` didn't have a line for `{}`, got:\n{}",
                field.trim(),
                output
            )
        })?
        .to_string();
    Ok(host)
}
