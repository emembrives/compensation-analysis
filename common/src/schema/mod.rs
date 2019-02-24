mod eval;
mod error;
mod details;
mod summary;

pub use self::details::{FundraisingDetails, Label, LabelType};
pub use self::summary::FundraisingCardSummary;
pub use self::eval::{FundraisingEvals, Eval};
pub use self::error::FromProtoError;