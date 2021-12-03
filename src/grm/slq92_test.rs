use crate::sql92;

#[test]
fn slq92_test() {
    let res = sql92::TermParser::new().parse("22");

    println!("{:?}", res);
}