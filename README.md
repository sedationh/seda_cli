# seda CLI

`seda` is SedationH's CLI toolkit

## Features

- `code`: Quickly clone a GitHub repository and open it in VSCode.

#### Prerequisites

- Rust and Cargo (Install from [https://www.rust-lang.org/](https://www.rust-lang.org/))
- Git
- Visual Studio Code

#### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/SedationH/seda_cli.git
   cd seda_cli
   ```

2. Build and install:
   ```bash
   cargo install --path .
   ```

## Usage

### `seda code`

Clone a GitHub repository and open it in VSCode:

```bash
seda code <repository_url> [new_name]
```

- `<repository_url>`: The URL of the GitHub repository you want to clone.
- `[new_name]`: (Optional) A new name for the cloned directory.

Example:
```bash
seda code https://github.com/example/repo.git my-project
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

SedationH