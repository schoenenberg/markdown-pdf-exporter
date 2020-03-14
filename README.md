# markdown-pdf-exporter

This small tool generates a PDF from a markdown file.

```
markdown_pdf_exporter 0.1.0
Maximilian Schoenenberg <max.schoenenberg@me.com>
Takes a Markdown file and converts it into a PDF.

USAGE:
    markdown_pdf_exporter [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Outputs also the HTML output to STDOUT.

OPTIONS:
    -i, --input <input>              The input file. If no file is provided, it takes the input from STDIN
    -o, --output <output>            The output file. [default: main.pdf]
    -s, --stylesheet <stylesheet>    Override the default stylesheet, by providing a path to a css file.
```
