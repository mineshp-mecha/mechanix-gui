use zbus::{fdo::Error as ZbusError, interface, zvariant::Type, SignalContext};

use mechanix_hw_buttons::{HwButton, Key, KeyEvent};

#[derive(Clone, Copy)]
pub struct HwButtonInterface {}

#[interface(name = "org.mechanix.services.HwButton")]
impl HwButtonInterface {
    #[zbus(signal)]
    async fn notification(
        &self,
        ctxt: &SignalContext<'_>,
        event: KeyEvent,
    ) -> Result<(), zbus::Error>;
}

pub async fn hw_buttons_notification_stream(
    hw_button_bus: &HwButtonInterface,
    conn: &zbus::Connection,
    power_button_path: String,
    home_button_path: String,
    volume_up_button_path: String,
    volume_down_button_path: String,
) -> Result<(), ZbusError> {
    let mut power_button = HwButton::new(power_button_path);
    let mut home_button = HwButton::new(home_button_path);
    let mut volume_up_button = HwButton::new(volume_up_button_path);
    let mut volume_down_button = HwButton::new(volume_down_button_path);
    println!("hw buttons notification stream started");
    loop {
        tokio::select! {
            (key, event) = power_button.poll() => {
                if key == Key::Power {
                println!("power button event is {:?}", event);
                let ctxt = SignalContext::new(conn, "/org/mechanix/services/HwButton/Power")?;
                hw_button_bus
                    .notification(
                        &ctxt,
                        event,
                    )
                    .await?;
                }

            }

            (key, event) = home_button.poll() => {
                if key == Key::Home {
                println!("home button event is {:?}", event);
                let ctxt = SignalContext::new(conn, "/org/mechanix/services/HwButton/Home")?;
                hw_button_bus
                    .notification(
                        &ctxt,
                        event,
                    )
                    .await?;
                }

            }
            (key, event) = volume_up_button.poll() => {
                if key == Key::VolumeUp {
                println!("volume up button event is {:?}", event);
                let ctxt = SignalContext::new(conn, "/org/mechanix/services/HwButton/VolumeUp")?;
                hw_button_bus
                    .notification(
                        &ctxt,
                        event,
                    )
                    .await?;
                }
            }
            (key, event) = volume_down_button.poll() => {
                if key == Key::VolumeDown {
                println!("volume down button event is {:?}", event);
                let ctxt = SignalContext::new(conn, "/org/mechanix/services/HwButton/VolumeDown")?;
                hw_button_bus
                    .notification(
                        &ctxt,
                        event,
                    )
                    .await?;
                }
            }
        }
    }
}
