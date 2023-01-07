#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct BucketObjectHead {
    pub key: String,
    pub mime: String,
    pub size: u64,
}
