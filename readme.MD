# Rust File Downloader

This Rust project demonstrates how to download a file from a given URL and save it to a temporary directory. This program uses asynchronous operations for making the HTTP request and writing the response to the file system. It also provides error handling for the IO operations and HTTP requests by using `anyhow` and `thiserror` crates.

## Functionality

The program creates a new HTTP request to download an image file from `https://www.rust-lang.org/static/images/rust-logo-blk.png`. Once the file is downloaded, the program saves it in a new temporary directory with a prefix of "logo". The filename is extracted from the URL path segments.


## Running the Program

Assuming you have Rust installed, you can run the program using the following command:

```bash
cargo run
```

## Dependencies

This project uses the following Rust crates:

- `reqwest` - for creating the HTTP client and sending the request.
- `tokio` - for providing asynchronous runtime.
- `anyhow` and `thiserror` - for error handling.
- `tempfile` - for creating the temporary directory.
- `std::fs` - for creating the file and writing to it.

## Possible Improvements/Contribution Areas

While this project serves as a simple demonstration, there are many areas that could be improved or expanded:

- **Customization**: Currently, the URL of the file to download and the temporary directory name are hard-coded. These could be made customizable by allowing the user to specify them as command-line arguments.
- **Parallel Downloads**: The program could be enhanced to download multiple files at once using asynchronous operations, improving efficiency for multiple files.
- **Resuming Downloads**: Implement more advanced functionality like pausing and resuming file downloads.
- **Progress Bar**: Add a progress bar or other means of showing the download progress to the user.
- **Error Handling**: Improve error handling, for instance by adding retries for failed downloads or providing more specific error messages.

Please feel free to submit a pull request or open an issue if you have any ideas or suggestions.
