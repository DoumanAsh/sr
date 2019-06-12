const USAGE: &str = "Search and replace

USAGE:
    sr [options] <pattern> <replace> [file]...

OPTIONS:
    -i, --in-place [SUFFIX] - Modifies files in place. If SUFFIX is specified creates backup with it.
    -q, --quiet             - Specifies silent mode. Default false.
    -h, --help              - Prints this help message.

ARGS:
    <pattern> - Specifies regex to look for.
    <replace> - Specifies expression to replace with. captured values like $1 are allowed
    [file]... - Optionally specifies list of files. If omitted reads from STDIN.
";

#[derive(Debug)]
pub struct Args {
    pub search: regex::Regex,
    pub replace: String,
    pub files: Vec<String>,
    pub suffix: Option<String>,
    pub silent: bool,
}

impl Args {
    pub fn new<A: Iterator<Item=String>>(mut args: A) -> Result<Self, i32> {
        let mut search = None;
        let mut replace = None;
        let mut suffix = None;
        let mut files = Vec::with_capacity(0);
        let mut silent = false;

        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                match &arg[1..]{
                    "q" | "-quiet" => silent = true,
                    "i" | "-in-place" => match args.next() {
                        Some(new_suffix) => {
                            suffix = Some(new_suffix);
                        },
                        None => {
                            eprintln!("Flag {} is specified, but suffix is missing", arg);
                            return Err(2);
                        }
                    },
                    "h" | "-help" => {
                        println!("{}", USAGE);
                        return Err(0);
                    },
                    _ => {
                        eprintln!("Invalid flag '{}' is specified", arg);
                        println!("{}", USAGE);
                        return Err(2);
                    }
                }
            } else if search.is_none() {
                match regex::Regex::new(&arg) {
                    Ok(pattern) => {
                        search = Some(pattern)
                    },
                    Err(error) => {
                        eprintln!("Unable to compile '{}' into regex expression. Error: {}", arg, error);
                        return Err(2);
                    }
                }
            } else if replace.is_none() {
                replace = Some(arg);
            } else {
                files.push(arg)
            }
        }

        let search = match search {
            Some(search) => search,
            None => {
                eprintln!("Missing <search> pattern");
                println!("{}", USAGE);
                return Err(2);
            }
        };

        let replace = match replace {
            Some(replace) => replace,
            None => {
                eprintln!("Missing <replace> pattern");
                println!("{}", USAGE);
                return Err(2);
            }
        };

        Ok(Self {
            search,
            replace,
            files,
            suffix,
            silent,
        })
    }
}
