use std::fs;

pub fn check_dir_format_right(fpath: &str) -> Result<bool, String> {
    let expected_entries = vec!["Caves", "Master", "cluster.ini"]; // 期望的文件和文件夹名称

    let mut found_entries = Vec::new();

    match fs::read_dir(fpath) {
        Ok(entries) => {
            for entry in entries.filter_map(|entry| entry.ok()) {
                if let Some(name) = entry.file_name().to_str() {
                    found_entries.push(name.to_owned());
                }
            }

            let mut missing_entries = Vec::new();
            for expected_entry in expected_entries.iter() {
                if !found_entries.contains(&expected_entry.to_string()) {
                    missing_entries.push(expected_entry);
                }
                log::info!("look {expected_entry}");
            }

            if missing_entries.is_empty() && found_entries.len() == expected_entries.len() {
                // println!("All expected entries found in the directory.");
            } else {
                println!("Missing entries:");
                for entry in missing_entries {
                    println!("{}", entry);
                }
                return Ok(false);
            }
        }
        Err(e) => {
            log::error!("something err: {e}");
            return Err(format!("something err: {}", e));
        }
    }
    Ok(true)
}
