use crate::sql92;

#[test]
fn slq92_test() {
    let res = sql92::ExprParser::new().parse("2+2");

    println!("{:?}", res);
}