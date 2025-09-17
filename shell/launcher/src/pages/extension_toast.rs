use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction}
    , node, rect, size_pct,
    style::Styled,
    txt,
    widgets::{Div, Text},
    Color,
};
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct ExtensionToast {
    pub label: String,
}

impl Component for ExtensionToast {
    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.label.hash(hasher);
    }

    fn view(&self) -> Option<mctk_core::Node> {
        if self.label.is_empty() {
            return None;
        }

        // Build the toast card first
        // The toast card with sharper corners and no close button
        let toast = node!(
            Div::new()
                .bg(Color::rgba(20., 20., 20., 0.94))
                .style("border_radius", 4.) // Sharper corners
                .style("stroke_color", Color::rgba(255., 255., 255., 0.12))
                .style("stroke_width", 1.),
            lay![
                margin: [0., 16., 24., 16.],
                padding: [12., 16., 12., 16.],
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center,
                direction: Direction::Row,
            ]
        )
        .push(node!(
            Text::new(txt!(format!("{}", self.label)))
                .with_class("text-base font-space-grotesk font-medium")
                .style("color", Color::rgb(235., 235., 235.)),
            lay![
                size_pct: [100, Auto],
            ]
        ));

        let node = node!(
            Div::new().bg(Color::rgba(0., 0., 0., 0.0)),
            lay![
                size_pct: [100],
                position_type: Absolute,
                position: [0., 0., 0., 0.],
                axis_alignment: Alignment::End,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(toast);

        Some(node)
    }
}
