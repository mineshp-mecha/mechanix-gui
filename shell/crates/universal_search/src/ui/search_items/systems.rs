use bevy::prelude::*;

use crate::types::{DesktopApp, SearchResult, SearchResultType};
use crate::ui::search_items::components::SearchItemsComponent;
use crate::ui::{search_result_item, BrowserApps, SearchResults, SearchText};

pub fn update_search_results_mod(
    mut commands: Commands,
    q_search_results: Single<Entity, With<SearchItemsComponent>>,
    results: Res<SearchResults>,
    results_for: Res<SearchText>,
    browser_apps: Res<BrowserApps>,
    asset_server: Res<AssetServer>,
) {
    let arrow_up_right: Handle<Image> = asset_server.load("icons/arrow_up_right.png");
    let entity = q_search_results.into_inner();
    commands.entity(entity).despawn_related::<Children>();
    commands.entity(entity).with_children(|parent| {
        for result in results.0.clone() {
            parent.spawn(search_result_item(result, arrow_up_right.clone()));
        }
        browser_apps.0.iter().for_each(|app| {
            parent.spawn(search_result_item(
                SearchResult {
                    name: format!("Search {} on {}", results_for.0.clone(), app.name.clone()),
                    icon: app.icon.clone(),
                    on_click: None,
                    _type: SearchResultType::App,
                },
                arrow_up_right.clone(),
            ));
        });
    });
}
