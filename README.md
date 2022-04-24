# Sucklessish Static Web Generator

SiSWG is a tool to convert markdown files to HTML for [my website](https://3top1a.github.io/).

## Help

```
EXAMPLES:
siswg /path/to/file.md                            # Convert /path/to/file.md to /path/to/file.html, titled "file"
siswg /path/to/file.md -o /path/to/output.html    # Convert /path/to/file.md to /path/to/output.html, titled "output"
siswg /path/to/file.md -t 'Hello World!'          # Convert /path/to/file.md to /path/to/file.html, titled "Hello World!"

USAGE:
    siswg [OPTIONS] <MARKDOWN_PATH>

ARGS:
    <MARKDOWN_PATH>    Specify the path of your Markdown file

OPTIONS:
    -t, --title <TITLE>                              Specify the title of your HTML file
    -o, --html-path <HTML_PATH>                      Specify the path of your HTML file
    -f, --force                                      Force to output if the HTML file exists
    -h, --help                                       Print help information
    -V, --version                                    Print version information
```

## Dependencies

If `#{{` - `}}#` or `#{{{` - `}}}#` is used in the input Markdown file, the [mathjax.js](https://www.mathjax.org/) will be automatically embedded in the output HTML file. `#{{` and `}}#` are `inlineMath` delimiters. `#{{{` and `}}}#` are `displayMath` delimiters. The default **mathjax.js** are using the [tex-mml-chtml](http://docs.mathjax.org/en/latest/web/components/combined.html#tex-mml-chtml) configuration file.

## A Markdown Example

[The Markdown File](https://github.com/magiclen/markdown2html-converter/blob/master/example.md)

[The HTML File](https://jsfiddle.net/magiclen/jgs324w0/latest)

## License

[MIT](LICENSE)