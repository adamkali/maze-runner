# Maze Runner

Maze Runner is a for running any executable command as from simple name listed in a configuration file. It is intended to be used as a tool for neovim, but can be used as a standalone tool as well. you can think of it as npm run but for any executable command.

## Getting Started 

### Installation

You can simply use
```bash
cargo install maze-runner
```

or you can clone the repo and build it yourself
```bash
git clone https://github.com/adamkali/maze-runner.git
cd maze-runner
cargo build --release
```
and then copy the binary to your path.

### Usage

Maze runner is intended to be used as a tool for neovim, so there is really only two commands
```bash
mr run <command>
mr list
```

### Configuration
 
Below is an example `runner.toml` file. The file must be named `runner.toml` and must be in the same directory as the executable you are running. The executable must be in your path. 
```toml
[[runners]]
name = "run"
command = [
    'cargo',
    'run',
]

[[runners]]
name = "test"
command = [
    'cargo',
    'test',
]

[[runners]]
name = "build"
command = [
    'cargo',
    'build',
    '--release',
]
```
however, you can also specify a path to the executable
```toml
[[runners]]
name = "run"
command = [
    'cargo',
    'run',
]
...
```
by running `mr run -p <path>` or `mr run --path <path>`. 

as a consequence of this, you can also specify a `dev.runner.toml` file in your project directory and then you can run `mr run -d` or `mr run --dev` to use the `dev.runner.toml` file instead of the `runner.toml` file. There is also a `test.runner.toml` file that can be used with `mr run -t` or `mr run --test`. Finally there is a `release.runner.toml` file that can be ran with `mr run -r`.

For more information: use `mr --help` or `mr <command> --help`.

## Contributing

If you would like to contribute, please open an issue or a pull request.

