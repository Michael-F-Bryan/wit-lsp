//! Disk discovery for the language server. Open buffers are owned by the server.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

use tower_lsp::lsp_types::{InitializeParams, Url};

#[derive(Debug, Clone, Default)]
pub(crate) struct WorkspaceFiles {
    roots: BTreeSet<PathBuf>,
}

impl WorkspaceFiles {
    #[allow(deprecated)] // Older clients send root_uri or root_path instead of folders.
    pub fn initialize(&mut self, params: &InitializeParams) {
        if let Some(folders) = &params.workspace_folders {
            for folder in folders {
                self.add(&folder.uri);
            }
        } else if let Some(uri) = &params.root_uri {
            self.add(uri);
        } else if let Some(path) = &params.root_path
            && Path::new(path).is_absolute()
        {
            self.roots.insert(PathBuf::from(path));
        }
    }

    pub fn add(&mut self, uri: &Url) {
        if let Ok(path) = uri.to_file_path() {
            self.roots.insert(path);
        }
    }

    pub fn remove(&mut self, uri: &Url) {
        if let Ok(path) = uri.to_file_path() {
            self.roots.remove(&path);
        }
    }

    pub fn contains(&self, uri: &Url) -> bool {
        uri.to_file_path()
            .ok()
            .is_some_and(|path| self.roots.iter().any(|root| eligible(root, &path)))
    }

    pub async fn scan(&self) -> Result<BTreeMap<String, String>, tokio::task::JoinError> {
        let roots = self.roots.clone();
        tokio::task::spawn_blocking(move || {
            let mut files = BTreeMap::new();
            let mut visited = BTreeSet::new();
            let mut pending: Vec<_> = roots.into_iter().collect();
            while let Some(directory) = pending.pop() {
                if !visited.insert(directory.clone()) {
                    continue;
                }
                let entries = match fs::read_dir(&directory) {
                    Ok(entries) => entries,
                    Err(error) => {
                        tracing::warn!(path = %directory.display(), %error, "Cannot scan WIT directory");
                        continue;
                    }
                };
                for entry in entries {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(error) => {
                            tracing::warn!(path = %directory.display(), %error, "Cannot read directory entry");
                            continue;
                        }
                    };
                    let path = entry.path();
                    let kind = match entry.file_type() {
                        Ok(kind) => kind,
                        Err(error) => {
                            tracing::warn!(path = %path.display(), %error, "Cannot inspect WIT path");
                            continue;
                        }
                    };
                    if kind.is_dir() && !excluded(&entry.file_name()) {
                        pending.push(path);
                    } else if kind.is_file() && is_wit(&path)
                        && let Ok(uri) = Url::from_file_path(&path)
                        && let Some(text) = read_file(&path)
                    {
                        files.insert(uri.to_string(), text);
                    }
                }
            }
            files
        }).await
    }

    pub async fn read(
        &self,
        uris: Vec<Url>,
    ) -> Result<Vec<(String, Option<String>)>, tokio::task::JoinError> {
        let roots = self.roots.clone();
        tokio::task::spawn_blocking(move || {
            uris.into_iter()
                .map(|uri| {
                    let text = uri
                        .to_file_path()
                        .ok()
                        .filter(|path| is_wit(path))
                        .filter(|path| {
                            roots
                                .iter()
                                .any(|root| eligible(root, path) && no_symlinks(root, path))
                        })
                        .and_then(|path| read_file(&path));
                    (uri.to_string(), text)
                })
                .collect()
        })
        .await
    }
}

fn is_wit(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "wit")
}

fn eligible(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root).is_ok_and(|relative| {
        relative.components().all(|part| match part {
            Component::Normal(name) => !excluded(name),
            _ => false,
        })
    })
}

fn excluded(name: &std::ffi::OsStr) -> bool {
    matches!(
        name.to_str(),
        Some(
            ".git"
                | ".hg"
                | ".svn"
                | ".worktrees"
                | ".direnv"
                | "target"
                | "build"
                | "dist"
                | "node_modules"
                | ".venv"
                | "venv"
                | ".cache"
                | ".yarn"
                | ".pnpm-store"
        )
    )
}

fn no_symlinks(root: &Path, path: &Path) -> bool {
    path.ancestors()
        .take_while(|part| *part != root)
        .all(|part| {
            fs::symlink_metadata(part).is_ok_and(|metadata| !metadata.file_type().is_symlink())
        })
}

fn read_file(path: &Path) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(text) => Some(text),
        Err(error) => {
            if error.kind() != std::io::ErrorKind::NotFound {
                tracing::warn!(path = %path.display(), %error, "Cannot read WIT file");
            }
            None
        }
    }
}
