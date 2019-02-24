use crate::protos::fundraising;

use super::error::FromProtoError;
use chrono::prelude::*;
use protobuf::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Eval {
    tags: Vec<String>,
    date: chrono::DateTime<Utc>,
    source: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FundraisingEvals {
    link: String,
    evals: Vec<Eval>,
}

impl FundraisingEvals {
    pub fn new(link: String) -> FundraisingEvals {
        FundraisingEvals {
            link: link,
            evals: Vec::new(),
        }
    }

    pub fn new_eval(&mut self, tags: Vec<String>, source: String) {
        self.evals.push(Eval{
            tags: tags,
            date: Utc::now(),
            source: source,
        });
    }

    pub fn to_proto(&self) -> Result<Vec<u8>, protobuf::error::ProtobufError> {
        let mut proto_evals = fundraising::FundraisingEval::new();
        proto_evals.set_link(self.link.clone());
        let eval_list = proto_evals.mut_eval();
        for eval in &self.evals {
            let mut eval_proto = fundraising::FundraisingEval_Eval::new();
            eval_proto.set_tags(protobuf::RepeatedField::from(eval.tags.clone()));
            eval_proto.set_source(eval.source.clone());
            eval_proto.set_date(eval.date.to_rfc3339());
            eval_list.push(eval_proto);
        }
        let mut out: Vec<u8> = Vec::new();
        match proto_evals.write_to_vec(&mut out) {
            Err(e) => return Err(e),
            Ok(_) => return Ok(out),
        }
    }

    pub fn from_proto(data: &Vec<u8>) -> Result<FundraisingEvals, FromProtoError> {
        let mut proto_evals = fundraising::FundraisingEval::new();
        match proto_evals.merge_from_bytes(&data) {
            Err(e) => return Err(FromProtoError::ProtobufError(e)),
            Ok(_) => {}
        }
        let mut eval_list = Vec::new();
        for proto_eval in proto_evals.get_eval() {
            let date_parsed = match DateTime::parse_from_rfc3339(proto_eval.get_date()) {
                Err(e) => return Err(FromProtoError::ParseError(e)),
                Ok(d) => d.with_timezone(&Utc),
            };
            let tags = proto_eval.get_tags();
            let source = proto_eval.get_source();
            let eval = Eval{
                tags: tags.to_vec(),
                date: date_parsed,
                source: source.to_owned(),
            };
            eval_list.push(eval);
        }
        let evals = FundraisingEvals{
            link: proto_evals.get_link().to_owned(),
            evals: eval_list,
        };
        return Ok(evals);

    }
}
