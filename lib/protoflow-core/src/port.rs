// This is free and unencumbered software released into the public domain.

use crate::{prelude::ToString, PortError, PortID, PortResult, PortState};

/// The common interface for ports, whether for input or output.
pub trait Port {
    /// A unique identifier for this port.
    fn id(&self) -> Option<PortID>;

    /// The current state of this port.
    fn state(&self) -> PortState;

    /// The machine-readable name of this port.
    fn name(&self) -> Option<&str> {
        None
    }

    /// A human-readable label for this port.
    fn label(&self) -> Option<&str> {
        None
    }

    /// Checks whether this port is currently closed.
    fn is_closed(&self) -> bool {
        self.state().is_closed()
    }

    /// Checks whether this port is currently open.
    fn is_open(&self) -> bool {
        self.state().is_open()
    }

    /// Checks whether this port is currently connected.
    fn is_connected(&self) -> bool {
        self.state().is_connected()
    }

    /// Closes this port, returning immediately.
    ///
    /// If the port had an open connection, it will be disconnected.
    /// If the port was already closed, no further action is taken.
    /// There is no facility to reopen a port once it has been closed.
    ///
    /// Returns `Ok(true)` if the port was successfully closed.
    /// Returns `Ok(false)` if the port was already closed.
    /// Returns `Err(PortError)` if an error occurs.
    fn close(&mut self) -> PortResult<bool> {
        Err(PortError::Other("not implemented".to_string()))
    }
}
