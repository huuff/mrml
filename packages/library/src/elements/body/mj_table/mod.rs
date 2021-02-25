mod parser;
mod renderer;

use crate::elements::body::mj_body::children::MJBodyChild;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-table";

#[derive(Debug)]
pub struct MJTable {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJBodyChild>,
}
