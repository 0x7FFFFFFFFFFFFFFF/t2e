use clipboard::{ClipboardContext, ClipboardProvider};
use quick_xml::Reader;
use quick_xml::events::Event;


fn main() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let xml = clipboard.get_contents().unwrap();

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut count = 0;
    let mut buf = Vec::new();

    let mut result: Vec<String> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"template" => result.push(e.attributes().find(|x| x.as_ref().unwrap().key == b"name").unwrap().unwrap().unescape_and_decode_value(&reader).unwrap()),
                    _ => (),
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
    println!("{:?}", format!("enum(\"{}\")", result.join("\", \"")));
    let result = format!("enum(\"{}\")", result.join("\", \""));
    clipboard.set_contents(result.to_owned()).unwrap();
}
