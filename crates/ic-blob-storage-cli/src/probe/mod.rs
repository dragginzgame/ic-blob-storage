mod record;
#[cfg(test)]
mod tests;
use record::{MAX_BODY, MAX_REQUESTS, PlanRecord, RequestRecord, ResponseRecord, SummaryRecord};
use std::{path::Path, process::ExitCode, time::Duration};

const COMMIT_URL: &str = "https://api.github.com/repos/caffeinelabs/skills/commits/main";
const PACKAGE_URL: &str = "https://registry.npmjs.org/@caffeineai%2fobject-storage/latest";
const LIMITATION: &str = "Public source/registry observations only. No gateway, account, payment or deployed behavioral qualification. Capture success is not provider acceptance.";

pub(super) fn run() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--help"] {
        println!(
            "caffeine-probe public-source NEW_DIRECTORY | verify DIRECTORY\nCaptures at most four anonymous HTTPS GETs: official skills commit, pinned Mixin.mo and Storage.mo, npm latest metadata. No retries, redirects, credentials, provider calls or payments. Every request is recorded before dispatch. Directory must not exist. Failed/interrupted runs remain; never resume them automatically. Verify checks artifact integrity, not authenticity or qualification."
        );
        return ExitCode::SUCCESS;
    }
    let result = match args.as_slice() {
        [command, path] if command == "public-source" => capture(Path::new(path)).map(
            |()| serde_json::json!({"outcome":"captured","directory":path,"limitation":LIMITATION}),
        ),
        [command, path] if command == "verify" => record::verify(Path::new(path)),
        _ => Err("arguments".into()),
    };
    match result {
        Ok(value) => {
            println!("{value}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            println!("{}", serde_json::json!({"error":error}));
            ExitCode::from(3)
        }
    }
}

fn plan() -> Result<PlanRecord, String> {
    Ok(PlanRecord {schema:1,started_unix_seconds:record::now()?,runner_version:env!("CARGO_PKG_VERSION").into(),
        runner_source_sha256:record::hash(concat!(include_str!("mod.rs"),include_str!("record/mod.rs"),include_str!("../probe_main.rs"),include_str!("../../Cargo.toml"),include_str!("../../../../Cargo.lock")).as_bytes()),
        kind:"public_source".into(),identity:"anonymous".into(),max_requests:MAX_REQUESTS,max_response_bytes:MAX_BODY,request_deadline_seconds:30,
        attached_cycles:"0".into(),provider_account:None,
        targets:vec![COMMIT_URL.into(),"https://raw.githubusercontent.com/caffeinelabs/skills/{observed_commit}/packages/object-storage/backend/src/Mixin.mo".into(),"https://raw.githubusercontent.com/caffeinelabs/skills/{observed_commit}/packages/object-storage/backend/src/Storage.mo".into(),PACKAGE_URL.into()]})
}
fn capture(directory: &Path) -> Result<(), String> {
    let plan = plan()?;
    std::fs::create_dir(directory).map_err(|_| "new_directory_required")?;
    let parent = directory
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    std::fs::File::open(parent)
        .and_then(|f| f.sync_all())
        .map_err(|_| "persist_run_directory")?;
    record::json(directory, "plan.json", &plan)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| "runtime")?;
    runtime.block_on(async {
        let client = client_builder().build().map_err(|_| "client")?;
        let mut attempted = 0;
        let mut commit = None;
        let result = sequence(directory, &client, &mut attempted, &mut commit).await;
        record::json(
            directory,
            "summary.json",
            &SummaryRecord {
                finished_unix_seconds: record::now()?,
                outcome: if result.is_ok() { "captured" } else { "failed" }.into(),
                requests_attempted: attempted,
                source_commit: commit,
                limitation: LIMITATION.into(),
            },
        )?;
        result
    })
}
fn client_builder() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(Duration::from_secs(30))
        .user_agent("ic-blob-storage-caffeine-probe")
}
fn commit(bytes: &[u8]) -> Result<String, String> {
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|_| "commit_json")?;
    let sha = value
        .get("sha")
        .and_then(serde_json::Value::as_str)
        .ok_or("commit_sha")?;
    if sha.len() != 40
        || !sha
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("commit_sha".into());
    }
    Ok(sha.into())
}
async fn sequence(
    directory: &Path,
    client: &reqwest::Client,
    attempted: &mut usize,
    observed: &mut Option<String>,
) -> Result<(), String> {
    let metadata = fetch(directory, client, COMMIT_URL, attempted).await?;
    let sha = commit(&metadata)?;
    *observed = Some(sha.clone());
    for file in ["Mixin.mo", "Storage.mo"] {
        fetch(directory,client,&format!("https://raw.githubusercontent.com/caffeinelabs/skills/{sha}/packages/object-storage/backend/src/{file}"),attempted).await?;
    }
    let package = fetch(directory, client, PACKAGE_URL, attempted).await?;
    let metadata: serde_json::Value =
        serde_json::from_slice(&package).map_err(|_| "package_json")?;
    if metadata.get("name").and_then(serde_json::Value::as_str)
        != Some("@caffeineai/object-storage")
        || metadata
            .get("version")
            .and_then(serde_json::Value::as_str)
            .is_none()
    {
        return Err("package_metadata".into());
    }
    Ok(())
}
async fn fetch(
    directory: &Path,
    client: &reqwest::Client,
    url: &str,
    attempted: &mut usize,
) -> Result<Vec<u8>, String> {
    let index = *attempted;
    if index >= MAX_REQUESTS {
        return Err("request_limit".into());
    }
    record::json(
        directory,
        &format!("request-{index}.json"),
        &RequestRecord {
            index,
            started_unix_seconds: record::now()?,
            method: "GET".into(),
            url: url.into(),
        },
    )?;
    *attempted += 1;
    let mut body = Vec::new();
    let mut status = None;
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        receive(client, url, &mut status, &mut body),
    )
    .await;
    let outcome = match result {
        Err(_) => "timeout",
        Ok(Err(error)) => error,
        Ok(Ok(())) => "captured",
    };
    record::save(directory, &format!("response-{index}.body"), &body)?;
    record::json(
        directory,
        &format!("response-{index}.json"),
        &ResponseRecord {
            index,
            finished_unix_seconds: record::now()?,
            status,
            outcome: outcome.into(),
            bytes: body.len(),
            sha256: record::hash(&body),
        },
    )?;
    if outcome != "captured" {
        return Err(outcome.into());
    }
    Ok(body)
}
async fn receive(
    client: &reqwest::Client,
    url: &str,
    status: &mut Option<u16>,
    body: &mut Vec<u8>,
) -> Result<(), &'static str> {
    let mut response = client.get(url).send().await.map_err(|_| "transport")?;
    *status = Some(response.status().as_u16());
    while let Some(chunk) = response.chunk().await.map_err(|_| "body_transport")? {
        let remaining = MAX_BODY - body.len();
        body.extend_from_slice(&chunk[..chunk.len().min(remaining)]);
        if chunk.len() > remaining {
            return Err("body_limit");
        }
    }
    if !response.status().is_success() {
        return Err("http_status");
    }
    Ok(())
}
