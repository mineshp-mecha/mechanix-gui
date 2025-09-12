use crate::SearchConfig;
use anyhow::Result;
use app_actions::service::AppActions;
use apps::{AppInfo, AppSearchService, RecentAppMetadata};
use files::FileInfo;
use log::{debug, error, info, warn};
use std::sync::Arc;
use zbus::{SignalContext, dbus_interface, fdo::Error as ZbusError};

/// The D-Bus path where the ConfigServer interface is served
pub const SERVED_AT: &str = "/org/mechanix/MxSearch";

/// ConfigServerInterface struct for D-Bus interface.
///
/// This struct implements the D-Bus interface for the configuration server.
/// It provides methods for listing schemas, listing keys, describing keys,
/// getting settings, and setting settings. It also emits signals when
/// settings are changed.
///
/// The interface is served at the path defined by the SERVED_AT constant.
#[derive()]
pub struct ServerInterface {
    pub(crate) config: SearchConfig,
    pub app_search_service: Option<AppSearchService>,
    pub file_search_service: Option<files::FileSearchService>,
    pub app_actions_service: Option<app_actions::AppActionsService>,
}

#[dbus_interface(name = "org.mechanix.MxSearch")]
impl ServerInterface {
    /// Signal emitted when a setting is changed.
    ///
    /// This signal is emitted whenever a setting is changed through the set_setting method.
    /// Clients can listen for this signal to be notified of changes to settings they are
    /// interested in.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the setting that was changed
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the signal was emitted successfully
    /// * `Err(...)` if there was an error during emission

    pub async fn search_applications(&self, search: &str) -> zbus::fdo::Result<Vec<AppInfo>> {
        info!("Search Apps: {}", search);
        if !self.config.apps.enable_search {
            warn!("Search Apps is disabled");
            return Err(ZbusError::Failed("Search Apps is disabled".to_string()));
        }
        // At some point later: perform a search
        if let Some(app_search_service) = &self.app_search_service {
            let result = match app_search_service.search(search, self.config.apps.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching apps: {}", err);
                    return Err(ZbusError::Failed("Error searching apps".to_string()));
                }
            };
            debug!("app search result: {:?}", result);
            Ok(result)
        } else {
            Err(ZbusError::Failed("Search Apps is disabled".to_string()))
        }
    }

    /// Lists available applications.
    ///
    /// This function queries the application search service to retrieve a list of applications.
    /// It checks if the search functionality is enabled before proceeding.
    ///
    /// # Errors
    ///
    /// Returns a `ZbusError::Failed` if the search functionality is disabled or if there is
    /// an error during the retrieval of applications.
    ///
    /// # Returns
    ///
    /// A vector of `AppInfo` representing the available applications if successful.
    pub async fn list_applications(&self) -> zbus::fdo::Result<Vec<AppInfo>> {
        info!("List applications");
        if !self.config.apps.enable_search {
            warn!("Search Apps is disabled");
            return Err(ZbusError::Failed("Search Apps is disabled".to_string()));
        }
        if let Some(app_search_service) = &self.app_search_service {
            let result = match app_search_service.list_applications(self.config.apps.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error listing apps: {}", err);
                    return Err(ZbusError::Failed("Error listing apps".to_string()));
                }
            };
            debug!("result: {:?}", result);
            Ok(result)
        } else {
            Err(ZbusError::Failed("Search Apps is disabled".to_string()))
        }
    }

    /// Searches for files matching the given search string.
    ///
    /// This function queries the file search service to retrieve a list of files matching the search string.
    /// It checks if the search functionality is enabled before proceeding.
    ///
    /// # Errors
    ///
    /// Returns a `ZbusError::Failed` if the search functionality is disabled or if there is
    /// an error during the retrieval of files.
    ///
    /// # Returns
    ///
    /// A vector of `FileInfo` representing the matching files if successful.
    pub async fn search_files(&self, search: &str) -> zbus::fdo::Result<Vec<FileInfo>> {
        info!("Search files: {}", search);
        if !self.config.files.enable_search {
            warn!("Search Files option is disabled");
            return Err(ZbusError::Failed(
                "Search Files option is disabled".to_string(),
            ));
        }

        if let Some(file_search_service) = &self.file_search_service {
            // At some point later: perform a search
            let results = match file_search_service.search(search, self.config.files.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching files: {}", err);
                    return Err(ZbusError::Failed("Error searching files".to_string()));
                }
            };
            debug!("file search result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed("Search Files is disabled".to_string()))
        }
    }

    /// Searches for app actions matching the given search string.
    ///
    /// This function queries the app actions service to retrieve a list of app actions
    /// matching the search string. It checks if the search functionality is enabled before proceeding.
    ///
    /// # Arguments
    ///
    /// * `search` - A search string to query app actions.
    ///
    /// # Errors
    ///
    /// Returns a `ZbusError::Failed` if the search functionality is disabled or if there is
    /// an error during the retrieval of app actions.
    ///
    /// # Returns
    ///
    /// A vector of `AppActions` representing the matching app actions if successful.
    pub async fn search_app_actions(&self, search: &str) -> zbus::fdo::Result<Vec<AppActions>> {
        info!("Search app actions: {}", search);
        if !self.config.app_actions.enable_search {
            warn!("Search App Action is disabled");
            return Err(ZbusError::Failed(
                "Search App Action is disabled".to_string(),
            ));
        }
        if let Some(app_action_service) = &self.app_actions_service {
            // At some point later: perform a search
            let results = match app_action_service.search(search, self.config.apps.search_limit) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching app actions: {}", err);
                    return Err(ZbusError::Failed("Error searching app actions".to_string()));
                }
            };
            debug!("app_actions search result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search App Action is disabled".to_string(),
            ))
        }
    }

    // Dbus method call return value:
    // ({'name': <'Application'>, 'icon': <'hello.png'>, 'exec': <'test'>, 'path': <'/hello/worlds'>, 'last_accessed': <'1898-04-09T00:00:00+00:00'>},)
    pub async fn register_recent_apps(
        &mut self,
        recent_app: RecentAppMetadata,
    ) -> zbus::fdo::Result<String> {
        info!("register_recent_app: {:?}", recent_app);
        if !self.config.apps.enable_search {
            warn!("Search App Action is disabled");
            return Err(ZbusError::Failed("Search App is disabled".to_string()));
        }
        if let Some(app_search_service) = &mut self.app_search_service {
            // At some point later: perform a search
            let results = match app_search_service.register_recent_app(recent_app) {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching app actions: {}", err);
                    return Err(ZbusError::Failed("Error searching app actions".to_string()));
                }
            };
            debug!("app_actions search result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search App Action is disabled".to_string(),
            ))
        }
    }
    pub async fn list_recent_apps(&self) -> zbus::fdo::Result<Vec<RecentAppMetadata>> {
        info!("list recent apps");
        if !self.config.apps.enable_search {
            warn!("Search App Action is disabled");
            return Err(ZbusError::Failed("Search App is disabled".to_string()));
        }
        if let Some(app_search_service) = &self.app_search_service {
            // At some point later: perform a search
            let results = match app_search_service.list_recent_apps() {
                Ok(results) => results,
                Err(err) => {
                    error!("Error searching app actions: {}", err);
                    return Err(ZbusError::Failed("Error searching app actions".to_string()));
                }
            };
            debug!("app_actions search result: {:?}", results);
            Ok(results)
        } else {
            Err(ZbusError::Failed(
                "Search App Action is disabled".to_string(),
            ))
        }
    }
}
