/// The type of asset that is linked into a project
#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum AssetType {

    /// Contains the extention of the image
    Image(String),
}

/// A linked asset copied into a project
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Asset {
    _type: AssetType,
    name: String,
    blob: Vec<u8>
}


