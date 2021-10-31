use self::{
    artifacts::ArtifactsCommand, build::BuildCommand, init::InitCommand, package::PackageCommand,
    upgrade_deps::UpgradeDepsCommand,
};
use anyhow::{Context, Error};
use structopt::StructOpt;

pub mod artifacts;
pub mod build;
pub mod init;
pub mod package;
pub mod upgrade_deps;

/// Manages the plugin. Used for developing plugins.
#[derive(Debug, StructOpt)]
pub enum PluginCommand {
    Init(InitCommand),
    Build(BuildCommand),
    Package(PackageCommand),
    Artifacts(ArtifactsCommand),
    UpgradeDeps(UpgradeDepsCommand),
}

impl PluginCommand {
    pub fn run(self) -> Result<(), Error> {
        match self {
            PluginCommand::Init(cmd) => {
                cmd.run().context("failed to initialize a plugin project")?;
            }
            PluginCommand::Build(cmd) => {
                cmd.run()?;
            }
            PluginCommand::Package(cmd) => {
                cmd.run()?;
            }
            PluginCommand::Artifacts(cmd) => {
                cmd.run()?;
            }
            PluginCommand::UpgradeDeps(cmd) => {
                cmd.run().context("failed to upgrade dependencies")?;
            }
        }

        Ok(())
    }
}
