# libmic-rs

A simple, cross-platform Rust crate for recording audio from microphones to WAV files.

## Features

- 🎤 **Simple API** - Record audio with just a few lines of code
- 🔊 **Multiple Sample Formats** - Supports F32, I16, I32, and I8 sample formats
- 🖥️ **Cross-Platform** - Works on Windows, macOS, and Linux
- 📁 **WAV Output** - Records directly to WAV files using the `hound` crate
- ⚡ **Low Latency** - Built on top of `cpal` for efficient audio handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
libmic-rs = "0.1.0"
```

## Quick Start

```rust
use libmic_rs::mic::Recorder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Record 5 seconds of audio to "recording.wav"
    Recorder::record_to_file("recording.wav", 5)?;
    println!("Recording saved to recording.wav");
    Ok(())
}
```

## Usage

### Basic Recording

The simplest way to record audio is using the `record_to_file` method:

```rust
use libmic_rs::mic::Recorder;

// Record 10 seconds of audio
Recorder::record_to_file("my_recording.wav", 10)?;
```

### Error Handling

The crate provides custom error types for better error handling:

```rust
use libmic_rs::{mic::Recorder, error::MicError};

match Recorder::record_to_file("recording.wav", 5) {
    Ok(()) => println!("Recording completed successfully!"),
    Err(MicError::DeviceNotFound) => eprintln!("No microphone found"),
    Err(MicError::Other(msg)) => eprintln!("Recording error: {}", msg),
}
```

## API Reference

### `Recorder`

The main struct for audio recording operations.

#### Methods

##### `record_to_file(path: &str, duration_sec: u64) -> Result<(), MicError>`

Records audio from the default input device to a WAV file.

**Parameters:**
- `path`: File path where the recording will be saved
- `duration_sec`: Recording duration in seconds

**Returns:**
- `Ok(())` on successful recording
- `Err(MicError)` on failure

**Example:**
```rust
Recorder::record_to_file("output.wav", 30)?; // Record for 30 seconds
```

### Error Types

#### `MicError`

Custom error type for audio recording operations.

**Variants:**
- `DeviceNotFound` - No audio input device available
- `Other(String)` - Other errors with descriptive messages

## Supported Audio Formats

The crate automatically detects and supports the following sample formats:

| Format | Description | Bit Depth |
|--------|-------------|-----------|
| `F32` | 32-bit floating point | 32-bit |
| `I16` | 16-bit signed integer | 16-bit |
| `I32` | 32-bit signed integer | 32-bit |
| `I8` | 8-bit signed integer | 8-bit |

The output WAV file format is automatically configured based on your system's default audio format.

## Requirements

### System Requirements

- **Windows**: Windows 7 or later
- **macOS**: macOS 10.7 or later  
- **Linux**: ALSA or PulseAudio

### Dependencies

This crate depends on:
- [`cpal`](https://crates.io/crates/cpal) ^0.15 - Cross-platform audio I/O
- [`hound`](https://crates.io/crates/hound) ^3.5 - WAV file encoding/decoding

## Module Structure

The crate is organized into the following modules:

- `mic` - Core recording functionality with the `Recorder` struct
- `error` - Custom error types for audio operations

## Troubleshooting

### Common Issues

**"No microphone found" error:**
- Ensure your microphone is connected and recognized by your operating system
- Check your system's audio input settings
- Try running with administrator/root privileges if needed

**Permission errors:**
- On some systems, microphone access requires special permissions
- Make sure your application has microphone access permissions

**Audio quality issues:**
- The crate uses your system's default audio format
- For specific format requirements, you may need to configure your system's audio settings

## Roadmap

This crate is a work in progress. Planned features include:

- [ ] Custom audio format configuration
- [ ] Real-time audio processing callbacks
- [ ] Multiple output formats (MP3, FLAC, etc.)
- [ ] Audio device selection
- [ ] Volume level monitoring
- [ ] Streaming support
- [ ] Audio filtering and effects

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Clone the repository
2. Install Rust (if not already installed)
3. Run tests: `cargo test`
4. Run examples: `cargo run --example recording`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [`cpal`](https://github.com/RustAudio/cpal) for cross-platform audio support
- Uses [`hound`](https://github.com/ruuda/hound) for WAV file handling

---

**Note:** This crate is currently in active development. The API may change in future versions.
