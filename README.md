# DEFLATE Algorithm Implementation

This project is an implementation of the DEFLATE algorithm as part of the Rust programming course at MIMUW. The DEFLATE algorithm combines LZ77 compression and Huffman coding, and is widely used in formats like PNG, ZIP, and gzip.

## Features

- **LZ77 Compression**: Efficient sliding window compression is already implemented.
- **Huffman Coding**: Planned for future development to complete the DEFLATE algorithm.
- **Command-Line Interface**: Flexible control over parameters like window size, block length, and pre-defined codes.
- **Multithreading** (Coming Soon): Enhancements for performance optimization.

---

## Usage

To run the program, use the following command:

```sh
cargo run -- [OPTIONS]
```

### Options

- **`-window_size <VALUE>`**: Specifies the size of the sliding window for LZ77 compression.
  - Example: `-window_size 1000`

- **`-max_len_of_block <VALUE>`**: Sets the maximum length of a block for compression.
  - Example: `-max_len_of_block 20`

- **`-codes_predef <0|1>`**: Toggles the use of predefined codes for Huffman coding.
  - Example: `-codes_predef 1`

After running `cargo run`, an explanation of the available options and usage will be displayed in the terminal.

---

## Example

Here is an example command:

```sh
cargo run -- -window_size 1000 -max_len_of_block 20 -codes_predef 1
```

This runs the program with:
- Sliding window size of 1000 bytes
- Maximum block length of 20 bytes
- Predefined Huffman codes enabled

---

## Progress

### Completed:
- Implementation of the LZ77 compression algorithm

### Upcoming:
- **Huffman Coding**: To be added soon to enable full DEFLATE functionality.
- **Multithreading**: To improve performance by parallelizing tasks.

---

## Development Setup

### Prerequisites
Ensure you have the following installed:
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **Cargo**: Comes bundled with Rust.

### Running the Project

1. Clone the repository:
   ```sh
   git clone <repository_url>
   cd <repository_name>
   ```

2. Build and run the project:
   ```sh
   cargo run -- [OPTIONS]
   ```

3. Run tests:
   ```sh
   cargo test
   ```

---

## Contributing

Contributions are welcome! If you have ideas or improvements, feel free to open an issue or submit a pull request.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Acknowledgments

This project is part of the Rust programming course at MIMUW. Special thanks to the course instructors and TAs for their guidance.

