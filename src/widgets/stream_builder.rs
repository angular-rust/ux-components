use futures::Stream;

use crate::foundation::Key;

use super::{BuildContext, Widget, NoneWidget};

pub enum ConnectionState {
    // Not currently connected to any asynchronous computation.
    // For example, a FutureBuilder whose FutureBuilder.future is null.
    None = 0,

    // Connected to an asynchronous computation and awaiting interaction.
    Waiting = 1,

    // Connected to an active asynchronous computation.
    // For example, a Stream that has returned at least one value, but is not yet done.
    Active = 2,

    // Connected to a terminated asynchronous computation.
    Done = 3,
}

pub struct AsyncSnapshot<T> {
    // Current state of connection to the asynchronous computation.
    connection_state: ConnectionState,

    // The latest data received by the asynchronous computation.
    data: Option<T>, // The latest error object received by the asynchronous computation.
                     // error: Object?

                     // Returns whether this snapshot contains a non-null data value.
                     // hasData: bool

                     // Returns whether this snapshot contains a non-null error value.
                     // hasError: bool

                     // Returns latest data received, failing if there is no data.
                     // requireData: T

                     // The latest stack trace object received by the asynchronous computation.
                     // stackTrace: StackTrace?
}

impl<T> AsyncSnapshot<T> {
    // AsyncSnapshot.nothing()
    // Creates an AsyncSnapshot in ConnectionState.none with null data and error.
    // const
    // AsyncSnapshot.waiting()
    // Creates an AsyncSnapshot in ConnectionState.waiting with null data and error.
    // const
    // AsyncSnapshot.withData(ConnectionState state, T data)
    // Creates an AsyncSnapshot in the specified state and with the specified data.
    // const
    // AsyncSnapshot.withError(ConnectionState state, Object error, [StackTrace stackTrace = StackTrace.empty])
    // Creates an AsyncSnapshot in the specified state with the specified error and a stackTrace.
    // const

    // Returns a snapshot like this one, but in the specified state.
    // inState(ConnectionState state) -> AsyncSnapshot<T>
}

// type AsyncWidgetBuilder<T> = dyn Fn(context: BuildContext, snapshot: AsyncSnapshot<T> ) -> Widget;
type AsyncWidgetBuilder<T> = Box<dyn Fn(BuildContext, AsyncSnapshot<T>) -> Box<dyn Widget>>;

pub struct StreamBuilder<T> {
    // The build strategy currently used by this builder.
    pub builder: AsyncWidgetBuilder<T>,

    // The data that will be used to create the initial snapshot.
    pub initial_data: Option<T>,

    // Controls how one widget replaces another widget in the tree.
    pub key: Key,
    
    // The asynchronous computation to which this builder is currently connected,
    // possibly null. When changed, the current summary is updated using afterDisconnected,
    // if the previous stream was not null, followed by afterConnected, if the new stream is not null.
    pub stream: Option<Box<dyn Stream<Item = T>>>
}

impl<T> StreamBuilder<T> {
    
    // NOT WORKING. JUST FOR REMEMBER
    // pub const NONE: AsyncWidgetBuilder<T> = box |_, _| {
    //     box NoneWidget
    // };

    // afterConnected(AsyncSnapshot<T> current) -> AsyncSnapshot<T>
    // Returns an updated version of the current summary reflecting that we are now connected to a stream.
    // override
    // afterData(AsyncSnapshot<T> current, T data) -> AsyncSnapshot<T>
    // Returns an updated version of the current summary following a data event.
    // override
    // afterDisconnected(AsyncSnapshot<T> current) -> AsyncSnapshot<T>
    // Returns an updated version of the current summary reflecting that we are no longer connected to a stream.
    // override
    // afterDone(AsyncSnapshot<T> current) -> AsyncSnapshot<T>
    // Returns an updated version of the current summary following stream termination.
    // override
    // afterError(AsyncSnapshot<T> current, Object error, StackTrace stackTrace) -> AsyncSnapshot<T>
    // Returns an updated version of the current summary following an error with a stack trace.
    // override
    // build(BuildContext context, AsyncSnapshot<T> currentSummary) -> Widget
    // Returns a Widget based on the currentSummary.
    // override
    // createElement() -> StatefulElement
    // Creates a StatefulElement to manage this widget's location in the tree.
    // inherited
    // createState() -> State<StreamBuilderBase<T, AsyncSnapshot<T>>>
    // Creates the mutable state for this widget at a given location in the tree.
    // inherited
    // debugDescribeChildren() -> List<DiagnosticsNode>
    // Returns a list of DiagnosticsNode objects describing this node's children.
    // @protected, inherited
    // debugFillProperties(DiagnosticPropertiesBuilder properties) -> void
    // Add additional properties associated with the node.
    // inherited
    // initial() -> AsyncSnapshot<T>
    // Returns the initial summary of stream interaction, typically representing the fact that no interaction has happened at all.
    // override
    // noSuchMethod(Invocation invocation) -> dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // toDiagnosticsNode({String? name, DiagnosticsTreeStyle? style}) -> DiagnosticsNode
    // Returns a debug representation of the object that is used by debugging tools and by DiagnosticsNode.toStringDeep.
    // inherited
    // toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) -> String
    // A string representation of this object.
    // inherited
    // toStringDeep({String prefixLineOne = '', String? prefixOtherLines, DiagnosticLevel minLevel = DiagnosticLevel.debug}) -> String
    // Returns a string representation of this node and its descendants.
    // inherited
    // toStringShallow({String joiner = ', ', DiagnosticLevel minLevel = DiagnosticLevel.debug}) -> String
    // Returns a one-line detailed description of the object.
    // inherited
    // toStringShort() -> String
    // A short, textual description of this widget.
    // inherited
}

impl<T> Default for StreamBuilder<T> {
    fn default() -> Self {
        Self {
            builder: box |_, _| {
                box NoneWidget
            },
            initial_data: Default::default(),
            key: Default::default(),
            stream: Default::default(),
        }
    }
}
