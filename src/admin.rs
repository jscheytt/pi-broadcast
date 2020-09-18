use std::{env, fs, path::Path};

pub fn audio_names() -> Vec<String> {
    let paths = fs::read_dir(&Path::new("media")).unwrap();
    let audio_extension = ".mp3";
    paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .filter(|name| name.ends_with(audio_extension))
        .map(|name| name.replace(audio_extension, ""))
        .collect::<Vec<String>>()
}

pub fn current_path() -> String {
    let current_path = env::current_dir().expect("Could not find directory");
    format!("{}/media", current_path.display())
}
