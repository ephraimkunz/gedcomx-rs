use gedcomx::Gedcomx;

fn main() {
    let s = r##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <gedcomx xmlns="http://gedcomx.org/v1/">
        <attribution>
            <contributor resource="#A-1"/>
            <modified>2014-03-07T00:00:00-07:00</modified>
        </attribution>
        <person extracted="true" id="P-1">
            <source description="#S-1"/>
            <gender type="http://gedcomx.org/Female"/>
            <name>
                <nameForm>
                    <fullText>Emma Bocock</fullText>
                </nameForm>
            </name>
            <fact type="http://gedcomx.org/Birth">
                <date>
                    <original>23 June 1843</original>
                </date>
                <place>
                    <original>Broadfield Bar, Abbeydale Road, Ecclesall-Bierlow, York, England, United Kingdom</original>
                </place>
            </fact>
        </person>
        <person extracted="true" id="P-2">
            <source description="#S-1"/>
            <name>
                <nameForm>
                    <fullText>William Bocock</fullText>
                </nameForm>
            </name>
            <fact type="http://gedcomx.org/Occupation">
                <value>Toll Collector</value>
            </fact>
        </person>
        <person extracted="true" id="P-3">
            <source description="#S-1"/>
            <name>
                <nameForm>
                    <fullText>Sarah Bocock formerly Brough</fullText>
                </nameForm>
            </name>
        </person>
        <person id="C-1">
            <analysis resource="#D-1"/>
            <evidence resource="#P-1"/>
        </person>
        <relationship type="http://gedcomx.org/ParentChild">
            <person1 resource="#P-2"/>
            <person2 resource="#P-1"/>
        </relationship>
        <relationship type="http://gedcomx.org/ParentChild">
            <person1 resource="#P-3"/>
            <person2 resource="#P-1"/>
        </relationship>
        <sourceDescription resourceType="http://gedcomx.org/PhysicalArtifact" id="S-1">
            <citation>
                <value>England, birth certificate for Emma Bocock, born 23 July 1843; citing 1843 Birth in District and Sub-district of Ecclesall-Bierlow in the County of York, 303; General Registry Office, Southport.</value>
            </citation>
            <title>Birth Certificate of Emma Bocock, 23 July 1843, General Registry Office</title>
            <created>1843-07-27T00:00:00-07:00</created>
            <repository resource="#A-2"/>
        </sourceDescription>
        <agent id="A-1">
            <email resource="mailto:example@example.org"/>
            <name>Jane Doe</name>
        </agent>
        <agent id="A-2">
            <name>General Registry Office, Southport</name>
        </agent>
        <document id="D-1">
            <text>...Jane Doe's analysis document...</text>
        </document>
    </gedcomx>
    "##;

    let gx: Gedcomx = quick_xml::de::from_str(s).unwrap();
    println!("{:?}", gx);
    // for dir_entry in std::fs::read_dir("data").unwrap() {
    //     let path = dir_entry.unwrap().path();
    //     let json = std::fs::read_to_string(path.clone()).unwrap();
    //     match serde_json::from_str::<Gedcomx>(&json) {
    //         Ok(g) => println!("Success: {:?}\n{:?}\n", path, g),
    //         Err(e) => println!("Failure: {:?} -> {:?}\n", path, e),
    //     };
    // }
}
