use axum::http::StatusCode;
use axum::response::Json;
use axum::{
    Router,
    routing::{get, post},
};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tempfile::tempdir;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorisBody {
    pub bx: f32,
    pub by: f32,
    pub bz: f32,
    pub ex: f32,
    pub ey: f32,
    pub ez: f32,
    pub dt: f32,
    pub steps: usize,
    pub posx: f32,
    pub posy: f32,
    pub posz: f32,
    pub velx: f32,
    pub vely: f32,
    pub velz: f32,
    pub q: f32,
    pub m: f32,
    pub plot_dim: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BorisResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub image_base64: Option<String>,
}

async fn boris_check() -> Json<String> {
    Json("Boris simulation started!".to_string())
}

async fn boris_run(
    Json(payload): Json<BorisBody>,
) -> Result<Json<BorisResponse>, (StatusCode, String)> {
    let work_dir = tempdir().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create temporary workspace: {}", e),
        )
    })?;

    let root_dir = std::env::current_dir().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get current directory: {}", e),
        )
    })?;

    let source_plot_jl = root_dir.join("tools/boris-cli/plot.jl");
    let dest_plot_jl = work_dir.path().join("plot.jl");

    if let Err(e) = std::fs::copy(&source_plot_jl, &dest_plot_jl) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to copy plot.jl into workspace: {}", e),
        ));
    }

    let binary_path = root_dir.join("tools/boris-cli/target/release/boris-cli");
    let mut child = Command::new(binary_path)
        .current_dir(work_dir.path())
        .env("GKSwstype", "100")
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to spawn boris-cli: {}", e),
            )
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        let inputs = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            payload.bx,
            payload.by,
            payload.bz,
            payload.ex,
            payload.ey,
            payload.ez,
            payload.dt,
            payload.steps,
            payload.posx,
            payload.posy,
            payload.posz,
            payload.velx,
            payload.vely,
            payload.velz,
            payload.q,
            payload.m,
            payload.plot_dim
        );

        tokio::spawn(async move {
            let _ = stdin.write_all(inputs.as_bytes()).await;
        });
    }

    let execution_result = timeout(Duration::from_secs(60), child.wait_with_output()).await;

    let output = match execution_result {
        Ok(Ok(out)) => out,
        Ok(Err(e)) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Process failed during execution: {}", e),
            ))
        }
        Err(_) => {
            return Err((
                StatusCode::GATEWAY_TIMEOUT,
                "The CLI took longer than 60 seconds and was killed. Check if it is waiting for unhandled inputs.".into(),
            ))
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let julia_execution = Command::new("julia")
        .arg("plot.jl")
        .current_dir(work_dir.path())
        .env("GKSwstype", "100")
        .output()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute Julia: {}", e),
            )
        })?;

    if !julia_execution.status.success() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Julia execution failed: {}",
                String::from_utf8_lossy(&julia_execution.stderr)
            ),
        ));
    }

    let plot_path = work_dir.path().join("trajectory_plot.png");
    let image_base64 = if plot_path.exists() {
        tokio::fs::read(&plot_path)
            .await
            .ok()
            .map(|bytes| base64::engine::general_purpose::STANDARD.encode(bytes))
    } else {
        None
    };

    Ok(Json(BorisResponse {
        stdout,
        stderr,
        exit_code: output.status.code(),
        image_base64,
    }))
}

pub fn boris_routes() -> Router {
    Router::new()
        .route("/", get(boris_check))
        .route("/run", post(boris_run))
}
