use std::collections::HashSet;

use anyhow::{anyhow, Result};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::{stream::types::StreamInformation, video::types::VideoSourceType};

//TODO: move to stream ?
#[derive(Apiv2Schema, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VideoAndStreamInformation {
    pub name: String,
    pub stream_information: StreamInformation,
    pub video_source: VideoSourceType,
}

impl VideoAndStreamInformation {
    pub fn conflicts_with(&self, other: &VideoAndStreamInformation) -> Result<()> {
        if self.name == other.name {
            return Err(anyhow!(
                "Stream ({other_name:#?} - {other_source:#?}) is already using the name {name:#?}.",
                other_name = other.name,
                other_source = other.video_source.inner().source_string(),
                name = self.name,
            ));
        }

        if (!self.video_source.inner().is_shareable())
            && (self.video_source.inner().source_string()
                == other.video_source.inner().source_string())
        {
            return Err(anyhow!(
                "Streams have same source: {:#?}",
                self.video_source.inner().source_string()
            ));
        }

        let our_endpoints: HashSet<_> = self.stream_information.endpoints.iter().collect();
        let other_endpoints: HashSet<_> = other.stream_information.endpoints.iter().collect();
        let common_endpoints: HashSet<_> = our_endpoints.intersection(&other_endpoints).collect();

        if !common_endpoints.is_empty() {
            return Err(anyhow!(
                "Stream ({other_name:#?} - {other_source:#?}) has common endpoint with Stream ({our_name:#?} - {our_source:#?}). The common endpoint: {endpoints:#?}",
                other_name = other.name,
                other_source = other.video_source.inner().source_string(),
                our_name = self.name,
                our_source = self.video_source.inner().source_string(),
                endpoints = common_endpoints
            ));
        }

        Ok(())
    }
}

//TODO: Add tests
