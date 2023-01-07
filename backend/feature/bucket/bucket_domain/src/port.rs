use common_domain::{define_port, error::Result};

use crate::model::{bucket_object_head::BucketObjectHead, presigned_url::PresignedUrl};

define_port!(GetUploadUrl = FnOnce<'a>(key: &'a str) -> Result<PresignedUrl>);

define_port!(GetBucketObjectInfo = Fn<'a>(key: &'a str) -> Result<BucketObjectHead>);

define_port!(DeleteBucketObject = Fn<'a>(key: &'a str) -> Result<()>);

define_port!(GetBucketObjectUrl = Fn<'a>(key: &'a str) -> Result<String>);
