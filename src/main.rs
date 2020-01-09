use clipboard::{ClipboardContext, ClipboardProvider};
use quick_xml::Reader;
use quick_xml::events::Event;
use clap::{*};
use std::env;
use log::{trace, debug, info, warn, error};


fn main() {
    let matches = App::new("Enum functions generator")
        .version("v2020.1.9")
        .author("Yangshuai <Yangshuai@Gmail.com>")
        .about("Generate enum function that can be used in JetBrains IDEs")
        .arg(Arg::with_name("from-lines")
            .short("l")
            .long("from-lines")
            .value_name("FROM-LINES")
            .help("Generate enum from lines of text in the clipboard")
            .takes_value(false))
        .arg(Arg::with_name("debug")
            .long("debug")
            .help("Show debugging info")
            .takes_value(false)
            .hidden(true))
        .get_matches();

    if matches.occurrences_of("debug") == 1 {
        warn!("Is in debugging mode.");
    }

    env::set_var("T2E_RUST_APP_LOG", "trace");
    pretty_env_logger::init_custom_env("T2E_RUST_APP_LOG");

    if matches.occurrences_of("debug") == 1 {
        trace!("Environment variable T2E_RUST_APP_LOG set!");

        if matches.occurrences_of("from-lines") == 1 {
            trace!("-from-lines: provided");
        }
    }

    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let text = clipboard.get_contents().unwrap();

    trace!("Data in clipboard: ");
    info!("{}", text);

    let mut result = "".to_string();

    if matches.occurrences_of("from-lines") == 1 {
        trace!("-from-lines: provided");
        result = get_enum_from_lines(&text);
    } else {
        result = get_enum_from_templates(&text);
    }

    trace!("Generated result (in clipboard): ");
    info!("{}", result);

    clipboard.set_contents(result.to_owned()).unwrap();
    env::remove_var("T2E_RUST_APP_LOG");

    if matches.occurrences_of("debug") == 1 {
        trace!("Environment variable T2E_RUST_APP_LOG removed!");
    }
}

fn get_enum_from_lines(text: &String) -> String {
    format!("enum(\"{}\")", text.lines().map(|line| line.replace("\"", "\\\"")).collect::<Vec<String>>().join("\", \""))
}

fn get_enum_from_templates(xml: &String) -> String {
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
    for e in &mut result {
        add_backslash_before_double_quote(e);
    }
    let result = format!("enum(\"{}\")", result.join("\", \""));
    result
}

