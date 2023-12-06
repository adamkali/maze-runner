mod structs;
use std::{path::PathBuf, fs::File, io::Read};

use structs::{MazeRunner, MazeRunnerResult, MazeRunnerError};
use clap::{Parser, Subcommand};

// use clap to create a command line interface
#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Opts {
    /// The command to run
    #[command(subcommand)]
    command: SubCommand,
}

// Define a subcommand for the actual command to be executed
#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Run a specific command
    #[command(name = "run")]
    Run {
        /// The name of the command to run
        name: String,
        /// flag -p, --Path to the toml file. cannot be used with -d, -t, or -r
        #[arg(short = 'p')]
        path: Option<PathBuf>,
        /// flag -d, --Develepment for dev.runner.toml
        #[arg(short = 'd')]
        dev: bool,
        /// flag -t, --Test for test.runner.toml
        #[arg(short = 't')]
        test: bool,
        /// flag -r, --Release for release.runner.toml
        #[arg(short = 'r')]
        release: bool,
    },
    /// list gets a list of all the runners in json format
    #[command(name = "list")]
    List {
        /// flag -p, --Path to the toml file. cannot be used with -d, -t, or -r
        #[arg(short = 'p')]
        path: Option<PathBuf>,
        /// flag -d, --Develepment for dev.runner.toml
        #[arg(short = 'd')]
        dev: bool,
        /// flag -t, --Test for test.runner.toml
        #[arg(short = 't')]
        test: bool,
        /// flag -r, --Release for release.runner.toml
        #[arg(short = 'r')]
        release: bool,
    },
}

#[tokio::main]
async fn main() -> MazeRunnerResult<()> {
    let opts: Opts = Opts::parse();
    match opts.command {
        SubCommand::Run { name, path, dev, test, release } => {
            let runners = load_runners(dev, test, release, path)?;
            run_by_name(&name, runners).await?;
        },
        SubCommand::List { path, dev, test, release } => {
            let runners = load_runners(dev, test, release, path)?;
            list_runners(runners)?;
        }
    }
    Ok(())
}

/// `load_runners` loads a toml file from the given path
/// and returns a vector of `MazeRunner` structs.
fn load_runners(dev: bool, test: bool, release: bool, path: Option<PathBuf>) -> MazeRunnerResult<Vec<MazeRunner>> {
    match (dev, test, release, path) {
        (true, _, _, _) => load_runners_from_toml(&PathBuf::from("dev.runner.toml")),
        (_, true, _, _) => load_runners_from_toml(&PathBuf::from("test.runner.toml")),
        (_, _, true, _) => load_runners_from_toml(&PathBuf::from("release.runner.toml")),
        (_, _, _, Some(p)) => load_runners_from_toml(&p),
        _ => load_runners_from_toml(&PathBuf::from("runner.toml")),
    }
}

/// `load_runners_from_toml` loads a toml file from the given path
/// and returns a vector of `MazeRunner` structs.
///
/// load_runners_from_toml is called when the user runs `$ mr run <name>` from the command line.
/// but us never called directly.
/// 
/// # Arguments
/// * `path` - A `PathBuf` that points to the toml file
///
/// # Example
/// ```rust
/// use runner::load_runners_from_toml;
///
/// let runners = load_runners_from_toml("runners.toml");
/// ```
///
/// # Errors
/// This function returns an error if the file cannot be opened
/// or if the file cannot be read.
fn load_runners_from_toml(path: &PathBuf) -> MazeRunnerResult<Vec<MazeRunner>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let maze_runners: structs::MazeRunners = toml::from_str(&contents)?;
    Ok(maze_runners.runners)
}

/// `run_by_name` runs a runner by name from the given bank of runners.
///
/// # Arguments
/// * `name` - A `&str` that is the name of the runner to run
/// * `runners` - A `Vec<MazeRunner>` that is the bank of runners
///
/// # CLI 
/// ```bash
/// mr run <name>
/// ```
///
/// # Example
/// ```rust
/// use runner::{run_by_name, load_runners_from_toml};
/// use std::path::PathBuf;
/// use std::sync::Arc;
///
/// let runners = load_runners_from_toml(PathBuf::from("runners.toml"));
/// let bank = Arc::new(runners);
/// run_by_name("runner1", bank);
/// ```
/// 
/// # Errors
/// This function returns an error if the runner cannot be found
/// or if the runner cannot be run.
///
/// # Notes
/// This function is async and must be awaited.
async fn run_by_name(name: &str, runners: Vec<MazeRunner>) -> MazeRunnerResult<()> {
    if let Some(runner) = runners.iter().find(|r| r.name() == name) {
        runner.run().await?;
    } else {
        Err(MazeRunnerError::new(structs::MazeRunnerErrorKind::Unplanned, "Runner not found".to_string()))?;
    }
    Ok(())
}

/// `list_runners` lists all the runners in the given toml file.
/// This returns a json string of the runners in stdout.
///
/// # Arguments
/// * `runners` - A `Vec<MazeRunner>` that is the list of runners
///
/// # Example
/// ```rust
/// use runner::{list_runners, load_runners_from_toml};
/// use std::path::PathBuf;
/// 
/// let runners = load_runners_from_toml(PathBuf::from("runners.toml"));
/// list_runners(runners);
/// ```
/// 
/// # CLI 
/// ```bash
/// mr list
/// ```
/// or
/// ```bash
/// mr list -p runners.toml
/// ```
///
/// # Errors
/// This function returns an error if the toml file cannot be opened
/// or if the toml file cannot be read.
pub fn list_runners(runners: Vec<MazeRunner>) -> MazeRunnerResult<()> {
    let runners = serde_json::to_string(&runners)?;
    println!("{}", runners);
    Ok(())
}
