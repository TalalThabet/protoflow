// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{Arc, Box, PhantomData, Rc, RefCell, VecDeque},
    runtimes::StdRuntime,
    transports::MpscTransport,
    Block, BlockResult, InputPort, Message, OutputPort, Process, Runtime, Transport,
};

pub trait SystemBuilding {
    /// Creates a new input port inside the system.
    fn input<M: Message + 'static>(&self) -> InputPort<M>;

    /// Creates a new output port inside the system.
    fn output<M: Message + 'static>(&self) -> OutputPort<M>;

    /// Instantiates a block inside the system.
    fn block<B: Block + Clone + 'static>(&self, block: B) -> B;

    /// Connects two ports of two blocks in the system.
    ///
    /// Both ports must be of the same message type.
    fn connect<M: Message>(&self, source: &OutputPort<M>, target: &InputPort<M>) -> bool;
}

pub trait SystemExecution {
    /// Executes the system, returning the system process.
    fn execute(self) -> BlockResult<Rc<dyn Process>>;
}

/// A system is a collection of blocks that are connected together.
pub struct System<X: Transport + Default + 'static = MpscTransport> {
    pub(crate) runtime: Arc<StdRuntime<X>>,

    /// The registered blocks in the system.
    pub(crate) blocks: RefCell<VecDeque<Box<dyn Block>>>,

    _phantom: PhantomData<X>,
}

pub type Subsystem<X> = System<X>;

impl<X: Transport + Default + 'static> System<X> {
    /// Builds a new system.
    pub fn build<F: FnOnce(&mut System<X>)>(f: F) -> Self {
        let transport = X::default();
        let runtime = StdRuntime::new(transport).unwrap();
        let mut system = System::new(&runtime);
        f(&mut system);
        system
    }

    /// Instantiates a new system.
    pub fn new(runtime: &Arc<StdRuntime<X>>) -> Self {
        Self {
            runtime: runtime.clone(),
            blocks: RefCell::new(VecDeque::new()),
            _phantom: PhantomData,
        }
    }

    pub fn execute(self) -> BlockResult<Rc<dyn Process>> {
        let mut runtime = self.runtime.clone();
        runtime.execute(self)
    }

    pub fn input<M: Message + 'static>(&self) -> InputPort<M> {
        InputPort::new(self)
    }

    pub fn output<M: Message + 'static>(&self) -> OutputPort<M> {
        OutputPort::new(self)
    }

    pub fn block<B: Block + Clone + 'static>(&self, block: B) -> B {
        self.blocks.borrow_mut().push_back(Box::new(block.clone()));
        block
    }

    pub fn connect<M: Message>(&self, source: &OutputPort<M>, target: &InputPort<M>) -> bool {
        let runtime = self.runtime.as_ref();
        let transport = runtime.transport.as_ref();
        transport.connect(source.id, target.id).unwrap()
    }
}

impl SystemBuilding for System {
    fn input<M: Message + 'static>(&self) -> InputPort<M> {
        System::input(self)
    }

    fn output<M: Message + 'static>(&self) -> OutputPort<M> {
        System::output(self)
    }

    fn block<B: Block + Clone + 'static>(&self, block: B) -> B {
        System::block(self, block)
    }

    fn connect<M: Message>(&self, source: &OutputPort<M>, target: &InputPort<M>) -> bool {
        System::connect(self, source, target)
    }
}

impl SystemExecution for System {
    fn execute(self) -> BlockResult<Rc<dyn Process>> {
        System::execute(self)
    }
}
