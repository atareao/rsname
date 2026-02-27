# 🎬 RSName - Intelligent File Renamer

[![Version](https://img.shields.io/badge/version-0.1.9-blue.svg)]()
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/rust--edition-2024-red.svg)]()
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)]()
[![TUI](https://img.shields.io/badge/interface-Terminal%20UI-purple.svg)]()

> 🚀 A powerful, intelligent terminal-based file renaming tool specifically designed for organizing media collections (movies, TV shows, documentaries) with automatic video file detection.

## ✨ Features

- 🎯 **Smart Video Detection**: Automatically selects only video files (mkv, avi, mp4, mov, wmv, etc.) by default
- 🧠 **Intelligent Name Cleaning**: Advanced regex-based algorithm that:
  - Removes technical garbage (resolutions, codecs, release groups)
  - Extracts and formats season/episode information (S01E01 format)
  - Properly handles years and movie titles
  - Cleans up brackets, parentheses, and unwanted text
- 🖥️ **Beautiful TUI Interface**: Modern terminal user interface built with Ratatui
- ⚡ **Real-time Preview**: See exactly how files will be renamed before applying changes
- 🎛️ **Selective Renaming**: Toggle individual files on/off before renaming
- 📁 **Safe Operations**: Preview-first approach prevents accidental renames
- 🔄 **Live Scanning**: Rescan directory without restarting the application

## 🎥 Supported Video Formats

| Format        | Extensions                 |
| ------------- | -------------------------- |
| **Common**    | mkv, avi, mp4, mov, wmv    |
| **Web**       | webm, flv, m4v             |
| **Legacy**    | mpg, mpeg, 3gp, rmvb, divx |
| **Broadcast** | ts, mts, m2ts, vob         |
| **Other**     | ogv, dv, asf               |

## 🛠️ Installation

### Prerequisites

- Rust 1.70+ with 2024 edition support
- Terminal with Unicode support

### From Source

```bash
git clone https://github.com/atareao/rsname.git
cd rsname
cargo build --release
cargo install --path .
```

### Quick Install

```bash
cargo install rsname
```

## 🚀 Quick Start

1. **Navigate to your media directory**:

   ```bash
   cd /path/to/your/media/files
   ```

2. **Run RSName**:

   ```bash
   rsname
   ```

3. **Use the interface**:
   - `↑/↓` or `j/k`: Navigate through files
   - `Space`: Toggle file selection
   - `Enter`: Confirm and rename selected files
   - `r`: Rescan current directory
   - `q`: Quit application

## 📖 Usage Examples

### Before and After Examples

| Original Filename                             | Cleaned Result                  |
| --------------------------------------------- | ------------------------------- |
| `Entrepreneurs [HDTV 720p][Cap.110].mkv`      | `Entrepreneurs - S01E10.mkv`    |
| `The.Simpsons.3x12.avi`                       | `The Simpsons - S03E12.avi`     |
| `Pulp_Fiction_1994_720p_Bluray.mp4`           | `Pulp Fiction (1994).mp4`       |
| `Better.Call.Saul.Capitulo.215.720p.h264.mp4` | `Better Call Saul - S02E15.mp4` |
| `[SubsCastellano] One Piece Cap 1045.mkv`     | `One Piece - S10E45.mkv`        |

### Advanced Pattern Recognition

The tool intelligently handles various naming conventions:

- **Season/Episode**: `S01E01`, `1x01`, `Cap.110`, `Capitulo 215`, `Ep 01`
- **Years**: Extracts and positions movie release years correctly
- **Technical Cleanup**: Removes resolution, codecs, release groups, watermarks
- **Language Tags**: Cleans up language and subtitle indicators
- **Proper Capitalization**: Applies title case formatting

## 🎮 Interface Controls

### Navigation Mode

| Key       | Action                  |
| --------- | ----------------------- |
| `↑` / `k` | Move selection up       |
| `↓` / `j` | Move selection down     |
| `Space`   | Toggle file selection   |
| `Enter`   | Proceed to confirmation |
| `r`       | Rescan directory        |
| `q`       | Quit application        |

### Confirmation Mode

| Key                 | Action                         |
| ------------------- | ------------------------------ |
| `y` / `Y` / `Enter` | Execute rename operation       |
| `n` / `N` / `Esc`   | Cancel and return to selection |

## 🏗️ Architecture

RSName is built with a modular architecture:

```
src/
├── main.rs              # TUI interface and app loop
├── models/
│   ├── mod.rs          # Module exports
│   ├── app.rs          # Application state and logic
│   ├── app_mode.rs     # Application modes
│   ├── cleaner.rs      # Name cleaning algorithms
│   └── rename_item.rs  # File item representation
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test cleaner::tests
```

## 📦 Dependencies

- **crossterm** (0.29.0): Cross-platform terminal manipulation
- **ratatui** (0.30.0): Terminal user interface framework
- **regex** (1.12.3): Regular expression engine for name cleaning
- **walkdir** (2.5.0): Directory traversal

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

```bash
git clone https://github.com/atareao/rsname.git
cd rsname
cargo build
cargo test
```

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Run clippy for linting (`cargo clippy`)
- Add tests for new features
- Update documentation for API changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ in Rust
- Inspired by the need for better media file organization
- Thanks to the Rust community for excellent crates

---

<div align="center">

**[⭐ Star this repo if you find it helpful!](https://github.com/atareao/rsname)**

Made with 💻 and ☕ by [Lorenzo Carbonell](https://github.com/atareao)

</div>
