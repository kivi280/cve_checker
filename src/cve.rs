use chrono::{DateTime, Utc};
use serde::{Deserialize};

#[derive(Debug)]
struct Output {
    cve: Cve,
    json_file: String,
}

#[derive( Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cve {
    data_type: String,
    cve_metadata: Metadata,
    containers: Container
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    cve_id: String,
    assigner_org_id: String,
    state: String,
    assigner_short_name: Option<String>,
    date_reserved: Option<DateTime<Utc>>,
    date_published: Option<DateTime<Utc>>,
    date_updated: Option<DateTime<Utc>>
}
#[derive(Deserialize, Debug)]
struct Container {
    cna: Cna,
}

#[derive(Deserialize, Debug)]
struct Cna {
    descriptions: Vec<Description>,
    affected: Vec<AffectedProduct>,
    title: Option<String>,
    references: Vec<Reference>
}

#[derive(Deserialize, Debug)]
struct Description {
    lang: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct AffectedProduct {
    vendor: String,
    product: String,
    versions: Option<Vec<Version>>,
    platforms: Option<Vec<String>>
}

// #[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
struct  Version {
    version: String,
    status: String,
    #[serde(rename(deserialize = "lessThanOrEqual", serialize = "lessThanOrEqual"))]
    less_than_or_equal: Option<String>,
    #[serde(rename(deserialize = "lessThan", serialize = "lessThan"))]
    less_than: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Reference {
    url: String,
}

impl std::fmt::Display for Cve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.cve_metadata.cve_id)
        }
}

//https://github.com/CVEProject/cvelistV5
//https://github.com/CVEProject/cvelistV5/blob/main/cves/2025/0xxx/CVE-2025-0058.json
//https://github.com/CVEProject/cvelistV5/blob/main/cves/2025/0xxx/CVE-2025-0051.json