<div align="center">
# `📜 Sucklessish Static Web Generator`
</div>

SiSWG is a tool to convert markdown files to HTML for [my website](https://3top1a.github.io/).
It's built "sucklessly", meaning that the configuration is a rust file that you overwrite and recompile.

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

## License

[MIT](LICENSE)
