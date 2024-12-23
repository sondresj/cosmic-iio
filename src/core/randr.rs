use cosmic_randr::{context::HeadConfiguration, Context, Message};
use tachyonix::Receiver;
use wayland_client::{
    backend::WaylandError, protocol::wl_output::Transform, Connection, DispatchError, EventQueue,
};

pub struct CosmicRandrClient {
    receiver: Receiver<Message>,
    context: Context,
    queue: EventQueue<Context>,
}

impl CosmicRandrClient {
    pub fn connect() -> Result<Self, cosmic_randr::Error> {
        let (sender, receiver) = tachyonix::channel(5);
        let (context, queue) = cosmic_randr::connect(sender)?;

        Ok(Self {
            receiver,
            context,
            queue,
        })
    }

    fn dispatch_until_manager_done(&mut self) -> Result<(), cosmic_randr::Error> {
        'outer: loop {
            while let Ok(msg) = self.receiver.try_recv() {
                if matches!(msg, Message::ManagerDone) {
                    break 'outer;
                }
            }

            dispatch(
                &self.context.connection.clone(),
                &mut self.queue,
                &mut self.context,
            )?;
        }
        Ok(())
    }

    fn receive_config_messages(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            while let Ok(message) = self.receiver.try_recv() {
                match message {
                    Message::ConfigurationSucceeded => return Ok(()),
                    Message::ConfigurationCancelled => return Err("Configuration cancelled".into()),
                    Message::ConfigurationFailed => return Err("Configuration failed".into()),
                    _ => {}
                }
            }
            dispatch(
                &self.context.connection.clone(),
                &mut self.queue,
                &mut self.context,
            )?;
        }
    }

    // pub fn get_output(&mut self, id: &str) -> Option<&cosmic_randr::output_head::OutputHead> {
    //     if self.dispatch_until_manager_done().is_ok() {
    //         self.context
    //             .output_heads
    //             .values()
    //             .filter(|h| h.name.as_str() == id)
    //             .nth(0)
    //     } else {
    //         None
    //     }
    // }

    pub fn apply_transform(
        &mut self,
        output: &str,
        transform: Transform,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.dispatch_until_manager_done()?;

        let mut config = self.context.create_output_config();
        let head_config = HeadConfiguration {
            transform: Some(transform),
            ..Default::default()
        };
        config.enable_head(output, Some(head_config))?;
        config.apply();

        self.receive_config_messages()?;

        Ok(())
    }
}

fn dispatch<Data>(
    connection: &Connection,
    event_queue: &mut EventQueue<Data>,
    data: &mut Data,
) -> Result<usize, DispatchError> {
    let dispatched = event_queue.dispatch_pending(data)?;

    if dispatched > 0 {
        return Ok(dispatched);
    }

    connection.flush()?;

    if let Some(guard) = connection.prepare_read() {
        if let Err(why) = guard.read() {
            if let WaylandError::Io(ref error) = why {
                if error.kind() != std::io::ErrorKind::WouldBlock {
                    return Err(DispatchError::Backend(why));
                }
            } else {
                return Err(DispatchError::Backend(why));
            }
        }
    }

    event_queue.dispatch_pending(data)
}
