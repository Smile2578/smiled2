use aws_config::{BehaviorVersion, Region};
use aws_credential_types::{
    provider::SharedCredentialsProvider,
    Credentials,
};
use aws_sdk_s3::{
    config::Builder as S3ConfigBuilder,
    operation::create_bucket::CreateBucketError,
    presigning::PresigningConfig,
    Client,
};
use std::time::Duration;
use thiserror::Error;

// ─── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("S3 upload failed: {0}")]
    Upload(String),
    #[error("Presigned URL generation failed: {0}")]
    Presign(String),
    #[error("Bucket setup failed: {0}")]
    BucketSetup(String),
}

// ─── S3Storage ────────────────────────────────────────────────────────────────

/// S3-compatible object storage abstraction (works with MinIO and AWS S3).
#[derive(Clone)]
pub struct S3Storage {
    client: Client,
    pub bucket: String,
}

impl S3Storage {
    /// Construct a new S3Storage pointing at `endpoint` with the given bucket and credentials.
    pub async fn new(
        endpoint: &str,
        bucket: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Self {
        let credentials = Credentials::new(access_key, secret_key, None, None, "smiled-static");
        let provider = SharedCredentialsProvider::new(credentials);

        let sdk_config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new("us-east-1"))
            .credentials_provider(provider)
            .load()
            .await;

        let s3_config = S3ConfigBuilder::from(&sdk_config)
            .endpoint_url(endpoint)
            .force_path_style(true) // Required for MinIO
            .build();

        let client = Client::from_conf(s3_config);

        Self {
            client,
            bucket: bucket.to_string(),
        }
    }

    /// Upload `body` bytes with `content_type` at `key`. Returns the storage key.
    pub async fn upload(
        &self,
        key: &str,
        body: Vec<u8>,
        content_type: &str,
    ) -> Result<String, StorageError> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body.into())
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| StorageError::Upload(e.to_string()))?;

        Ok(key.to_string())
    }

    /// Generate a presigned GET URL valid for `expires_secs` seconds.
    pub async fn presigned_url(
        &self,
        key: &str,
        expires_secs: u64,
    ) -> Result<String, StorageError> {
        let presign_config = PresigningConfig::expires_in(Duration::from_secs(expires_secs))
            .map_err(|e| StorageError::Presign(e.to_string()))?;

        let presigned = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presign_config)
            .await
            .map_err(|e| StorageError::Presign(e.to_string()))?;

        Ok(presigned.uri().to_string())
    }

    /// Create the bucket if it does not already exist.
    pub async fn ensure_bucket(&self) -> Result<(), StorageError> {
        let result = self
            .client
            .create_bucket()
            .bucket(&self.bucket)
            .send()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(sdk_err) => {
                // BucketAlreadyOwnedByYou is not an error — bucket already exists
                if let Some(service_err) = sdk_err.as_service_error() {
                    if matches!(
                        service_err,
                        CreateBucketError::BucketAlreadyOwnedByYou(_)
                            | CreateBucketError::BucketAlreadyExists(_)
                    ) {
                        return Ok(());
                    }
                }
                Err(StorageError::BucketSetup(sdk_err.to_string()))
            }
        }
    }
}

// ─── Storage key helper ───────────────────────────────────────────────────────

/// Build the S3 object key for a document:
/// `{cabinet_id}/{patient_id}/{uuid}_{filename}`.
pub fn build_storage_key(
    cabinet_id: uuid::Uuid,
    patient_id: uuid::Uuid,
    file_uuid: uuid::Uuid,
    filename: &str,
) -> String {
    format!("{cabinet_id}/{patient_id}/{file_uuid}_{filename}")
}
