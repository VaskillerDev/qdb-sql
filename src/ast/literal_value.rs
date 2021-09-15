pub enum  LiteralValue {
    NumericLiteral(i128),
    StringLiteral(str),
    BlobLiteral(str),
    Null,
    True,
    False,
    CurrentTime,
    CurrentDate,
    CurrentTimestamp
}