//! Registry polling for npm and crates.io
//!
//! Uses HTTP APIs directly instead of shelling out to npm/cargo.

use anyhow::Result;
use serde::Deserialize;

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

    match ureq::get(&url).call() {
        Ok(response) => {
            let pkg: NpmPackageResponse = response.into_json()?;
            Ok(pkg.version)
        }
        Err(ureq::Error::Status(404, _)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Get the latest version of a crates.io package via API
pub fn crates_version(crate_name: &str) -> Result<Option<String>> {
    let url = format!("https://crates.io/api/v1/crates/{}", crate_name);

    match ureq::get(&url)
        .set("User-Agent", "mf-cli (github.com/anthropics/macroforge)")
        .call()
    {
        Ok(response) => {
            let resp: CratesResponse = response.into_json()?;
            Ok(resp.crate_info.map(|c| c.max_version))
        }
        Err(ureq::Error::Status(404, _)) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
