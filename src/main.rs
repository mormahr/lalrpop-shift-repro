use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    test,
    "/test.rs"
);

fn main() {
    let parser = test::StmtsParser::new();
    let result = parser.parse("5-5");
    println!("{:?}", result);
}
