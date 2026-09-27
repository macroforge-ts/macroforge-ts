//! Registry polling for npm, crates.io, and JSR
//!
//! Uses HTTP APIs directly instead of shelling out to npm/cargo.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Create a ureq agent with reasonable timeouts to prevent hanging
fn agent() -> ureq::Agent {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(30)))
        .timeout_connect(Some(Duration::from_secs(10)))
        .build();
    ureq::Agent::new_with_config(config)
}

/// npm registry response for package metadata
#[derive(Deserialize)]
struct NpmPackageResponse {
    version: Option<String>,
}

/// crates.io API response
#[derive(Deserialize)]
struct CratesResponse {
    #[serde(rename = "crate")]
    crate_info: Option<CrateInfo>,
}

#[derive(Deserialize)]
struct CrateInfo {
    max_version: String,
}

/// Get the latest version of an npm package via registry API
pub fn npm_version(package_name: &str) -> Result<Option<String>> {
    let url = format!("https://registry.npmjs.org/{}/latest", package_name);

    match agent().get(&url).call() {
        Ok(resp) => {
            if resp.status() == 404 {
                return Ok(None);
            }
            let pkg: NpmPackageResponse = resp.into_body().read_json()?;
            Ok(pkg.version)
        }
        Err(ureq::Error::StatusCode(404)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// JSR API response for package metadata
#[derive(Deserialize)]
struct JsrPackageResponse {
    latest: Option<String>,
}

/// Get the latest version of a JSR package via API
pub fn jsr_version(package_name: &str) -> Result<Option<String>> {
    // JSR API expects @scope/name -> @scope%2Fname in URL path,
    // but the meta endpoint uses @scope/name directly
    let url = format!("https://jsr.io/{}/meta.json", package_name);

    match agent().get(&url).call() {
        Ok(resp) => {
            if resp.status() == 404 {
                return Ok(None);
            }
            let pkg: JsrPackageResponse = resp.into_body().read_json()?;
            Ok(pkg.latest)
        }
        Err(ureq::Error::StatusCode(404)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Get the latest version of a crates.io package via API
pub fn crates_version(crate_name: &str) -> Result<Option<String>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);

    match agent().get(&url).header("User-Agent", "mf-cli").call() {
        Ok(resp) => {
            if resp.status() == 404 {
                return Ok(None);
            }
            let crates_resp: CratesResponse = resp.into_body().read_json()?;
            Ok(crates_resp.crate_info.map(|c| c.max_version))
        }
        Err(ureq::Error::StatusCode(404)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// crates.io's trusted publishing endpoint: POST exchanges, DELETE revokes.
const CRATES_TRUSTED_PUBLISHING_TOKENS: &str = "https://crates.io/api/v1/trusted_publishing/tokens";

#[derive(Serialize)]
struct TrustedPublishingRequest<'a> {
    jwt: &'a str,
}

#[derive(Deserialize)]
struct TrustedPublishingResponse {
    token: String,
}

/// Exchanges a CI OIDC ID token for a crates.io publish token. The ID token is
/// single-use and the publish token expires after 30 minutes.
pub fn crates_trusted_publishing_token(id_token: &str) -> Result<String> {
    let mut response = agent()
        .post(CRATES_TRUSTED_PUBLISHING_TOKENS)
        .config()
        .http_status_as_error(false)
        .build()
        .header("User-Agent", "mf-cli")
        .send_json(TrustedPublishingRequest { jwt: id_token })
        .context("crates.io trusted publishing request failed")?;
    let status = response.status();
    if !status.is_success() {
        let body = response
            .body_mut()
            .read_to_string()
            .context("failed to read the crates.io error response")?;
        anyhow::bail!("crates.io refused the trusted publishing exchange ({status}): {body}");
    }
    let exchanged: TrustedPublishingResponse = response
        .body_mut()
        .read_json()
        .context("unexpected crates.io trusted publishing response")?;
    Ok(exchanged.token)
}

/// Revokes a publish token from [`crates_trusted_publishing_token`].
pub fn revoke_crates_trusted_publishing_token(token: &str) -> Result<()> {
    agent()
        .delete(CRATES_TRUSTED_PUBLISHING_TOKENS)
        .header("User-Agent", "mf-cli")
        .header("Authorization", &format!("Bearer {token}"))
        .call()
        .context("crates.io did not revoke the trusted publishing token")?;
    Ok(())
}
