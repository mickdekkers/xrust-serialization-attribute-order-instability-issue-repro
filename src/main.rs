use std::rc::Rc;

use xrust::item::Node;
use xrust::parser::xml::parse as xmlparse;
use xrust::trees::smite::Node as SmiteNode;

fn main() {
    let doc_text = include_str!("../data/example.xml");

    let mut prev_xml_output: Option<String> = None;
    for iteration in 0..10 {
        let doc = xmlparse(Rc::new(SmiteNode::new()), doc_text, None).unwrap();
        let xml_output = doc.to_xml();

        if let Some(prev_xml_output) = &prev_xml_output {
            similar_asserts::assert_eq!(
                &xml_output,
                prev_xml_output,
                "output of iteration {} != output of iteration {}",
                iteration,
                iteration - 1
            );
        }

        prev_xml_output = Some(xml_output);
    }
}
