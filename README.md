# pdf-to-image-rs

The objective of this project is to serve as example of converting a pdf to image using pdfium-render rust lib, and report a bug of missing letters in the resulting image file.

## Running
```
cargo run -- --input data/file.pdf --output data/file.png
```
The resulting file will be located on `data/file.png`, but missing some letters in relation to the original file.
