use std::fs;
use std::path::PathBuf;

use zed_extension_api::{self as zed, settings::LspSettings, Result};

#[derive(Default)]
struct TreeSitterQueryExtension {
    cached_binary_path: Option<PathBuf>,
}

impl TreeSitterQueryExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<PathBuf> {
        if let Some(path) = &self.cached_binary_path {
            if path.is_file() {
                return Ok(path.clone());
            }
        }

        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path.filter(|p| !p.trim().is_empty()) {
                let path_buf = PathBuf::from(path);
                self.cached_binary_path = Some(path_buf.clone());
                return Ok(path_buf);
            }
        }

        if let Some(path) = worktree.which("ts_query_ls") {
            let path_buf = PathBuf::from(path);
            self.cached_binary_path = Some(path_buf.clone());
            return Ok(path_buf);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "ribru17/ts_query_ls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let version = release.version;

        let (platform, arch) = zed::current_platform();
        let binary_filename = if matches!(platform, zed::Os::Windows) {
            "ts_query_ls.exe"
        } else {
            "ts_query_ls"
        };

        let asset_stem = match (platform, arch) {
            (zed::Os::Linux, zed::Architecture::Aarch64) => "ts_query_ls-aarch64-unknown-linux-gnu",
            (zed::Os::Linux, zed::Architecture::X8664) => "ts_query_ls-x86_64-unknown-linux-gnu",
            (zed::Os::Mac, zed::Architecture::Aarch64) => "ts_query_ls-aarch64-apple-darwin",
            (zed::Os::Mac, zed::Architecture::X8664) => "ts_query_ls-x86_64-apple-darwin",
            (zed::Os::Windows, _) => "ts_query_ls-x86_64-pc-windows-msvc",
            (unsupported_os, unsupported_arch) => {
                return Err(format!(
                    "Unsupported OS {unsupported_os:?} and architecture {unsupported_arch:?} combination",
                ));
            }
        };
        let asset_name = format!(
            "{asset_stem}.{suffix}",
            suffix = match platform {
                zed::Os::Windows => "zip",
                _ => "tar.gz",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name}"))?;

        let version_dir = format!("ts_query_ls-{version}");
        let binary_path = PathBuf::from(&format!("{version_dir}/{binary_filename}"));

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let file_kind = match platform {
                zed::Os::Windows => zed::DownloadedFileType::Zip,
                _ => zed::DownloadedFileType::GzipTar,
            };
            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for TreeSitterQueryExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command_path = self.language_server_binary_path(language_server_id, worktree)?;
        Ok(zed::Command {
            command: command_path.to_string_lossy().into_owned(),
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(Some(
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_default(),
        ))
    }
}

zed::register_extension!(TreeSitterQueryExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::Extension;

    #[test]
    fn test_new_extension_initial_state() {
        let ext = TreeSitterQueryExtension::new();
        assert!(
            ext.cached_binary_path.is_none(),
            "A new extension instance should have no cached binary path by default."
        );
    }
}
