use core::ProtoParser;
use core::proto3::Proto3;

#[test]
fn simple_proto() -> Result<(), Box<dyn std::error::Error>> {
    let files = std::fs::read_dir("../../passport/proto/passport/api/module")?;
    for file in files {
        let path = file?.path();
        println!("Testing {}", &path.to_string_lossy());
        if path.is_dir() {
            continue;
        }
        let data = std::fs::read_to_string(path)?;
        println!("{:?}", Proto3::default().parse(data.as_str())?);
    }
    Ok(())
}
