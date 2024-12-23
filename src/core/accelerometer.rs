use std::time::Duration;

use dbus::{
    blocking::{BlockingSender, Connection},
    Error, Message,
};

use super::transform::Transform;

const DBUS_NAME: &str = "net.hadess.SensorProxy";
const DBUS_PATH: &str = "/net/hadess/SensorProxy";
const DBUS_PROPERTIES_INTERFACE: &str = "org.freedesktop.DBus.Properties";
const DBUS_SENSORS_INTERFACE: &str = "net.hadess.SensorProxy";

pub struct Accelerometer {
    connection: Connection,
}

impl Accelerometer {
    pub fn connect() -> Result<Self, Error> {
        let connection = Connection::new_system()?;
        Ok(Self { connection })
    }

    /// Claim Accelerometer.
    ///
    /// # See <https://hadess.pages.freedesktop.org/iio-sensor-proxy/gdbus-net.hadess.SensorProxy.html#gdbus-method-net-hadess-SensorProxy.ClaimAccelerometer>
    /// "To start receiving accelerometer reading updates from the proxy, the application must call the `ClaimAccelerometer` method.
    /// It can do so whether an accelerometer is available or not, updates would then be sent when an accelerometer appears."
    ///
    /// # Errors
    ///
    /// This function will return an error if unable to send and receive dbus message
    pub fn claim(self) -> Result<Self, Error> {
        let message = Message::new_method_call(
            DBUS_NAME,
            DBUS_PATH,
            DBUS_SENSORS_INTERFACE,
            // NOTE: When this process exits(/crashes), the iio-sensors-proxy will free up any
            // resources, so it's not neccessary to call ReleaseAccelerometer, as this process should never exit
            "ClaimAccelerometer",
        )
        .map_err(|msg| Error::new_custom("InvalidMessage", &msg))?;
        let _ = self
            .connection
            .send_with_reply_and_block(message, Duration::from_secs(10))?
            .as_result()?;
        Ok(self)
    }

    pub fn get_transform(&self) -> Result<Transform, Error> {
        let mut reply =
            Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_PROPERTIES_INTERFACE, "Get")
                .map(|msg| msg.append2(DBUS_SENSORS_INTERFACE, "AccelerometerOrientation"))
                .map_err(|msg| Error::new_custom("InvalidMessage", &msg))
                .and_then(|msg| {
                    self.connection
                        .send_with_reply_and_block(msg, Duration::from_secs(10))
                })?;

        let checked_reply = reply.as_result()?;
        checked_reply
            .get1::<dbus::arg::Variant<&str>>()
            .map(|v| v.0)
            .map(Transform::from_orientation)
            .ok_or_else(|| Error::new_custom("InvalidResponse", "Unable to parse response"))
    }

    pub fn poll_orientation_changed(&self, duration: Duration) -> Result<bool, Error> {
        self.connection.process(duration)
    }
}