fn add_backslash_before_double_quote(s: &mut String) {
    *s = s.replace("\"", "\\\"");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let xml = r###"
<template name="&gt;: redirect stderr to stdout (2&gt;&amp;1)" value="2&gt;&amp;1&#10;# Note the order when you want to redirect both stdout and stderr to a file.&#10;# You must first redirect stdout to a file, then stderr to stdout. Otherwise it &#10;# won't work!!!&#10;# geph-exit -singleHop :44444 &gt; /tmp/pk 2&gt;&amp;1" description="Redirect standard error to standard output." toReformat="false" toShortenFQNames="false">
  <context>
    <option name="SHELL_SCRIPT" value="true" />
  </context>
</template>
<template name="&gt;: redirect stdout and stderr to /dev/null (&gt; /dev/null 2&gt;&amp;1)" value="&gt; /dev/null 2&gt;&amp;1" description="" toReformat="false" toShortenFQNames="false">
  <context>
    <option name="SHELL_SCRIPT" value="true" />
  </context>
</template>
<template name="&gt;: redirect the result of multiple pipe line command (command1 | command2 | command3 &gt;&gt; file)" value="# Redirect stdout of (command1 | command2 | command3) to file&#10;# command1 | command2 | command3 &gt;&gt; file" description="command1 | command2 | command3 &gt;&gt; file" toReformat="false" toShortenFQNames="false">
  <context>
    <option name="SHELL_SCRIPT" value="true" />
  </context>
</template>
"###;
        let result = get_enum_from_templates(&xml.to_string());
        assert_eq!(result, r###"enum(">: redirect stderr to stdout (2>&1)", ">: redirect stdout and stderr to /dev/null (> /dev/null 2>&1)", ">: redirect the result of multiple pipe line command (command1 | command2 | command3 >> file)")"###);
    }


    #[test]
    fn test2() {
        let xml = r###"
<template name="mdict_after_blue_speaker" value="content: &quot;&quot;;&#10;background-image: url('data:image/png;base64,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');&#10;background-size: 20px;&#10;width: 20px;&#10;height: 20px;&#10;display: inline-block;&#10;background-repeat: no-repeat;&#10;vertical-align: middle;&#10;margin-right: 20px;" description=":after background" toReformat="false" toShortenFQNames="true">
  <context>
    <option name="CSS" value="true" />
  </context>
</template>
<template name="mdict_after_red_speaker" value="content: &quot;&quot;;&#10;background-image: url('data:image/png;base64,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');&#10;background-size: 20px;&#10;width: 20px;&#10;height: 20px;&#10;display: inline-block;&#10;background-repeat: no-repeat;&#10;vertical-align: middle;&#10;margin-right: 20px;" description=":after background" toReformat="false" toShortenFQNames="true">
  <context>
    <option name="CSS" value="true" />
  </context>
</template>
"###;
        let result = get_enum_from_templates(&xml.to_string());
        assert_eq!(result, r###"enum("mdict_after_blue_speaker", "mdict_after_red_speaker")"###);
    }


    #[test]
    fn test3() {
        let xml = r###"
<template name="mdict_after_&quot;blue'_speaker" value="content: &quot;&quot;;&#10;background-image: url('data:image/png;base64,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');&#10;background-size: 20px;&#10;width: 20px;&#10;height: 20px;&#10;display: inline-block;&#10;background-repeat: no-repeat;&#10;vertical-align: middle;&#10;margin-right: 20px;" description=":after background" toReformat="false" toShortenFQNames="true">
  <context>
    <option name="CSS" value="true" />
  </context>
</template>
<template name="mdict_after_red_speaker" value="content: &quot;&quot;;&#10;background-image: url('data:image/png;base64,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');&#10;background-size: 20px;&#10;width: 20px;&#10;height: 20px;&#10;display: inline-block;&#10;background-repeat: no-repeat;&#10;vertical-align: middle;&#10;margin-right: 20px;" description=":after background" toReformat="false" toShortenFQNames="true">
  <context>
    <option name="CSS" value="true" />
  </context>
</template>
"###;
        let result = get_enum_from_templates(&xml.to_string());
        assert_eq!(result, "enum(\"mdict_after_\\\"blue'_speaker\", \"mdict_after_red_speaker\")");
    }

    #[test]
    fn test4() {
        let data = r#"g
3
g3"#;
        let result = get_enum_from_lines(&data.to_string());
        assert_eq!(result, "enum(\"g\", \"3\", \"g3\")")
    }

    #[test]
    fn test5() {
        let data = r#"
g
3
g3
--------------------------------
Nothing: the first occurence in every line will be replaced
g: all occurences will be replaced
3: the 3rd occurrence will be replaced (count from 1)
g3 or 3g: occurrence 3, 4, 5, ... will be replaced"#;
        let result = get_enum_from_lines(&data.to_string());
        assert_eq!(result, "enum(\"\", \"g\", \"3\", \"g3\", \"--------------------------------\", \"Nothing: the first occurence in every line will be replaced\", \"g: all occurences will be replaced\", \"3: the 3rd occurrence will be replaced (count from 1)\", \"g3 or 3g: occurrence 3, 4, 5, ... will be replaced\")")
    }

    #[test]
    fn test6() {
        let data = r#"
g
3
g"3
--------------------------------
No"thing: the first occurence in every line will be replaced
g: all occurences will be replaced
3: the 3rd occurrence will be replaced (count from 1)
g3 or 3g: occurrence 3, 4, 5, ... will be replaced"#;
        let result = get_enum_from_lines(&data.to_string());
        assert_eq!(result, "enum(\"\", \"g\", \"3\", \"g\\\"3\", \"--------------------------------\", \"No\\\"thing: the first occurence in every line will be replaced\", \"g: all occurences will be replaced\", \"3: the 3rd occurrence will be replaced (count from 1)\", \"g3 or 3g: occurrence 3, 4, 5, ... will be replaced\")")
    }
}
