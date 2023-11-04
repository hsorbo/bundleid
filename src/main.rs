use std::{
    env,
    io::{Cursor, Read},
};

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("Usage: bundleid filname.ipa")?;
    let file = std::fs::File::open(&filename).map_err(|error| format!("Error opening file: {error}"))?;
    let reader = std::io::BufReader::new(file);
    let mut archive =  zip::ZipArchive::new(reader).map_err(|error| format!("Error opening zip: {error}"))?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let outpath = match entry.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if outpath.to_str().unwrap().ends_with(".app/Info.plist") {
            let mut buffer = Vec::new();
            entry.read_to_end(&mut buffer).unwrap();
            let reader = Cursor::new(buffer);
            let val = plist::Value::from_reader(reader).unwrap();
            let dict = val.as_dictionary().unwrap();
            let bundle_id =
                dict.get("CFBundleIdentifier").unwrap().as_string().unwrap();
            println!("{bundle_id}");
            return Ok(());
        }
    }
    Err("No Info.plist found".to_string())
}

/*
    let entry = open(filename)
    |> toReader
    |> files
    |> filter (f -> f.name.endsWith(".app/Info.plist"))
    |> first

    match entry
    | None -> "No Info.plist found"
    | Some -> plist...


                    let ape = (0..archive.len())
                        .map(|i| archive.by_index(i).ok())
                        .filter_map(|x| x);


*/
