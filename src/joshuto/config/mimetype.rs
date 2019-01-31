extern crate toml;
extern crate xdg;

use std::fmt;
use std::collections::HashMap;
use std::process;

#[derive(Debug, Deserialize)]
pub struct JoshutoMimetypeEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
    pub fork: Option<bool>,
    pub silent: Option<bool>,
}

impl std::fmt::Display for JoshutoMimetypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.write_str(self.program.as_str()).unwrap();
        if let Some(s) = self.args.as_ref() {
            for arg in s {
                write!(f, " {}", arg).unwrap();
            }
        }
        f.write_str("\t[").unwrap();
        if let Some(s) = self.fork {
            if s {
                f.write_str("fork,").unwrap();
            }
        }
        if let Some(s) = self.silent {
            if s {
                f.write_str("silent").unwrap();
            }
        }
        f.write_str("]")
    }
}

#[derive(Debug, Deserialize)]
pub struct JoshutoRawMimetype {
    mimetypes: Option<HashMap<String, Vec<JoshutoMimetypeEntry>>>,
    extensions: Option<HashMap<String, Vec<JoshutoMimetypeEntry>>>,
}

impl JoshutoRawMimetype {
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        JoshutoRawMimetype {
            mimetypes: None,
            extensions: None,
        }
    }

    pub fn flatten(self) -> JoshutoMimetype
    {
        let mimetypes = self.mimetypes.unwrap_or(HashMap::new());
        let extensions = self.extensions.unwrap_or(HashMap::new());

        JoshutoMimetype {
            mimetypes,
            extensions
        }
    }
}

#[derive(Debug)]
pub struct JoshutoMimetype {
    pub mimetypes: HashMap<String, Vec<JoshutoMimetypeEntry>>,
    pub extensions: HashMap<String, Vec<JoshutoMimetypeEntry>>,
}

impl JoshutoMimetype {

    pub fn new() -> Self
    {
        JoshutoMimetype {
            mimetypes: HashMap::new(),
            extensions: HashMap::new(),
        }
    }

    fn read_config() -> Option<JoshutoRawMimetype> {
        let config_contents = crate::joshuto::config::read_config(::MIMETYPE_FILE)?;
        match toml::from_str(&config_contents) {
            Ok(config) => {
                Some(config)
            },
            Err(e) => {
                eprintln!("Error parsing mimetype file: {}", e);
                process::exit(1);
            },
        }
    }

    pub fn get_config() -> Self
    {
        match Self::read_config() {
            Some(config) => {
                config.flatten()
            }
            None => {
                Self::new()
            }
        }
    }
}
