use harpi::proto3::Proto3;
use harpi::{ProtoParser, ProtoVisitor};

#[derive(ProtoParser)]
#[parser(Proto3)]
struct MyParser;

#[derive(Default)]
struct NoOpVisitor;

impl ProtoVisitor for NoOpVisitor {
    fn on<'a>(&mut self, _: harpi::Node<'a>) {}
}
#[test]
fn simple_proto() -> Result<(), Box<dyn std::error::Error>> {
    let files = std::fs::read_dir("./proto")?;
    for file in files {
        let path = file?.path();
        println!("Testing {}", &path.to_string_lossy());
        if path.is_dir() {
            continue;
        }
        let data = std::fs::read_to_string(path)?;
        let mut visitor = NoOpVisitor::default();
        MyParser::parse(data.as_str(), &mut visitor)?
    }
    Ok(())
}
