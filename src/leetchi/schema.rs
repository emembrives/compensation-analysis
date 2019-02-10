use super::fundraising_capnp::fundraising;
use capnp::serialize;

#[derive(Debug)]
pub struct FundraisingCardSummary {
    pub link: String,
    pub title: String,
    pub description: String,
    pub verified: bool,
    pub contributors: u32,
}

impl FundraisingCardSummary {
    pub fn new(link: &str, title: &str, description: &str, verified: bool, contributors: u32) -> FundraisingCardSummary {
        FundraisingCardSummary{
            link: link.to_owned(),
            title: title.to_owned(),
            description: description.to_owned(),
            verified: verified,
            contributors: contributors,
        }
    }

    pub fn to_proto(&self) -> std::io::Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut summary = message.init_root::<fundraising::Builder>();
        summary.set_link(&self.link);
        summary.set_title(&self.title);
        summary.set_short_description(&self.description);
        summary.set_verified(self.verified);
        summary.set_contributors(self.contributors);

        let mut out: Vec<u8> = Vec::new();
        match serialize::write_message(&mut out, &message) {
            Err(e) => return Err(e),
            Ok(_) => return Ok(out),
        }
    }
}

#[derive(Debug)]
pub enum LabelType {
    EventType,
    Location,
}

#[derive(Debug)]
pub struct Label {
    pub label_type: LabelType,
    pub name: String,
}

#[derive(Debug)]
pub struct FundraisingDetail {
    pub title: String,
    pub description: String,
    pub verified: bool,
    pub collected: Option<String>,
    pub contributors: u32,
    pub fundraiser: String,
    pub tags: Vec<Label>
}

impl FundraisingDetail {
    pub fn new(title: String, description: String, verified: bool,
                collected: Option<String>, contributors: u32, fundraiser: String) -> FundraisingDetail {
        FundraisingDetail{
            title: title,
            description: description,
            verified: verified,
            contributors: contributors,
            collected: collected,
            fundraiser: fundraiser,
            tags: Vec::new(),
        }
    }

    pub fn to_proto(&self) -> std::io::Result<Vec<u8>> {
        let mut message = ::capnp::message::Builder::new_default();
        let mut details = message.init_root::<fundraising_details::Builder>();
        details.set_title(&self.title);
        details.set_description(&self.description);
        details.set_verified(self.verified);
        match self.collected {
            None => {},
            Some(t) => details.set_collected(&t)
        }
        details.set_contributors(self.contributors);
        details.set_fundraiser(&self.fundraiser);
        details.init_tags(self.tags.len());
/*        for i in 0..self.tags.len() {
            let tag = &self.tags.get(i).unwrap();
            let mut tag_proto = details.get(i);
            tag_proto.set_name(&tag.name);
            match tag.label_type {
                EventType => tag_proto.set_label_type(fundraising_details::LabelType::eventType),
                Location => tag_proto.set_label_type(fundraising_details::LabelType::location),
            }
        }*/

        let mut out: Vec<u8> = Vec::new();
        match serialize::write_message(&mut out, &message) {
            Err(e) => return Err(e),
            Ok(_) => return Ok(out),
        }
    }
}