use tokio::io::AsyncBufReadExt;


#[derive(Default, Debug, serde::Deserialize,
         serde::Serialize, Clone)]
pub struct MazeRunners {
    /// the object of the toml file
    pub runners: Vec<MazeRunner>,
}

#[derive(Default, Debug, serde::Deserialize,
         serde::Serialize, Clone)]
pub struct MazeRunner {
    name: String,
    command: Vec<String> // fancy vec
}

impl MazeRunner {
    /// `run` executes the command of the runner
    /// and returns the output of the command as a
    /// asynchrous stream of bytes until the command
    /// is finished using tokio.
    /// 
    /// # Example
    /// ```rust
    /// use runner::Runner;
    ///
    /// let runner = Runner::default();
    /// runner.run().await?;
    /// ```
    ///
    pub async fn run(&self) -> MazeRunnerResult<()> {
        let mut command = tokio::process::Command::new(&self.command[0]);
        command.args(&self.command[1..]);
        let mut child = command.spawn()?;
        let stdout = child.stdout.take().unwrap();
        let mut reader = tokio::io::BufReader::new(stdout).lines();
        while let Some(line) = reader.next_line().await? {
            println!("{}", line);
        }
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
}

#[derive(Default, Debug)]
pub enum MazeRunnerErrorKind {
    #[default]
    Unplanned,
    Io,
    Toml,
    Std,
    Serde,
}

pub struct MazeRunnerRunError {
    message: String,
}

impl MazeRunnerRunError { }

impl std::fmt::Display for MazeRunnerRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error running command: {}", self.message)
    }
}

#[derive(Default, Debug)]
pub struct MazeRunnerError {
    kind: MazeRunnerErrorKind,
    message: String,
}

impl MazeRunnerError {
    pub fn new(kind: MazeRunnerErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
        }
    }

    pub fn kind(&self) -> &MazeRunnerErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for MazeRunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MazeRunnerError { }

impl From<std::io::Error> for MazeRunnerError {
    fn from(err: std::io::Error) -> Self {
        Self {
            kind: MazeRunnerErrorKind::Io,
            message: format!("IO Error: {}", err),
        }
    }
}

impl From<toml::de::Error> for MazeRunnerError {
    fn from(err: toml::de::Error) -> Self {
        Self {
            kind: MazeRunnerErrorKind::Toml,
            message: format!("TOML Error: {}", err),
        }
    }
}

impl From<String> for MazeRunnerError {
    fn from(err: String) -> Self {
        Self {
            kind: MazeRunnerErrorKind::Std,
            message: format!("MazeRunner Error: {}", err),
        }
    }
}

impl From<serde_json::Error> for MazeRunnerError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            kind: MazeRunnerErrorKind::Serde,
            message: format!("Serde Error: {}", err),
        }
    }
}

pub type MazeRunnerResult<T> = Result<T, MazeRunnerError>;
