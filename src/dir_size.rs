use walkdir::WalkDir;

pub struct DirInfo {
    pub size: u64,
    pub filecount: u64,
    pub dircount: u64,
}

pub fn get_dir_info(directory: String, debug_output: bool) -> DirInfo {
    let mut tsize = 0;
    let mut file_count = 0;
    let mut dir_count = 0;

    for file in WalkDir::new(directory.clone())
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            tsize += file.metadata().unwrap().len();
            file_count += 1;
            if debug_output {
                println!(
                    "{}:\n    {}",
                    file.path().display(),
                    file.metadata().unwrap().len()
                );
            }
        } else if file.metadata().unwrap().is_dir() {
            dir_count += 1;
            tsize += file.file_name().len() as u64;
        }
    }

    let out_final = DirInfo {
        size: tsize,
        filecount: file_count,
        dircount: dir_count,
    };

    return out_final;
}
