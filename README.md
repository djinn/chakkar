# Chakkar

**Chakkar** is a command-line tool written in Rust that optimizes and transforms PNG images with speed and efficiency. The name comes from the Punjabi word "Chakkar" (ਚੱਕਰ), meaning "cycle" or "whirl," symbolizing the transformation and compression of images.

## Why Use Chakkar?

Chakkar is designed for users who need to process and optimize PNG images efficiently. Whether you're preparing images for the web, mobile applications, or other performance-sensitive environments, Chakkar provides several benefits:

- **Fast and Efficient**: Built in Rust, Chakkar takes advantage of Rust's memory safety and performance optimizations, ensuring that image processing is both fast and reliable.
- **Automated Image Transformation**: Chakkar applies several transformations automatically, including RGBA to BGRA conversion and alpha premultiplication. This is particularly useful when preparing images for platforms that require different color formats.
- **Chunked Processing**: For large images, Chakkar supports chunked processing, reducing memory usage by breaking down image operations into smaller pieces.
- **In-Place Image Editing**: Chakkar can overwrite the original image with the transformed version, reducing the need for intermediate files and streamlining workflows.
- **Command-Line Control**: As a CLI tool, Chakkar allows you to integrate image optimization into automated scripts and pipelines, perfect for CI/CD environments.

## Features

- **RGBA to BGRA Conversion**: Automatically swaps color channels and applies alpha premultiplication for images that require it.
- **Chunked Image Processing**: Customize the number of rows processed at a time to optimize memory usage for large images.
- **Verbose Logging**: Increase verbosity to debug the processing pipeline and see detailed information about transformations.
- **Cross-Platform**: Written in Rust, Chakkar runs on Windows, macOS, and Linux.

## Installation

### Prerequisites

- Rust and Cargo installed (instructions [here](https://www.rust-lang.org/tools/install))

To install Chakkar, clone the repository and build the tool using Cargo:

```bash
git clone https://github.com/yourusername/chakkar.git
cd chakkar
cargo build --release
```

The binary will be available in the `target/release` directory.

## Usage

Chakkar is a command-line tool. Here's how to use it:

```bash
chakkar -i input.png -o output.png
```

### Command-Line Options:

- `-i`, `--input <INPUT>`: The input PNG file (required).
- `-o`, `--output <OUTPUT>`: The output PNG file (optional, defaults to `output.png`).
- `-p`, `--inplace`: Overwrite the input file with the transformed version.
- `-v`, `--verbose`: Increase the verbosity of logging. Use multiple times for more detailed logs.
- `-c`, `--chunk_size <ROWS>`: Set the number of rows to process at a time. Default is 8.

### Example Usage

1. **Basic Image Processing:**

   ```bash
   chakkar -i input.png -o output.png
   ```

2. **In-Place Transformation:**

   ```bash
   chakkar -i input.png -p
   ```

3. **Increase Verbosity for Debugging:**

   ```bash
   chakkar -i input.png -o output.png -v
   ```

4. **Process Large Images with Custom Chunk Size:**

   ```bash
   chakkar -i large_image.png -o optimized.png -c 16
   ```

## Performance Considerations

Chakkar uses a chunked processing approach to optimize memory usage. This is particularly helpful when processing large images on machines with limited memory. By adjusting the chunk size, you can balance memory consumption and processing speed.

## Contributions

We welcome contributions! Please feel free to submit issues, bug reports, and pull requests to improve Chakkar.

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Submit a pull request for review.

## License

Chakkar is licensed under the MIT License. See the `LICENSE` file for more details.

---

With **Chakkar**, you get a simple, efficient, and fast PNG processing tool, designed to streamline image optimization and transformations for developers and designers alike.
