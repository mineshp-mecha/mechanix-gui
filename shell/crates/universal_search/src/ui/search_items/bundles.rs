use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};

use crate::{
    icons::UniversalSearchIcons,
    systems::NORMAL_BUTTON,
    types::{SearchResult, SearchResultType},
    ui::SearchItemsComponent,
};
use types::prelude::*;

pub fn search_items(
    results: Vec<SearchResult>,
    font_assets: &FontAssets,
    icons: &UniversalSearchIcons,
) -> impl Bundle {
    let font_assets = font_assets.clone();
    let search_icon: Handle<Image> = icons.search.clone();
    let arrow_up_right_icon: Handle<Image> = icons.arrow_up_right.clone();

    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(230.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
        ScrollPosition {
            offset_x: 0.0,
            offset_y: 0.0,
        },
        SearchItemsComponent,
    )
}

fn search_item(
    result: &SearchResult,
    arrow_up_right_icon: &Handle<Image>,
    font_assets: &FontAssets,
    // on_click: SystemId
) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(28.),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.),
                    margin: UiRect::vertical(Val::Px(12.)),
                    ..Default::default()
                },
                // CoreButton {
                //     on_click: Some(on_click),
                //     on_long_press: None,
                // },
                children![
                    (
                        Node {
                            width: Val::Px(36.),
                            height: Val::Px(36.),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        Button,
                        BackgroundColor(NORMAL_BUTTON.into()),
                        BorderRadius::all(Val::Px(6.)),
                        children![(
                            ImageNode::new(result.icon.clone()),
                            Node {
                                width: Val::Px(24.),
                                height: Val::Px(24.),
                                ..Default::default()
                            },
                        )],
                    ),
                    (
                        Node {
                            flex_grow: 1.,
                            ..Default::default()
                        },
                        Text::new(result.name.clone()),
                        TextFont {
                            font_size: 16.,
                            font: font_assets.primary_500.clone(),
                            ..Default::default()
                        },
                        TextColor(Color::oklch(0.7572, 0., 0.)),
                    ),
                    (
                        ImageNode::new(arrow_up_right_icon.clone()),
                        Node {
                            width: Val::Px(18.),
                            height: Val::Px(18.),
                            ..Default::default()
                        },
                    ),
                ],
            ),
            separator()
        ],
    )
}

fn separator() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(2.),
            ..Default::default()
        },
        BackgroundColor(Color::oklch(0.2435, 0., 0.)),
    )
}
