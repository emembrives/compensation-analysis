use super::protos::fundraising;

use protobuf::Message;
use chrono::prelude::*;

#[derive(Debug)]
pub enum FromProtoError {
    ProtobufError(protobuf::error::ProtobufError),
    ParseError(chrono::format::ParseError)
}

#[derive(Debug)]
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

#[derive(Debug)]
#[derive(PartialEq)]
pub enum LabelType {
    EventType,
    Location,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Label {
    pub label_type: LabelType,
    pub name: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct FundraisingDetails {
    pub link: String,
    pub title: String,
    pub description: String,
    pub verified: bool,
    pub collected: Option<String>,
    pub contributors: Option<u32>,
    pub fundraiser: String,
    pub delay: Option<u32>,
    pub tags: Vec<Label>,
    pub date: chrono::DateTime<Utc>,
}

impl FundraisingDetails {
    pub fn new(
        link: String,
        title: String,
        description: String,
        verified: bool,
        collected: Option<String>,
        contributors: Option<u32>,
        fundraiser: String,
        delay: Option<u32>,
    ) -> FundraisingDetails {
        FundraisingDetails {
            link: link,
            title: title,
            description: description,
            verified: verified,
            contributors: contributors,
            collected: collected,
            fundraiser: fundraiser,
            delay: delay,
            tags: Vec::new(),
            date: Utc::now(),
        }
    }

    pub fn to_proto(&self) -> Result<Vec<u8>, protobuf::error::ProtobufError> {
        let mut details = fundraising::FundraisingDetails::new();
        details.set_link(self.link.clone());
        details.set_title(self.title.clone());
        details.set_description(self.description.clone());
        details.set_verified(self.verified);
        match &self.collected {
            None => {}
            Some(t) => details.set_collected(t.clone()),
        }
        match &self.contributors {
            None => {}
            Some(t) => details.set_contributors(*t),
        }
        details.set_fundraiser(self.fundraiser.clone());
        match &self.delay {
            None => {}
            Some(t) => details.set_delay(*t),
        }
        let mut_tags = details.mut_tags();
        for tag in &self.tags {
            let mut proto_tag = fundraising::FundraisingDetails_Label::new();
            proto_tag.set_name(tag.name.clone());
            match &tag.label_type {
                LabelType::EventType => proto_tag
                    .set_label_type(fundraising::FundraisingDetails_Label_LabelType::EVENT_TYPE),
                LabelType::Location => proto_tag
                    .set_label_type(fundraising::FundraisingDetails_Label_LabelType::LOCATION),
            }
            mut_tags.push(proto_tag);
        }
        details.set_date(self.date.to_rfc3339());

        let mut out: Vec<u8> = Vec::new();
        match details.write_to_vec(&mut out) {
            Err(e) => return Err(e),
            Ok(_) => return Ok(out),
        }
    }

    pub fn from_proto(
        data: &Vec<u8>,
    ) -> Result<FundraisingDetails, FromProtoError> {
        let mut proto_detail = fundraising::FundraisingDetails::new();
        match proto_detail.merge_from_bytes(&data) {
            Err(e) => return Err(FromProtoError::ProtobufError(e)),
            Ok(_) => {}
        }
        let date_parsed = match DateTime::parse_from_rfc3339(proto_detail.get_date()) {
            Err(e) => return Err(FromProtoError::ParseError(e)),
            Ok(d) => d.with_timezone(&Utc),
        };
        let detail = FundraisingDetails{
            link: proto_detail.get_link().to_owned(),
            title: proto_detail.get_title().to_owned(),
            description: proto_detail.get_description().to_owned(),
            verified: proto_detail.get_verified(),
            contributors: match proto_detail.has_contributors() {
                true => Some(proto_detail.get_contributors()),
                false => None,
            },
            collected: match proto_detail.has_collected() {
                true => Some(proto_detail.get_collected().to_owned()),
                false => None,
            },
            fundraiser: proto_detail.get_fundraiser().to_owned(),
            delay: match proto_detail.has_delay() {
                true => Some(proto_detail.get_delay()),
                false => None,
            },
            tags: proto_detail.get_tags().into_iter().map(|t| Label{
                name: t.get_name().to_owned(),
                label_type: match t.get_label_type() {
                    fundraising::FundraisingDetails_Label_LabelType::EVENT_TYPE => LabelType::EventType,
                    fundraising::FundraisingDetails_Label_LabelType::LOCATION => LabelType::Location,
                }
            }).collect(),
            date: date_parsed,
        };
        return Ok(detail);
    }
}
