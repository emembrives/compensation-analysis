use proto_capnp::fundraising_summary;

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

    pub fn to_proto() -> proto_capnp::FundraisingCardSummary {}
}