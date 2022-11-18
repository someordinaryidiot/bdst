use std::process;

struct Arguments {
    help_s: String,
    help_l: String,
    noformatting_s: String,
    noformatting_l: String,
    files_s: String,
    files_l: String,
    dirs_s: String,
    dirs_l: String,
    size_s: String,
    size_l: String,
}

struct ArgsDone {
    path: String,
    formatting: bool,
    files: bool,
    dirs: bool,
    size: bool,
}

pub fn handle(args: Vec<String>) -> () {
    if args.len() < 2 {
        println!(
            "bdst: arg_handler: handle: Expected at least two arguments, but {} was given!",
            args.len()
        );
        process::exit(-1);
    }

    let args2 = Arguments {
        help_s: String::from("-h"),
        help_l: String::from("--help"),
        noformatting_s: String::from("-nf"),
        noformatting_l: String::from("--no-formatting"),
        files_s: String::from("-f"),
        files_l: String::from("--files"),
        dirs_s: String::from("-d"),
        dirs_l: String::from("--directories"),
        size_s: String::from("-s"),
        size_l: String::from("--size"),
    };

    let mut args_done = ArgsDone {
        path: String::from("/..."),
        formatting: true,
        files: false,
        dirs: false,
        size: false,
    };

    let mut got_path = false;
    let mut counter: u16 = 0;
    for arg in args {
        if counter > 0 {
            if arg == args2.help_s || arg == args2.help_l {
                println!("Help:");
                println!(
                    "    {} {}: shows this help message",
                    args2.help_s, args2.help_l
                );
                println!(
                    "    {} {}: disables formatting for the output (f.e. \'files: 12\' -> \'12\')",
                    args2.noformatting_s, args2.noformatting_l
                );
                println!(
                    "    {} {}: shows the number of files",
                    args2.files_s, args2.files_l
                );
                println!(
                    "    {} {}: shows the number of directories",
                    args2.dirs_s, args2.dirs_l
                );
                println!(
                    "    {} {}: shows the size of the directory",
                    args2.size_s, args2.size_l
                );
                process::exit(0);
            } else if arg == args2.noformatting_s || arg == args2.noformatting_l {
                args_done.formatting = false;
            } else if arg == args2.files_s || arg == args2.files_l {
                args_done.files = true;
            } else if arg == args2.dirs_s || arg == args2.dirs_l {
                args_done.dirs = true;
            } else if arg == args2.size_s || arg == args2.size_l {
                args_done.size = true;
            } else if !arg.contains("-") {
                args_done.path = arg;
                got_path = true;
            } else if !got_path {
                println!(
                    "bdst: arg_handler: handle: Unknown {}th argument `{}`! ",
                    counter, arg
                );
                process::exit(1);
            }
        }
        counter += 1;
    }

    if !got_path {
        println!("bdst: arg_handler: handle: No path specified!");
        process::exit(-2);
    } else {
        let dir_info = crate::dir_size::get_dir_info(args_done.path, false);
        if args_done.formatting {
            if args_done.size {
                print!("size: ");
                print_size(dir_info.size);
            }
            if args_done.files {
                println!("files: {}", dir_info.filecount);
            }
            if args_done.dirs {
                println!("directories: {}", dir_info.dircount);
            }
        } else {
            if args_done.size {
                println!("{}", dir_info.size);
            } else if args_done.files {
                println!("{}", dir_info.filecount);
            } else if args_done.dirs {
                println!("{}", dir_info.dircount);
            }
        }
    }
}

fn print_size(size_dir: u64) -> () {
    if size_dir < 1024 {
        println!("{} b", size_dir);
    } else if size_dir < 1024 * 1024 {
        println!("{} kb", size_dir / 1024);
    } else if size_dir < 1024 * 1024 * 1024 {
        println!("{} mb", size_dir / 1024 / 1024)
    } else {
        println!("{} gb", size_dir / 1024 / 1024 / 1024);
    }
    return;
}
