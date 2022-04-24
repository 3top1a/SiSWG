# Sucklessish Static Web Generator

SiSWG is a tool to convert markdown files to HTML for [my website](https://3top1a.github.io/).

## Help

```
EXAMPLES:
siswg file.md                            # Convert file.md to file.html, titled "file"
siswg file.md -o /path/to/output.html    # Convert file.md to output.html, titled "output"

USAGE:
    siswg [OPTIONS] <MARKDOWN_PATH>

ARGS:
    <MARKDOWN_PATH>    Specify the path of your Markdown file

OPTIONS:
    -o, --html-path <HTML_PATH>                      Specify the path of your HTML file
    -f, --force                                      Force to output if the HTML file exists
    -h, --help                                       Print help information
    -V, --version                                    Print version information
```

## Dependencies

If `#{{` - `}}#` or `#{{{` - `}}}#` is used in the input Markdown file, the [mathjax.js](https://www.mathjax.org/) will be automatically embedded in the output HTML file. `#{{` and `}}#` are `inlineMath` delimiters. `#{{{` and `}}}#` are `displayMath` delimiters. The default **mathjax.js** are using the [tex-mml-chtml](http://docs.mathjax.org/en/latest/web/components/combined.html#tex-mml-chtml) configuration file.

## License

[MIT](LICENSE)
