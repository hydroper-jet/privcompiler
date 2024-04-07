#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedOrInvalidToken = 1024,
    UnexpectedEnd = 1025,
    UnallowedNumericSuffix = 1026,
    UnallowedLineBreak = 1027,
    Expected = 1028,
    ExpectedIdentifier = 1029,
    ExpectedExpression = 1030,
    ExpectedXmlName = 1031,
    ExpectedXmlAttributeValue = 1032,
    IllegalNullishCoalescingLeftOperand = 1033,
    WrongParameterPosition = 1034,
    DuplicateRestParameter = 1035,
    NotAllowedHere = 1036,
    MalformedRestParameter = 1037,
    IllegalForInInitializer = 1038,
    MultipleForInBindings = 1039,
    UndefinedLabel = 1040,
    IllegalContinue = 1041,
    IllegalBreak = 1042,
    ExpressionMustNotFollowLineBreak = 1043,
    TokenMustNotFollowLineBreak = 1044,
    ExpectedStringLiteral = 1045,
    DuplicateAttribute = 1046,
    DuplicateVisibility = 1047,
    ExpectedDirectiveKeyword = 1048,
    UnallowedAttribute = 1049,
    MalformedEnumMember = 1051,
    FunctionMayNotBeGenerator = 1052,
    FunctionMayNotBeAsynchronous = 1053,
    FunctionMustNotContainBody = 1054,
    FunctionMustContainBody = 1055,
    FunctionMustNotContainAnnotations = 1056,
    NestedClassesNotAllowed = 1057,
    DirectiveNotAllowedInInterface = 1058,
    FailedParsingJetDocTag = 1059,
    UnrecognizedJetDocTag = 1060,
    UnrecognizedProxy = 1061,
    EnumMembersMustBeConst = 1062,
    ConstructorMustNotSpecifyResultType = 1063,
    IncompatibleTypes = 1064,
    ReferenceIsWriteOnly = 1065,
    ReferenceIsReadOnly = 1066,
    ReferenceIsNotDeletable = 1067,
    AmbiguousReference = 1068,
    AccessingPropertyOfVoidBase = 1069,
    AccessingPropertyOfNullableBase = 1070,
    InaccessibleProperty = 1071,
    ParameterizedTypeMustBeArgumented = 1072,
    UnrecognizedEmbedExpressionField = 1073,
    MustResolveToType = 1074,
    EmbedSourceOrTypeNotSpecified = 1075,
    EmbedUnsupportedType = 1076,
    FailedLoadingEmbeddedFile = 1077,
    FailedParsingNumericLiteral = 1078,
    StringLiteralMustBeASingleCharacter = 1079,
    EnumerationHasNoMember = 1080,
    UnrecognizedMetadataSyntax = 1081,
    FailedLoadingMetadataFile = 1082,
    IllegalThisReference = 1083,
    CannotUseTypeInRest = 1084,
    ArrayLiteralMustNotContainElision = 1085,
    ArrayLiteralMustNotContainRest = 1086,
    ArrayLiteralExceedingTupleElements = 1087,
    InitializerUnsupportedType = 1088,
    UndefinedProperty = 1089,
    IncompatibleFieldKey = 1090,
    MissingPropertyInLiteral = 1091,
}

impl DiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}