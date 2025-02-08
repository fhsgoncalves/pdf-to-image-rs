use clap::Parser;
use pdfium_render::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}

fn main() {
    let cli = Cli::parse();

    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_file(&cli.input, None).unwrap();

    let page = document.pages().first().unwrap();

    let render_config = PdfRenderConfig::new().scale_page_by_factor(4.0);

    page.render_with_config(&render_config)
        .unwrap()
        .as_image()
        .save_with_format(cli.output, image::ImageFormat::Png)
        .unwrap();
}
