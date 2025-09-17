use crate::AppMessage;
use anyhow::Result;
use futures::StreamExt;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mechanix_system_dbus_client::hardware_buttons::{HwButton, Key, KeyEvent};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Instant};
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::{sleep, Duration};
use tracing::{debug, info};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::{
    ToplevelHandler, ToplevelMessage,
};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct VolumeButtonSettings {
    pub min_time_long_press: u64,
}

pub struct VolumeButtonHandler {
    app_channel: Sender<AppMessage>,
    pressed_at: Arc<Mutex<Option<Instant>>>, // Use Arc<Mutex<>> for shared mutable state
    configs: VolumeButtonSettings,
    long_press_sent: Arc<Mutex<bool>>, // Track if long press action was sent
}
impl VolumeButtonHandler {
    pub fn new(app_channel: Sender<AppMessage>) -> Self {
        Self {
            pressed_at: Arc::new(Mutex::new(None)),
            configs: VolumeButtonSettings {
                min_time_long_press: 1,
            },
            app_channel,
            long_press_sent: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        let mut volume_up_stream = HwButton::get_notification_stream(
            "/org/mechanix/services/HwButton/VolumeUp".to_string(),
        )
            .await?;
        let mut volume_down_stream = HwButton::get_notification_stream(
            "/org/mechanix/services/HwButton/VolumeDown".to_string(),
        )
            .await?;

        let app_channel = self.app_channel.clone();
        loop {
            tokio::select! {
                maybe_volume_up_signal = volume_up_stream.next() => {
                    if let Some(signal) = maybe_volume_up_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            info!("volume up button event is {:?}", event);
                            let _ = app_channel.send(AppMessage::Extension {
                                                message: crate::ExtensionAppsMessage::Attached {
                                                    label: "Keyboard attached!".to_string(),
                                                },
                                            });
                            info!("the attached event is published");
                            // Handle volume up events here
                        }
                    }
                }
                maybe_volume_down_signal = volume_down_stream.next() => {
                    if let Some(signal) = maybe_volume_down_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            info!("volume down button event is {:?}", event);
                            // Handle volume down events here
                        }
                    }
                }
            }
        }
    }
}
