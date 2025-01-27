use anyhow::{Context, Error};
use webc::compat::Container;

use crate::runtime::resolver::{
    DistributionInfo, PackageInfo, PackageSpecifier, PackageSummary, Source, WebcHash,
};

/// A [`Source`] that knows how to query files on the filesystem.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FileSystemSource {}

#[async_trait::async_trait]
impl Source for FileSystemSource {
    #[tracing::instrument(level = "debug", skip_all, fields(%package))]
    async fn query(&self, package: &PackageSpecifier) -> Result<Vec<PackageSummary>, Error> {
        let path = match package {
            PackageSpecifier::Path(path) => path.canonicalize().with_context(|| {
                format!(
                    "Unable to get the canonical form for \"{}\"",
                    path.display()
                )
            })?,
            _ => return Ok(Vec::new()),
        };

        // FIXME: These two operations will block
        let webc_sha256 = WebcHash::for_file(&path)
            .with_context(|| format!("Unable to hash \"{}\"", path.display()))?;
        let container = Container::from_disk(&path)
            .with_context(|| format!("Unable to parse \"{}\"", path.display()))?;

        let url = crate::runtime::resolver::utils::url_from_file_path(&path)
            .ok_or_else(|| anyhow::anyhow!("Unable to turn \"{}\" into a URL", path.display()))?;

        let summary = PackageSummary {
            pkg: PackageInfo::from_manifest(container.manifest())?,
            dist: DistributionInfo {
                webc: url,
                webc_sha256,
            },
        };

        Ok(vec![summary])
    }
}
