//! Registry polling for npm and crates.io
//!
//! Uses HTTP APIs directly instead of shelling out to npm/cargo.

use anyhow::Result;
use serde::Deserialize;
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
