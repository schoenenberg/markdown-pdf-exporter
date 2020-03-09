extern crate comrak;
extern crate wkhtmltopdf;

#[macro_use]
extern crate clap;

use wkhtmltopdf::{PdfApplication, Orientation, Size, PageSize};
use comrak::{markdown_to_html, ComrakOptions};
use clap::{App, Arg};
use std::io::Read;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("The input file. If no file is provided, it takes the input from STDIN")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .default_value("main.pdf")
            .help("The output file.")
        )
        .get_matches();

    let path: Option<String> = match app.is_present("input") {
        true => Option::Some(app.value_of("input").unwrap().to_string()),
        false => Option::None,
    };

    let file_content = read_input(path);
    let html = markdown_to_html(&file_content, &ComrakOptions::default());

    convert_to_pdf(&html, app.value_of("output").unwrap())
}

fn convert_to_pdf(content: &String, output_path: &str) {
    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Portrait)
        .margin(Size::Inches(1))
        .page_size(PageSize::A4)
        .build_from_html(content)
        .expect("failed to build pdf");

    pdfout.save(output_path).expect("failed to save foo.pdf");
}

fn read_input(path: Option<String>) -> String {
    let mut content: Vec<u8> = Vec::with_capacity(2048);

    match path {
        None => {
            // Read from STDIN
            std::io::stdin().read_to_end(&mut content).expect("reading from STDIN failed");
        }
        Some(file) => {
            // Read from file
            let mut io = std::fs::File::open(file).expect("file couldn't be opened");
            io.read_to_end(&mut content).expect("reading file failed");
        }
    }

    String::from_utf8(content).expect("failed to read markdown")
}