use super::proto_capnp::fundraising_summary;
use capnp::serialize;

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
        let mut summary = message.init_root::<fundraising_summary::Builder>();
        summary.set_link(&self.link);
        summary.set_title(&self.title);
        summary.set_description(&self.description);
        summary.set_verified(self.verified);
        summary.set_contributors(self.contributors);

        let mut out: Vec<u8> = Vec::new();
        match serialize::write_message(&mut out, &message) {
            Err(e) => return Err(e),
            Ok(_) => return Ok(out),
        }
    }
}

pub struct FundraisingDetail {
    pub link: String,
    pub title: String,
    pub description: String,
    pub verified: bool,
    pub contributors: u32,
    pub collected: u32,
    pub objective: u32,
    pub fundraiser: String,
    pub tags: Vec<String>
}
