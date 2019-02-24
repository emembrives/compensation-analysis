#[derive(Debug)]
pub enum FromProtoError {
    ProtobufError(protobuf::error::ProtobufError),
    ParseError(chrono::format::ParseError)
}

