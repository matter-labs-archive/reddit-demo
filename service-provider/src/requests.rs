use crate::database::Community;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclareCommunityRequest {
    #[serde(flatten)]
    pub community: Community,
}
