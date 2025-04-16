# md2pdf-split-files

Rust Pandoc wrapper that converts Markdown files to multiple PDFs by splitting on `\newfile` markers.

## Installation

```bash
git clone https://github.com/zimengxiong/md2pdf-split-files.git && cd md2pdf-split-files
cargo install --path .
```

## Usage

```bash
md2pdf-split-files -i input.md [-o output_dir] -- [pandoc_arguments]
```

## Functionality

1. Takes a Markdown file as input
2. Splits the content at each `\newfile` marker
3. Uses the first heading after each marker as the PDF filename
4. Converts each section to PDF using Pandoc

## Example

```bash
git clone https://github.com/zimengxiong/md2pdf-split-files.git
cd md2pdf-split-files
cargo build
cd example
./../target/debug/md2pdf-split-files -i input.md -o out -- -V geometry:margin=1in
```

```bash
❯ ./../target/debug/md2pdf-split-files -i input.md -o out -- -V geometry:margin=1in
Generated PDF: cb_u3.3_&_u3.4:_mathematical_expressions_and_strings.pdf
Generated PDF: cb_u3.5:_boolean_expression.pdf
Generated PDF: cb_u3.6_&_u3.7_conditionals_&_nested_conditionals.pdf
Generated PDF: cb_u3.8_&_u3.9:_iteration_&_developing_algorithm.pdf
Generated PDF: cb_u3.10_&_u_3.11:_lists_and_binary_search.pdf
Generated PDF: cb_u3.12_&_u3.13:_calling_procedures_&_developing_procedures.pdf
Generated PDF: cb_u4:_the_internet,_fault_tolerance,_parallel_and_distributed_computing.pdf
```

## Requirements

- Pandoc must be installed on your path
