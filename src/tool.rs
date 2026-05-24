#[cfg(not(target_os = "macos"))]
compile_error!("applescript_tool only supports macOS");


use rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tokio::process::Command;

// ── Errors ───────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum AppleScriptError {

    #[error("Script timed out after {0}s")]
    Timeout(u64),

    #[error("Script failed (exit {code}): {stderr}")]
    ScriptFailed { code: i32, stderr: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// ── Tool args ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AppleScriptArgs {
    /// The AppleScript code to execute.
    pub script: String,

    /// Timeout in seconds. Defaults to 10.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    10
}

// ── Tool output ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AppleScriptOutput {
    pub output: String,
}

impl std::fmt::Display for AppleScriptOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.output.is_empty() {
            write!(f, "(script ran, no output)")
        } else {
            write!(f, "{}", self.output)
        }
    }
}

// ── Tool ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct AppleScriptTool;

impl Tool for AppleScriptTool {
    const NAME: &'static str = "run_applescript";

    type Error = AppleScriptError;
    type Args = AppleScriptArgs;
    type Output = AppleScriptOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "\
                Run an AppleScript on the macOS system. \
                Use this to control apps, read system info, send notifications, \
                manipulate files, or automate UI. \
                Returns the script's output text, or an error if the script failed. \
                Only works on macOS."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "script": {
                        "type": "string",
                        "description": "Complete, valid AppleScript code to execute."
                    },
                    "timeout_secs": {
                        "type": "integer",
                        "description": "Max seconds to wait before killing the process. Default: 10.",
                        "default": 10
                    }
                },
                "required": ["script"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        #[cfg(not(target_os = "macos"))] {
    return Err(AppleScriptError::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        "applescript_tool only supports macOS",
    )));
        }
    let max_attempts = 7;
    let base_backoff_ms = 500;
    #[cfg(target_os = "macos")]
    for attempt in 1..=max_attempts {
    let timeout = std::time::Duration::from_secs(args.timeout_secs);
    println!("Attempt {}/{}", attempt, max_attempts);

    let child = Command::new("osascript")
        .args(["-e", &args.script])
        .output();

    match tokio::time::timeout(timeout, child).await {
        Err(_) => {
            if attempt == max_attempts {
                return Err(AppleScriptError::Timeout(args.timeout_secs));
            }
        }
        Ok(Err(e)) => {
            if attempt == max_attempts {
                return Err(AppleScriptError::Io(e));
            }
        }
        Ok(Ok(out)) => {
            if out.status.success() {
                return Ok(AppleScriptOutput {
                    output: String::from_utf8_lossy(&out.stdout).trim().to_string(),
                });
            } else if attempt == max_attempts {
                // only give up on last attempt
                return Err(AppleScriptError::ScriptFailed {
                    code: out.status.code().unwrap_or(-1),
                    stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
                });
            }
            // else: fall through to backoff and retry
        }
    }

    let backoff_ms = base_backoff_ms * (1u64 << ((attempt - 1) as u32));
    tokio::time::sleep(std::time::Duration::from_millis(backoff_ms)).await;
}
        unreachable!();

        
    
    }
    }
