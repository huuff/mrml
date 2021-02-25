use super::RawElement;
use crate::elements::body::node::Node;
use crate::elements::body::prelude::{
    empty_children_iterator, BodyComponent, BodyComponentChildIterator,
};
use crate::elements::{Component, Error};
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;

impl RawElement {
    pub fn as_node(&self) -> Option<&Node> {
        match self {
            RawElement::Node(node) => Some(node),
            _ => None,
        }
    }
}

impl Component for RawElement {
    fn context(&self) -> Option<&Context> {
        self.as_node().and_then(|node| node.context())
    }

    fn set_context(&mut self, ctx: Context) {
        if let RawElement::Node(node) = self {
            node.set_context(ctx);
        }
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        match self {
            RawElement::Comment(value) => value.render(header),
            RawElement::Node(node) => node.render(header),
            RawElement::Text(value) => value.render(header),
        }
    }
}

impl BodyComponent for RawElement {
    fn is_raw(&self) -> bool {
        true
    }

    fn get_children_len(&self) -> usize {
        self.as_node()
            .map(|node| node.get_children_len())
            .unwrap_or_default()
    }

    fn get_children<'p>(&'p self) -> BodyComponentChildIterator<'p> {
        self.as_node()
            .map(|node| node.get_children())
            .unwrap_or_else(empty_children_iterator)
    }

    fn get_current_width(&self) -> Option<Size> {
        self.as_node().and_then(|node| node.get_current_width())
    }

    fn attributes(&self) -> Option<&Attributes> {
        self.as_node().and_then(|node| node.attributes())
    }
}
