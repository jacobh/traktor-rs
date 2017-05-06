use std;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use xml;
use xml::reader::{EventReader, XmlEvent};

type EntryElements = Vec<XmlEvent>;

pub struct Entry {
    pub elements: EntryElements,
}

pub struct Entries {
    _parser: EventReader<std::io::BufReader<File>>,
}

impl Entries {
    pub fn new<P: AsRef<Path>>(collection_path: P) -> Entries {
        let file = File::open(collection_path).unwrap();
        let file = BufReader::new(file);
        Entries { _parser: EventReader::new(file) }
    }
}

impl Iterator for Entries {
    type Item = Entry;
    fn next(&mut self) -> Option<Entry> {
        let mut entry_elements = EntryElements::new();
        loop {
            match self._parser.next() {
                Ok(e) => {
                    match e {
                        XmlEvent::StartElement { .. } => {
                            match entry_elements.is_empty() {
                                true => {
                                    let is_entry = {
                                        match e {
                                            XmlEvent::StartElement { ref name, .. } => {
                                                name.local_name == "ENTRY"
                                            }
                                            _ => false,
                                        }
                                    };
                                    if is_entry {
                                        entry_elements.push(e);
                                    }
                                }
                                false => {
                                    entry_elements.push(e);
                                }
                            }
                        }
                        XmlEvent::EndElement { name } => {
                            if name.local_name == "ENTRY" {
                                return Some(Entry { elements: entry_elements });
                            }
                        }
                        XmlEvent::EndDocument => {
                            break;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
        return None;
    }
}

fn get_element_with_name<'a, 'b>(elements: &'a EntryElements,
                                 lookup_name: &'b str)
                                 -> Option<&'a XmlEvent> {
    elements
        .iter()
        .find(|x| match *x {
                  &XmlEvent::StartElement { ref name, .. } => &name.local_name == lookup_name,
                  _ => false,
              })
}

fn get_attribute(attributes: &Vec<xml::attribute::OwnedAttribute>, key: &str) -> Option<String> {
    attributes
        .iter()
        .find(|&x| x.name.local_name == key)
        .and_then(|x| Some(x.value.clone()))
}

pub fn get_element_attribute(elements: &EntryElements,
                             element_name: &str,
                             attribute_key: &str)
                             -> Option<String> {
    match get_element_with_name(elements, element_name) {
        Some(element) => {
            match element {
                &XmlEvent::StartElement { ref attributes, .. } => {
                    get_attribute(&attributes, attribute_key)
                }
                _ => None,
            }
        }
        _ => None,
    }
}