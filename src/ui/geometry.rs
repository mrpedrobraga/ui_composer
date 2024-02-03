/// Direction of flow, taking into account locales.
#[derive(Debug, Clone, Copy)]
pub enum FlowDirection {
    Forward,
    Backward,
    Horizontal,
    HorizontalReverse,
    Vertical,
    VerticalReverse,
    LeftToRight,
    TopToBottom,
    RightToLeft,
    BottomToTop,
}

#[derive(Debug, Clone, Copy)]
pub enum UIVector {
    InFlow(VectorEmbedded),
    Cartesian(VectorCartesian),
}

/// Vector that aligns itself to the basis of the parent's flow direction.
#[derive(Debug, Clone, Copy)]
pub struct VectorEmbedded {
    flow_axis: i32,
    cross_axis: i32,
}

/// Vector that aligns itself with the basis of the screen.
#[derive(Debug, Clone, Copy)]
pub struct VectorCartesian {
    x: i32,
    y: i32,
}

/// Axis-aligned bounding box specified in UIVectors.
#[derive(Debug, Clone, Copy)]
pub struct UIAABB {
    position: UIVector,
    size: UIVector,
}
