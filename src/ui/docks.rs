use std::fmt::Debug;

use super::{geometry::FlowDirection, utils::RefStr};

#[derive(Debug)]
pub enum UIFragment {
    Container(Box<dyn UIFragmentContainer>),
    Leaf(Box<dyn UIFragmentLeaf>),
}

pub trait UIFragmentLeaf: Debug {}

pub trait UIFragmentContainer: Debug {}

#[derive(Debug)]
pub struct Workspace<'a> {
    pub name: RefStr,
    pub root_node: &'a UIFragment,
}

////

#[derive(Debug)]
pub struct UIFragmentList {
    pub direction: FlowDirection,
    pub list: Vec<UIFragment>,
}

impl UIFragmentContainer for UIFragmentList {}
