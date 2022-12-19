//! All the classes provided by Roblox

use uuid::Uuid;
use crate::model::Attributes;
use rbxm_proc::PropertyConvert;

pub use crate::__gen::classes::*;

#[doc = doc_link!("class/Instance")]
#[derive(Debug, Clone, PropertyConvert)]
#[non_exhaustive]
pub struct Base {
    /// The name of this instance
    pub name: String,
    /// Custom tags applied to the instance
    // FIXME(CraftSpider) This is most likely actually a list of some kind
    pub tags: String,
    /// The ID of the asset source for this instance
    pub source_asset_id: i64,
    /// Serialized custom attributes on the instance
    #[propname = "AttributesSerialize"]
    pub attributes: Attributes,
    /// A UUID identifying this instance in a world. Generally not present in model files
    pub unique_id: Option<Uuid>,
}
