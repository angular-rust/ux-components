pub trait InlineSpan {
    // The TextStyle to apply to this span.
    // style: TextStyle?

    // build(ParagraphBuilder builder, {double textScaleFactor = 1.0, List<PlaceholderDimensions>? dimensions}) → void
    // Apply the properties of this object to the given ParagraphBuilder, from which a Paragraph can be obtained.
    // codeUnitAt(int index) → int?
    // Returns the UTF-16 code unit at the given index in the flattened string.
    // codeUnitAtVisitor(int index, Accumulator offset) → int?
    // Performs the check at each InlineSpan for if the index falls within the range of the span and returns the corresponding code unit. Returns null otherwise.
    // @protected
    // compareTo(InlineSpan other) → RenderComparison
    // Describe the difference between this span and another, in terms of how much damage it will make to the rendering. The comparison is deep.
    // computeSemanticsInformation(List<InlineSpanSemanticsInformation> collector) → void
    // Walks the InlineSpan tree and accumulates a list of InlineSpanSemanticsInformation objects.
    // @protected
    // computeToPlainText(StringBuffer buffer, {bool includeSemanticsLabels = true, bool includePlaceholders = true}) → void
    // Walks the InlineSpan tree and writes the plain text representation to buffer.
    // @protected
    // debugAssertIsValid() → bool
    // In debug mode, throws an exception if the object is not in a valid configuration. Otherwise, returns true.
    // debugDescribeChildren() → List<DiagnosticsNode>
    // Returns a list of DiagnosticsNode objects describing this node's children.
    // @protected, inherited
    // debugFillProperties(DiagnosticPropertiesBuilder properties) → void
    // Add additional properties associated with the node.
    // override
    // getSemanticsInformation() → List<InlineSpanSemanticsInformation>
    // Flattens the InlineSpan tree to a list of InlineSpanSemanticsInformation objects.
    // getSpanForPosition(TextPosition position) → InlineSpan?
    // Returns the InlineSpan that contains the given position in the text.
    // getSpanForPositionVisitor(TextPosition position, Accumulator offset) → InlineSpan?
    // Performs the check at each InlineSpan for if the position falls within the range of the span and returns the span if it does.
    // @protected
    // noSuchMethod(Invocation invocation) → dynamic
    // Invoked when a non-existent method or property is accessed.
    // inherited
    // toDiagnosticsNode({String? name, DiagnosticsTreeStyle? style}) → DiagnosticsNode
    // Returns a debug representation of the object that is used by debugging tools and by DiagnosticsNode.toStringDeep.
    // inherited
    // toPlainText({bool includeSemanticsLabels = true, bool includePlaceholders = true}) → String
    // Flattens the InlineSpan tree into a single string.
    // toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) → String
    // A string representation of this object.
    // inherited
    // toStringDeep({String prefixLineOne = '', String? prefixOtherLines, DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
    // Returns a string representation of this node and its descendants.
    // inherited
    // toStringShallow({String joiner = ', ', DiagnosticLevel minLevel = DiagnosticLevel.debug}) → String
    // Returns a one-line detailed description of the object.
    // inherited
    // toStringShort() → String
    // A brief description of this object, usually just the runtimeType and the hashCode.
    // inherited
    // visitChildren(InlineSpanVisitor visitor) → bool
    // Walks this InlineSpan and any descendants in pre-order and calls visitor for each span that has content.
}

pub struct NoneInlineSpan;

impl InlineSpan for NoneInlineSpan {
    
}