use std::fmt::{Display, Formatter};
use crate::err::code::ErrCode;

#[derive(Debug)]
pub struct OpResult {
    code : ErrCode,
    text : String
}

impl Display for OpResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}", self)
    }
}

impl PartialEq<Self> for OpResult {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

mod test {
    use crate::err::code::ErrCode;
    use crate::err::result::OpResult;

    #[test]
    fn display_op_result() {
        let op_res = OpResult {
            code: ErrCode::SuccessfulCompletion,
            text: String::from("Successful completion")
        };

        println!("{:?}", op_res);
        assert_eq!(op_res.text, String::from("Successful completion"));
    }

    #[test]
    fn cmp_op_result() {
        let op_res = OpResult {
            code: ErrCode::SuccessfulCompletion,
            text: String::from("Successful completion")
        };

        let op_res2 = OpResult {
            code: ErrCode::SuccessfulCompletion,
            text: String::from("Done")
        };

        assert_eq!(op_res, op_res2);
    }

    #[test]
    fn negative_cmp_op_result() {
        let op_res = OpResult {
            code: ErrCode::SuccessfulCompletion,
            text: String::from("Successful completion")
        };

        let op_res2 = OpResult {
            code: ErrCode::SqlClientUnableToEstablishSqlConnection,
            text: String::from("Establish connection")
        };

        assert_ne!(op_res, op_res2);
    }
}