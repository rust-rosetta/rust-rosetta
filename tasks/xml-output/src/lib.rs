use std::collections::HashMap;

use xml::writer::{EmitterConfig, XmlEvent};

pub fn characters_to_xml(characters: HashMap<String, String>) -> String {
    let mut output = Vec::new();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut output);

    writer
        .write(XmlEvent::start_element("CharacterRemarks"))
        .unwrap();

    for (character, line) in &characters {
        let element = XmlEvent::start_element("Character").attr("name", character);
        writer.write(element).unwrap();
        writer.write(XmlEvent::characters(line)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();
    }

    writer.write(XmlEvent::end_element()).unwrap();
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::characters_to_xml;
    use std::collections::HashMap;

    #[test]
    fn test_xml_output() {
        let mut input = HashMap::new();
        input.insert(
            "April".to_string(),
            "Bubbly: I'm > Tam and <= Emily".to_string(),
        );
        input.insert(
            "Tam O'Shanter".to_string(),
            // Using raw string literals let's us use double quotes without escaping
            r#"Burns: "When chapman billies leave the street ...""#.to_string(),
        );
        input.insert("Emily".to_string(), "Short & shrift".to_string());

        let output = characters_to_xml(input);

        println!("{}", output);
        assert!(output.contains(
            "<Character name=\"Tam O&apos;Shanter\">Burns: \"When chapman \
             billies leave the street ...\"</Character>"
        ));
        assert!(output
            .contains(r#"<Character name="April">Bubbly: I'm > Tam and &lt;= Emily</Character>"#));
        assert!(output.contains(r#"<Character name="Emily">Short &amp; shrift</Character>"#));
    }
}
