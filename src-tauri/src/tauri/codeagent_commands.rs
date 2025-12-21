//! codeagent-wrapper commands
//!
//! Integrates the `codeagent-wrapper` binary published by https://github.com/cexll/myclaude
//! as an executable CLI from the Rust backend.

use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tauri::async_runtime;
use tracing::info;

#[derive(Debug, Serialize)]
pub struct CliExecResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

fn is_executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, rely on file extension and existence.
        return path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("exe"))
            .unwrap_or(false);
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = std::fs::metadata(path) {
            return (meta.permissions().mode() & 0o111) != 0;
        }
        false
    }
}

fn which_in_path(program: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(program);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn find_codeagent_wrapper(explicit_path: Option<String>) -> Result<PathBuf, String> {
    if let Some(p) = explicit_path {
        let pb = PathBuf::from(p);
        if pb.exists() {
            return Ok(pb);
        }
        return Err("指定的 codeagent-wrapper 路径不存在".to_string());
    }

    #[cfg(target_os = "windows")]
    let program_name = "codeagent-wrapper.exe";
    #[cfg(not(target_os = "windows"))]
    let program_name = "codeagent-wrapper";

    if let Some(p) = which_in_path(program_name) {
        return Ok(p);
    }

    let home = dirs::home_dir().ok_or_else(|| "无法获取用户 HOME 目录".to_string())?;

    // myclaude README: Windows installer places it under %USERPROFILE%\bin
    let candidates = [
        home.join("bin").join(program_name),
        home.join(".claude").join("bin").join(program_name),
    ];

    for c in candidates {
        if c.exists() {
            return Ok(c);
        }
    }

    Err(
        "未找到 codeagent-wrapper。请先安装 myclaude（或将 codeagent-wrapper 放入 PATH / $HOME/bin / $HOME/.claude/bin），或在调用时传入 binary_path。"
            .to_string(),
    )
}

/// Execute `codeagent-wrapper` (myclaude release binary).
///
/// Notes:
/// - Do NOT pass secrets via args; pass them via `env`.
/// - `binary_path` is optional; if omitted we will try PATH and common install locations.
#[tauri::command]
pub async fn execute_codeagent_wrapper(
    binary_path: Option<String>,
    args: Vec<String>,
    cwd: Option<String>,
    env: Option<HashMap<String, String>>,
) -> Result<CliExecResult, String> {
    // Avoid logging arguments/env to prevent secret leakage.
    info!("Executing codeagent-wrapper (args_len={})", args.len());

    async_runtime::spawn_blocking(move || {
        let bin = find_codeagent_wrapper(binary_path)?;
        if !is_executable_file(&bin) {
            return Err(format!("codeagent-wrapper 不是可执行文件: {}", bin.display()));
        }

        let mut cmd = std::process::Command::new(&bin);
        cmd.args(&args);

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        if let Some(map) = env {
            for (k, v) in map {
                cmd.env(k, v);
            }
        }

        let output = cmd.output().map_err(|e| e.to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok(CliExecResult {
            stdout,
            stderr,
            exit_code,
        })
    })
    .await
    .map_err(|e| format!("执行 codeagent-wrapper 任务失败: {}", e))?
}
