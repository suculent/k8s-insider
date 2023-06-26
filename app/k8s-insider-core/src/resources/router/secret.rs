use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::Secret, ByteString};

use super::RouterRelease;

pub const SERVER_PRIVATE_KEY_SECRET: &str = "SERVER_PRIVATE_KEY";

impl RouterRelease {
    pub fn generate_secret(&self) -> Secret {
        let secret_data = BTreeMap::from([(
            SERVER_PRIVATE_KEY_SECRET.to_owned(),
            ByteString(self.server_private_key.as_bytes().to_vec()),
        )]);

        Secret {
            metadata: self.generate_router_metadata(),
            data: Some(secret_data),
            ..Default::default()
        }
    }
}
