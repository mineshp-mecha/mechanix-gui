use anyhow::Result;
use command::spawn_command;
use futures_util::stream::StreamExt;
use logind::session_lock;
use mechanix_system_dbus_client::hardware_buttons::{HwButton, Key, KeyEvent};
use std::time::Instant;
use tracing::debug;
use crate::settings::lock_button::LockButtonSettings;

pub struct VolButtonHandler {
    pressed_at: Option<Instant>,
    configs: LockButtonSettings,
}
impl VolButtonHandler {
    pub fn new(configs: LockButtonSettings) -> Self {
        Self {
            pressed_at: None,
            configs,
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

        loop {
            tokio::select! {
                maybe_volume_up_signal = volume_up_stream.next() => {
                    if let Some(signal) = maybe_volume_up_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            debug!("volume up button event is {:?}", event);
                            // Handle volume up events here
                        }
                    }
                }
                maybe_volume_down_signal = volume_down_stream.next() => {
                    if let Some(signal) = maybe_volume_down_signal {
                        if let Ok(args) = signal.args() {
                            let event = args.event;
                            debug!("volume down button event is {:?}", event);
                            // Handle volume down events here
                        }
                    }
                }
            }
        }
    }
}
