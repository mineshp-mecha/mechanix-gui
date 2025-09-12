use bevy::ecs::error::info;
use bevy::{
    asset::{AssetMetaCheck, AssetPath},
    ecs::{query, system::SystemId},
    prelude::*,
};
use freedesktop_icons::lookup;
use std::path::Path;

mod components;
mod icons;
mod mock;
mod resources;
mod states;
mod systems;
mod types;
mod ui;

use service_plugins::{mxsearch, MxSearchAction, MxSearchActionEvent, MxSearchPlugin};
use systems::*;
use types::*;
use utils::prelude::{fonts_loaded, FontAssetsPlugin};

use crate::ui::{search_result_item, update_search_results_mod};
use crate::{
    icons::{icons_loaded, UniversalSearchIconsPlugin},
    resources::IsOpen,
    states::{listen_action, Action},
    ui::{
        BrowserApps, FrequentlyUsedApps, SearchInputPlugin, SearchItems, SearchResults, SearchText,
    },
};
use headless_widgets::prelude::*;

#[derive(Event)]
pub struct UniversalSearchOpen;

#[derive(Event)]
pub struct UniversalSearchClose;

pub struct UniversalSearchPlugin;
impl Plugin for UniversalSearchPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FrequentlyUsedApps(vec![]));
        app.insert_resource(SearchItems(vec![]));
        app.insert_resource(SearchResults(vec![]));
        app.insert_resource(SearchText("".to_string()));
        app.insert_resource(BrowserApps(vec![]));

        app.add_plugins(FontAssetsPlugin);
        app.add_plugins(UniversalSearchIconsPlugin);
        app.add_plugins((animation::DefaultTweenPlugins,));
        // app.add_plugins((mock::MockPlugin,));
        app.add_plugins((headless_widgets::CoreWidgetsPlugin));
        app.add_plugins(SearchInputPlugin);
        app.add_plugins(MxSearchPlugin);

        app.add_event::<UniversalSearchOpen>();
        app.add_event::<UniversalSearchClose>();
        app.add_systems(Startup, camera_setup);
        app.add_systems(
            Update,
            setup
                .run_if(resource_exists::<UniversalSearchWindowCamera>)
                .run_if(fonts_loaded)
                .run_if(icons_loaded),
        );

        app.add_observer(listen_open_event);
        app.add_observer(listen_close_event);
        app.add_systems(Update, listen_close_completed);
        app.add_systems(Update, (button_system, exit_on_esc));
        app.add_systems(
            Update,
            query_search_results.run_if(resource_changed::<SearchText>),
        );

        app.add_systems(
            Update,
            update_search_results
                // .run_if(resource_changed::<mxsearch::AppSearchResult>)
                .run_if(resource_changed::<mxsearch::FileSearchResult>),
        );
        app.add_systems(
            Update,
            update_search_results_mod.run_if(resource_changed::<SearchResults>),
        );

        // app.insert_resource(IsOpen(false));
        // app.add_systems(Update, (button_system, effect_system, exit_on_esc));
        // app.add_event::<Action>();

        app.add_observer(on_bar_drag_start);
        app.add_observer(on_bar_drag);
        app.add_observer(on_bar_drag_end);

        // app.add_systems(Update, listen_action);
    }
}
fn update_search_results(
    mx_search_app_result: Res<mxsearch::AppSearchResult>,
    mx_search_file_result: Res<mxsearch::FileSearchResult>,
    mut search_result: ResMut<SearchResults>,
    asset_server: Res<AssetServer>,
) {
    let apps = mx_search_app_result.0.clone();
    let files = mx_search_file_result.0.clone();
    // Avoiding unnessesary resource update when results and existing results are empty
    if (apps.len() == 0 && files.len() == 0 && search_result.0.len() == 0) {
        return;
    }
    let mut final_results: Vec<SearchResult> = Vec::new();
    if apps.len() > 0 {
        for app in apps {
            let result = SearchResult {
                name: app.name,
                icon: lookup_icon(&app.icon, &SearchResultType::App, &asset_server),
                on_click: None,
                _type: SearchResultType::App,
            };
            if !final_results.iter().any(|r| r.name == result.name && r._type == result._type) {
                final_results.push(result);
            }
        }
        info!("apps:: final result iter: {:?} ", final_results.len());
    }

    if files.len() > 0 {
        for file in files {
            let result = SearchResult {
                name: file.name,
                icon: lookup_icon(&file.icon, &SearchResultType::File, &asset_server),
                on_click: None,
                _type: SearchResultType::File,
            };
            if !final_results.iter().any(|r| r.name == result.name && r._type == result._type) {
                final_results.push(result);
            }
        }
    }
    info!("final result before assign: {:?}",final_results);
    search_result.0 = final_results;
    info!("final result iter: {:?} ", search_result.0);
}
fn query_search_results(
    search_text: Res<SearchText>,
    mut search_result: ResMut<SearchResults>,
    mut mx_search_action_writer: EventWriter<MxSearchActionEvent>,
) {
    // On earch search first clear existing search results
    if search_text.0.is_empty() {
        search_result.0.clear();
    }
    mx_search_action_writer.write(MxSearchActionEvent(MxSearchAction::SearchApplications(
        search_text.0.clone(),
    )));
    mx_search_action_writer.write(MxSearchActionEvent(MxSearchAction::SearchFiles(
        search_text.0.clone(),
    )));
}
pub mod prelude {
    pub use crate::UniversalSearchPlugin;
    pub use crate::{UniversalSearchClose, UniversalSearchOpen};
}

fn lookup_icon(
    icon_name: &str,
    result_type: &SearchResultType,
    asset_server: &AssetServer,
) -> Handle<Image> {
    let icon = lookup(&icon_name)
        .with_size(84)
        .with_theme("Papirus")
        .find()
        .unwrap_or_default()
        .into_os_string()
        .into_string()
        .unwrap();
    let default_icon = match result_type {
        SearchResultType::App => Path::new("icons/kitty.png"),
        SearchResultType::File => Path::new("icons/files.png"),
        SearchResultType::Action => Path::new("icons/rotation_on.png"),
    };

    let path = Path::new(&icon);
    info!("icon path: {:?}", path);
    match path.extension() {
        Some(ext) if ext == "svg" => asset_server.load(AssetPath::from_path(default_icon)),
        Some(ext) if ext == "png" => asset_server.load(AssetPath::from_path(path)),
        _ => {
            println!("Unsupported icon format: {:?}", path.extension());
            asset_server.load(AssetPath::from_path(Path::new("icons/files.png")))
        }
    }
}
