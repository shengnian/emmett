// Specification: https://www.elastic.co/guide/en/logstash/current/plugins-inputs-google_cloud_storage.html

use std::path::Path;

/// Extracts events from files in a Google Cloud Storage bucket.
pub struct GoogleCloudStorage {
    pub bucket_id: String,
    pub json_key_file: Option<&'static Path>,
    pub interval: Option<u64>,
    pub file_matches: Option<String>,
    pub file_exclude: Option<String>,
    pub metadata_key: Option<String>,
    pub processed_db_path: Option<&'static Path>,
    pub delete: Option<bool>,
    pub unpack_gzip: Option<bool>,
}
