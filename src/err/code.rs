#[derive(Debug, PartialEq)]
pub enum ErrCode {
    // class 00 - successful completion
    SuccessfulCompletion = 0x00000,
    // class 01 - warning
    Warning = 0x01000,
    DynamicResultSetsReturned = 0x0100C,
    ImplicitZeroBitPadding = 0x01008,
    NullValueEliminatedInSetFunction = 0x01003,
    PrivilegeNotGranted = 0x01007,
    PrivilegeNotRevoked = 0x01006,
    StringDataRightTruncation = 0x01004,
    DeprecatedFeature = 0x01201,
    // class 02 - no data
    NoData = 0x02000,
    NoAdditionalDynamicResultSetsReturned = 0x02001,
    // class 03 - sql statement not yet complete
    SqlStatementNotYetComplete = 0x03000,
    // class 08 - connection exception
    ConnectionException = 0x08000,
    ConnectionDoesNotExist = 0x08003,
    ConnectionFailure = 0x08006,
    SqlClientUnableToEstablishSqlConnection = 0x08001,
    SqlServerRejectedEstablishmentOfSqlConnection = 0x08004,
    TransactionResolutionUnknown = 0x08007,
    ProtocolViolation = 0x08201,
}
