use std::{collections::HashMap, fmt::Display};

use clap::{Parser, Subcommand};
use serde::Deserialize;

const DB_PATH: &str = "db/2022-11-19-020018/data/crates.csv";

time::serde::format_description!(
    time_format,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
);
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Crate {
    #[serde(with = "time_format")]
    created_at: time::PrimitiveDateTime,
    // created_at: String,
    description: Option<String>,
    documentation: Option<String>,
    downloads: i32,
    homepage: Option<String>,
    id: i32,
    max_upload_size: Option<i32>,
    name: String,
    readme: String,
    repository: Option<String>,
    // #[serde(with = "time_format")]
    // updated_at: time::PrimitiveDateTime,
    updated_at: String,
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            created_at,
            description,
            documentation,
            downloads,
            homepage,
            id: _,
            max_upload_size,
            name,
            readme,
            repository,
            updated_at,
        } = self;
        writeln!(f, "Name              : {name}")?;
        if let Some(description) = description {
            writeln!(f, "Description       : {}", description)?;
        }
        writeln!(f, "Version           : {:?}", ())?;
        if let Some(desc) = description {
            writeln!(f, "Description       : {desc}")?;
        }
        if let Some(homepage) = homepage {
            writeln!(f, "Homepage          : {}", homepage)?;
        }
        if let Some(documentation) = documentation {
            writeln!(f, "Documentation     : {}", documentation)?;
        }
        if let Some(repository) = repository {
            writeln!(f, "Repository        : {}", repository)?;
        }
        writeln!(f, "Licenses          : {:?}", ())?;
        writeln!(f, "Dependencies      : {:?}", &[()])?;
        writeln!(f, "Dev. dependencies : {:?}", &[()])?;
        writeln!(f, "Authors           : {:?}", &[()])?;
        writeln!(f, "Downloads         : {}", downloads)?;
        if let Some(max_upload_size) = max_upload_size {
            writeln!(f, "Max upload size   : {}", max_upload_size)?;
        }
        writeln!(f, "Created at        : {}", created_at)?;
        writeln!(f, "UpdatedAt         : {}", updated_at)?;
        // writeln!(f, "Readme : {}", readme)?;
        bat::PrettyPrinter::new()
            .input_from_bytes(readme.as_bytes())
            .language("markdown")
            .grid(false)
            .header(false)
            .line_numbers(false)
            .print()
            .unwrap();

        Ok(())
    }
}

#[derive(Debug, Clone, Parser)]
#[command(
    subcommand_required = true,
    arg_required_else_help = true,
    disable_version_flag = true,
    disable_help_subcommand = true
)]
struct Args {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Debug, Clone, Subcommand)]
enum Subcommands {
    /// Query the package database.
    #[command(short_flag = 'Q', long_flag = "query")]
    Query(Query),
    /// Remove a package from the manifest.
    #[command(short_flag = 'R', long_flag = "remove")]
    Remove(Remove),
    /// Synchronize local package database.
    #[command(short_flag = 'S', long_flag = "sync")]
    Sync(Sync),
    /// Upgrade a package.
    #[command(short_flag = 'U', long_flag = "upgrade")]
    Upgrade(Upgrade),
    /// Display version and exit.
    #[command(short_flag = 'V', long_flag = "version")]
    Version,
}

#[derive(Debug, Clone, clap::Args)]
struct Query {
    /// Package(s).
    #[arg(required_unless_present = "search")]
    packages: Vec<String>,

    /// View package information.
    #[arg(short = 'i', long = "info", conflicts_with = "search")]
    info: bool,
    /// Show less information for query and search.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
    /// Search locally-installed packages for matching strings.
    #[arg(
        short = 's',
        long = "search",
        value_name = "regex",
        conflicts_with = "info"
    )]
    search: Option<String>,
    /// Be verbose.
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Debug, Clone, clap::Args)]
struct Remove {
    /// Package(s).
    #[arg(required = true)]
    packages: Vec<String>,

    /// Be verbose.
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Debug, Clone, clap::Args)]
struct Sync {
    /// Package(s).
    #[arg(required_unless_present_any = ["search", "list"])]
    packages: Vec<String>,

    /// View package information.
    #[arg(short = 'i', long = "info", conflicts_with = "search")]
    info: bool,
    /// List all packages in the index.
    #[arg(short = 'l', long = "list", conflicts_with = "search")]
    list: bool,
    /// Show less information for query and search.
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
    /// Search locally-installed packages for matching strings.
    #[arg(
        short = 's',
        long = "search",
        value_name = "regex",
        conflicts_with_all = ["info", "list"]
    )]
    search: Option<String>,
    /// Download fresh package database from the server.
    ///
    /// (-yy to force a refresh even if up to date)
    #[arg(short = 'y', long = "refresh")]
    refresh: bool,
    /// Be verbose.
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Debug, Clone, clap::Args)]
struct Upgrade {
    /// Package(s).
    #[arg(required = true)]
    packages: Vec<String>,

    /// Be verbose.
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    match args.command {
        Subcommands::Query(q) => query(q),
        Subcommands::Remove(r) => remove(r),
        Subcommands::Sync(s) => sync(s)?,
        Subcommands::Upgrade(u) => upgrade(u),
        Subcommands::Version => println!("version"),
    };

    Ok(())
}

fn query(q: Query) {
    dbg!(&q);
}

fn remove(r: Remove) {
    dbg!(&r);
}

fn sync(
    Sync {
        packages,
        info,
        list,
        quiet,
        search,
        refresh,
        verbose,
    }: Sync,
) -> color_eyre::Result<()> {
    match (list, info, packages, search) {
        (true, false, _, None) => {
            let mut rdr = csv::Reader::from_path(DB_PATH)?;
            for res in rdr.deserialize() {
                let krate: Crate = res?;
                match quiet {
                    true => println!("{}", krate.name),
                    false => todo!(),
                }
            }
        }
        (_, _, _, Some(re)) => {
            let re = regex::Regex::new(&re)?;
            let mut rdr = csv::Reader::from_path(DB_PATH)?;
            for res in rdr.deserialize() {
                let krate: Crate = res?;
                if re.is_match(&krate.name)
                    || krate
                        .description
                        .as_deref()
                        .map(|desc| re.is_match(desc))
                        .unwrap_or_default()
                {
                    match quiet {
                        true => println!("{}", krate.name),
                        false => todo!(),
                    }
                }
            }
        }
        (false, true, packages, None) => {
            let mut rdr = csv::Reader::from_path(DB_PATH)?;
            let crates: HashMap<_, _> = rdr
                .deserialize()
                .into_iter()
                .flat_map(
                    |res: csv::Result<Crate>| -> color_eyre::Result<(String, Crate)> {
                        let krate: Crate = res?;
                        Result::Ok((krate.name.clone(), krate))
                    },
                )
                .collect();
            for ele in packages {
                if let Some(krate) = crates.get(&ele) {
                    match quiet {
                        true => println!("{}", krate.name),
                        false => println!("{krate}"),
                    }
                }
            }
        }
        _ => todo!(),
    }

    Ok(())
}

fn upgrade(u: Upgrade) {
    dbg!(&u);
}
