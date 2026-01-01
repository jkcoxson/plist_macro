/// Converts a PLIST dictionary to XML-formatted bytes
///
/// # Arguments
/// * `p` - The PLIST dictionary to serialize
///
/// # Returns
/// A byte vector containing the XML representation
///
/// # Panics
/// Will panic if serialization fails (should only happen with invalid data)
///
/// # Example
/// ```rust
/// use plist_macro::*;
///
/// let mut dict = plist::Dictionary::new();
/// dict.insert("key".into(), "value".into());
/// let xml_bytes = plist_to_xml_bytes(&dict);
/// ```
pub fn plist_to_xml_bytes(p: &plist::Dictionary) -> Vec<u8> {
    let buf = Vec::new();
    let mut writer = std::io::BufWriter::new(buf);
    plist::to_writer_xml(&mut writer, &p).unwrap();

    writer.into_inner().unwrap()
}
