use crate::protos::fundraising;

use protobuf::Message;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use super::error::FromProtoError;

#[derive(Serialize, Deserialize, Debug)]
pub struct FundraisingCardSummary {
    pub link: String,
    pub title: String,
    pub description: String,
    pub verified: bool,
    pub contributors: u32,
    pub date: chrono::DateTime<Utc>,
}

impl FundraisingCardSummary {
    pub fn new(
        link: &str,
        title: &str,
        description: &str,
        verified: bool,
        contributors: u32,
    ) -> FundraisingCardSummary {
        FundraisingCardSummary {
            link: link.to_owned(),
            title: title.to_owned(),
            description: description.to_owned(),
            verified: verified,
            contributors: contributors,
            date: Utc::now(),
        }
    }

    pub fn from_proto(
        data: &Vec<u8>,
    ) -> Result<FundraisingCardSummary, FromProtoError> {
        let mut proto_summary = fundraising::FundraisingSummary::new();
        match proto_summary.merge_from_bytes(&data) {
            Err(e) => return Err(FromProtoError::ProtobufError(e)),
            Ok(_) => {}
        }
        let date_parsed = match DateTime::parse_from_rfc3339(proto_summary.get_date()) {
            Err(e) => return Err(FromProtoError::ParseError(e)),
            Ok(d) => d.with_timezone(&Utc),
        };
        let summary = FundraisingCardSummary{
            link: proto_summary.get_link().to_owned(),
            title: proto_summary.get_title().to_owned(),
            description: proto_summary.get_description().to_owned(),
            verified: proto_summary.get_verified(),
            contributors: proto_summary.get_contributors().to_owned(),
            date: date_parsed,
        };
        return Ok(summary);
    }

    pub fn to_proto(&self) -> Result<Vec<u8>, protobuf::error::ProtobufError> {
        let mut summary = fundraising::FundraisingSummary::new();
        summary.set_link(self.link.clone());
        summary.set_title(self.title.clone());
        summary.set_description(self.description.clone());
        summary.set_verified(self.verified);
        summary.set_contributors(self.contributors);
        summary.set_date(self.date.to_rfc3339());

        let mut out: Vec<u8> = Vec::new();
        match summary.write_to_vec(&mut out) {
            Err(e) => return Err(e),
            Ok(_) => return Ok(out),
        }
    }
}
