// This is free and unencumbered software released into the public domain.

use crate::{StdioConfig, StdioError, StdioSystem, System};
use protoflow_core::{
    prelude::{Duration, Range},
    Block, BlockResult, BlockRuntime, InputPort, Message, OutputPort, Port,
};
use protoflow_derive::Block;
use simple_mermaid::mermaid;

/// A block that passes messages through while delaying them by a fixed or
/// random duration.
///
/// # Block Diagram
#[doc = mermaid!("../../doc/core/delay.mmd")]
///
/// # Sequence Diagram
#[doc = mermaid!("../../doc/core/delay.seq.mmd" framed)]
///
/// # Examples
///
/// ## Using the block in a system
///
/// ```rust
/// # use protoflow_blocks::*;
/// # use std::time::Duration;
/// # fn main() {
/// System::build(|s| {
///     let stdin = s.read_stdin();
///     let line_decoder = s.decode_lines();
///     let delay = Duration::from_secs(1);
///     let delayer = s.delay_by_fixed::<String>(delay);
///     let line_encoder = s.encode_lines();
///     let stdout = s.write_stdout();
///     s.connect(&stdin.output, &line_decoder.input);
///     s.connect(&line_decoder.output, &delayer.input);
///     s.connect(&delayer.output, &line_encoder.input);
///     s.connect(&line_encoder.output, &stdout.input);
/// });
/// # }
/// ```
///
/// ## Running the block via the CLI
///
/// ```console
/// $ protoflow execute Delay fixed=1
/// ```
///
/// ```console
/// $ protoflow execute Delay random=1..10
/// ```
///
#[derive(Block, Clone)]
pub struct Delay<T: Message> {
    /// The input message stream.
    #[input]
    pub input: InputPort<T>,

    /// The output target for the stream being passed through.
    #[output]
    pub output: OutputPort<T>,

    /// A configuration parameter for which type of delay to add.
    #[parameter]
    pub delay: DelayType,
}

/// The type of delay (fixed or random) to apply to message relay.
#[derive(Clone, Debug)]
pub enum DelayType {
    Fixed(Duration),
    Random(Range<Duration>),
}

impl<T: Message> Delay<T> {
    pub fn new(input: InputPort<T>, output: OutputPort<T>) -> Self {
        Self::with_params(input, output, DelayType::Fixed(Duration::from_secs(1)))
    }

    pub fn with_params(input: InputPort<T>, output: OutputPort<T>, delay: DelayType) -> Self {
        Self {
            input,
            output,
            delay,
        }
    }
}

impl<T: Message> Block for Delay<T> {
    fn execute(&mut self, runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(message) = self.input.recv()? {
            if !self.output.is_connected() {
                drop(message);
                continue;
            }

            let duration = match self.delay {
                DelayType::Fixed(duration) => duration,
                DelayType::Random(ref range) => runtime.random_duration(range.clone()),
            };
            runtime.sleep_for(duration)?;

            self.output.send(&message)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: Message + crate::prelude::FromStr + crate::prelude::ToString + 'static> StdioSystem
    for Delay<T>
{
    fn build_system(config: StdioConfig) -> Result<System, StdioError> {
        use crate::{CoreBlocks, IoBlocks, SysBlocks, SystemBuilding};

        let fixed_delay = config
            .params
            .get("fixed")
            .map(|v| v.as_str().parse::<f64>());
        if let Some(Err(_)) = fixed_delay {
            return Err(StdioError::InvalidParameter("fixed"))?;
        }
        let fixed_delay = fixed_delay.map(Result::unwrap);
        let delay = DelayType::Fixed(Duration::from_secs_f64(fixed_delay.unwrap()));

        Ok(System::build(|s| {
            let stdin = s.read_stdin();
            let message_decoder = s.decode_with::<T>(config.encoding);
            let delayer = s.delay_by(delay);
            let message_encoder = s.encode_with::<T>(config.encoding);
            let stdout = s.write_stdout();
            s.connect(&stdin.output, &message_decoder.input);
            s.connect(&message_decoder.output, &delayer.input);
            s.connect(&delayer.output, &message_encoder.input);
            s.connect(&message_encoder.output, &stdout.input);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{Delay, DelayType};
    use crate::{prelude::Duration, System, SystemBuilding};

    #[test]
    fn instantiate_block() {
        // Check that the block is constructible:
        let _ = System::build(|s| {
            let _ = s.block(Delay::<i32>::with_params(
                s.input(),
                s.output(),
                DelayType::Fixed(Duration::from_secs(1)),
            ));
        });
    }
}
