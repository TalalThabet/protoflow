// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt, Arc, Bytes, PhantomData},
    Message, MessageSender, OutputPortID, Port, PortID, PortResult, PortState, System, Transport,
};

#[derive(Clone)] //, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OutputPort<T: Message> {
    pub(crate) id: OutputPortID,
    pub(crate) transport: Arc<dyn Transport>,
    _phantom: PhantomData<T>,
}

impl<T: Message> OutputPort<T> {
    pub fn new<X: Transport + Default>(system: &System<X>) -> Self {
        let runtime = system.runtime.as_ref();
        let transport = runtime.transport.clone();
        Self {
            _phantom: PhantomData,
            id: transport.open_output().unwrap(),
            transport,
        }
    }

    pub fn close(&mut self) -> PortResult<bool> {
        self.transport.close(PortID::Output(self.id))
    }

    pub fn send<'a>(&self, message: impl Into<&'a T>) -> PortResult<()>
    where
        T: 'a,
    {
        let message: &T = message.into();
        let bytes = Bytes::from(message.encode_length_delimited_to_vec());
        self.transport.send(self.id, bytes)
    }
}

impl<T: Message> Port for OutputPort<T> {
    fn id(&self) -> Option<PortID> {
        Some(PortID::Output(self.id))
    }

    fn state(&self) -> PortState {
        self.transport
            .state(PortID::Output(self.id))
            .unwrap_or(PortState::Closed)
    }

    fn close(&mut self) -> PortResult<bool> {
        OutputPort::close(self)
    }
}

impl<T: Message> MessageSender<T> for OutputPort<T> {
    fn send<'a>(&self, message: impl Into<&'a T>) -> PortResult<()>
    where
        T: 'a,
    {
        OutputPort::send(self, message)
    }
}

impl<T: Message> fmt::Display for OutputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}→", self.id)
    }
}

impl<T: Message> fmt::Debug for OutputPort<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OutputPort").field("id", &self.id).finish()
    }
}
