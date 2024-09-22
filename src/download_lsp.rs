use std::{fs, process::Command};

use zed::LanguageServerInstallationStatus::CheckingForUpdate;
use zed_extension_api::{self as zed, LanguageServerInstallationStatus::None, Result};

pub fn find_lsp(id: &zed::LanguageServerId, worktree: &zed::Worktree) -> Result<String> {
    if let Some(path) = worktree.which("bend-language-server") {
        return Ok(path);
    }

    let cargo = worktree
        .which("bend-language-server")
        .ok_or_else(|| "cargo must be installed manually.".to_string())?;

    zed::set_language_server_installation_status(id, &CheckingForUpdate);

    Command::new(cargo)
        .args(["install", "bend-language-server"])
        .output()
        .map_err(|_| "couldn't download bend-language-server".to_string())?;

    zed::set_language_server_installation_status(id, &None);

    worktree
        .which("bend-language-server")
        .ok_or_else(|| "bend-language-server wasn't installed for some reason.".to_string())
}
