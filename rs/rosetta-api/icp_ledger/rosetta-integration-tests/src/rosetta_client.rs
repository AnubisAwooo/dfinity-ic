use ic_rosetta_api::models::{
    Block, BlockRequest, BlockResponse, MetadataRequest, NetworkIdentifier, NetworkListResponse,
    PartialBlockIdentifier,
};
use url::Url;

pub struct RosettaClient {
    pub url: Url,
}

impl RosettaClient {
    pub async fn network_list(&self) -> Result<Vec<NetworkIdentifier>, reqwest::Error> {
        let request = MetadataRequest::new();
        let client = reqwest::Client::new();
        let response = client
            .post(self.url.join("/network/list").unwrap())
            .json(&request)
            .send()
            .await?
            .json::<NetworkListResponse>()
            .await?;
        Ok(response.network_identifiers)
    }

    pub async fn block(
        &self,
        network_identifier: NetworkIdentifier,
        index: u64,
    ) -> Result<Option<Block>, reqwest::Error> {
        let block_identifier = PartialBlockIdentifier {
            index: Some(index as i64),
            hash: None,
        };
        let request = BlockRequest {
            network_identifier,
            block_identifier,
        };
        let client = reqwest::Client::new();
        let response = client
            .post(self.url.join("/block").unwrap())
            .json(&request)
            .send()
            .await?
            .json::<BlockResponse>()
            .await?;
        Ok(response.block)
    }
}
