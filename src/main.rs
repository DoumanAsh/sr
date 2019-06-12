use std::{io, path, fs, env};
use std::io::{BufRead, Read, Write};

mod cli;

const NEWLINE: &[u8] = &[10];

fn run_from_stdin(args: cli::Args) -> Result<(), i32> {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = io::BufWriter::new(stdout);
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = io::BufReader::new(stdin);

    for line in stdin.lines() {
        match line {
            Ok(line) => {
                let _ = stdout.write_all(args.search.replace_all(&line, args.replace.as_str()).as_bytes());
                let _ = stdout.write_all(NEWLINE);
            },
            Err(error) => {
                if !args.silent {
                    eprintln!("Error reading stdin: {}", error);
                }
                return Err(1);
            }
        }
    }

    let _ = stdout.flush();

    Ok(())
}

fn run_from_files(args: cli::Args) -> Result<(), i32> {
    const MMAP_THRESHOLD: u64 = 16 * 1024;
    let mut result = Ok(());

    for file in args.files {
        let mut input = match fs::File::open(&file) {
            Ok(input) => input,
            Err(error) => {
                if !args.silent {
                    eprintln!("{}: Unable to open. Error {}", file, error);
                }
                result = Err(1);
                continue;
            }
        };

        let metadata = match input.metadata() {
            Ok(metadata) => metadata,
            Err(error) => {
                if !args.silent {
                    eprintln!("{}: Unable to read. Error {}", file, error);
                }
                result = Err(1);
                continue;
            }
        };

        match metadata.len() > MMAP_THRESHOLD {
            true => match unsafe { memmap::Mmap::map(&input) } {
                Ok(input_mmap) => {
                    let input_data = match std::str::from_utf8(&input_mmap[..]) {
                        Ok(input) => input,
                        Err(error) => {
                            if !args.silent {
                                eprintln!("{}: Unable to read. Error {}", file, error);
                            }
                            result = Err(1);
                            continue;
                        }
                    };

                    if !args.search.is_match(&input_data) {
                        continue;
                    }

                    let tmp_dir = match path::Path::new(&file).parent() {
                        Some(tmp_dir) => tmp_dir,
                        None => {
                            if !args.silent {
                                eprintln!("{}: Unable to get parent dir", file);
                            }
                            result = Err(1);
                            continue;
                        }
                    };

                    let dest = match tempfile::NamedTempFile::new_in(&tmp_dir) {
                        Ok(dest) => dest,
                        Err(error) => {
                            if !args.silent {
                                eprintln!("{}: Unable to create tmp file. Error {}", tmp_dir.display(), error);
                            }
                            result = Err(1);
                            continue;
                        }
                    };

                    {
                        let dest_file = dest.as_file();
                        let _ = dest_file.set_permissions(metadata.permissions());

                        let mut writer = io::BufWriter::new(dest_file);

                        for line in input_data.lines() {
                            match writer.write_all(args.search.replace_all(&line, args.replace.as_str()).as_bytes()).and_then(|_| writer.write_all(NEWLINE)) {
                                Ok(_) => (),
                                Err(error) => {
                                    if !args.silent {
                                        eprintln!("{}: Unable to write. Error {}", file, error);
                                    }
                                    result = Err(1);
                                    continue;
                                }
                            }
                        }

                        match writer.flush() {
                            Ok(_) => (),
                            Err(error) => {
                                if !args.silent {
                                    eprintln!("{}: Unable to write. Error {}", file, error);
                                }
                                result = Err(1);
                                continue;
                            }
                        }
                    }

                    drop(input_mmap);
                    drop(input);

                    match args.suffix.as_ref() {
                        Some(suffix) => {
                            let backup = format!("{}{}", file, suffix);
                            match fs::rename(&file, &backup) {
                                Ok(_) => (),
                                Err(error) => {
                                    if !args.silent {
                                        eprintln!("{}: Unable to create backup. Error {}", backup, error);
                                    }
                                    result = Err(1);
                                    continue;
                                }
                            }
                        }
                        None => (),
                    };

                    match dest.persist(&file) {
                        Ok(_) => (),
                        Err(error) => {
                            if !args.silent {
                                eprintln!("{}: {}", file, error);
                            }
                            result = Err(1);
                            continue;
                        }
                    }
                },
                Err(error) => {
                    if !args.silent {
                        eprintln!("{}: Unable to read. Error {}", file, error);
                    }
                    result = Err(1);
                    continue;
                }
            },
            false => {
                let mut input_data = String::with_capacity(metadata.len() as usize);
                match input.read_to_string(&mut input_data) {
                    Ok(_) => (),
                    Err(error) => {
                        if !args.silent {
                            eprintln!("{}: Unable to read. Error {}", file, error);
                        }
                        result = Err(1);
                        continue;
                    }
                }

                if !args.search.is_match(&input_data) {
                    continue;
                }

                drop(input);

                match args.suffix.as_ref() {
                    Some(suffix) => {
                        let backup = format!("{}{}", file, suffix);
                        match fs::rename(&file, &backup) {
                            Ok(_) => (),
                            Err(error) => {
                                if !args.silent {
                                    eprintln!("{}: Unable to create backup. Error {}", backup, error);
                                }
                                result = Err(1);
                                continue;
                            }
                        }
                    }
                    None => (),
                };

                let mut dest = match fs::File::create(&file) {
                    Ok(dest) => dest,
                    Err(error) => {
                        if !args.silent {
                            eprintln!("{}: Unable to create. Error {}", file, error);
                        }
                        result = Err(1);
                        continue;
                    }
                };

                let _ = dest.set_permissions(metadata.permissions());

                let input_data = args.search.replace_all(&input_data, args.replace.as_str());
                match dest.write_all(input_data.as_bytes()).and_then(|_| dest.flush()) {
                    Ok(_) => (),
                    Err(error) => {
                        if !args.silent {
                            eprintln!("{}: Unable to write. Error {}", file, error);
                        }
                        result = Err(1);
                        continue;
                    }
                }
            }
        }
    }

    result
}

fn run() -> Result<(), i32> {
    let args = cli::Args::new(env::args().skip(1))?;

    match args.files.len() == 0 {
        true => run_from_stdin(args),
        false => run_from_files(args),
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(exit_code) => std::process::exit(exit_code)
    }
}
