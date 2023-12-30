use wgpu::Color;
use wgpu_text::{
    glyph_brush::{
        ab_glyph::{FontArc, InvalidFont},
        Layout, OwnedSection, Section, Text,
    },
    BrushBuilder, TextBrush,
};

const FONT: &[u8; 647344] = include_bytes!("../../assets/fonts/SourceSans.ttf");

pub fn create_brush(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> Result<TextBrush, InvalidFont> {
    let fa = FontArc::try_from_slice(FONT)?;
    let b = BrushBuilder::using_font(fa).build(device, config.width, config.height, config.format);
    Ok(b)
}

pub fn get_example_test_section() -> OwnedSection {
    let scale = 64.0;
    Section::default()
        .add_text(Text::new("Hey there").with_scale(scale))
        .add_text(
            Text::new(" benichi")
                .with_scale(scale)
                .with_color([1.0, 1.0, 1.0, 1.0]),
        )
        .add_text(Text::new("!!!").with_scale(scale))
        .with_layout(Layout::default().v_align(wgpu_text::glyph_brush::VerticalAlign::Top))
        .with_screen_position((32.0, 32.0))
        .to_owned()
}
