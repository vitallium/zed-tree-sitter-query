# Tree-sitter Query Extension for Zed

This extension adds support for Tree-sitter query files (`.scm`) to the Zed editor.

## Using [the `ts_query_ls` language server](https://github.com/ribru17/ts_query_ls)

This extension is configured to use the `ts_query_ls` language server, which
provides intelligent features for Tree-sitter query files including:

- Syntax checking and diagnostics
- Auto-completion for node types and captures
- Hover information
- Go to definition support

### Configuration

The language server is configured with the following settings via `.tsqueryrc.json`:

```json
{
  "$schema": "https://raw.githubusercontent.com/ribru17/ts_query_ls/refs/heads/master/schemas/config.json",
  "parser_install_directories": ["./grammars"],
  "language_retrieval_patterns": ["languages/([^/]+)/[^/]+\\.scm"],
  "valid_captures": {
    "highlights": {
      "attribute": "An attribute",
      "boolean": "A boolean value",
      "comment": "A comment",
      "comment.doc": "A documentation comment",
      "constant": "A constant",
      "constructor": "A constructor",
      "embedded": "Embedded content",
      "emphasis": "Emphasized text",
      "emphasis.strong": "Strongly emphasized text",
      "enum": "An enumeration",
      "function": "A function",
      "hint": "A hint",
      "keyword": "A keyword",
      "keyword.exception": "A keyword exception",
      "label": "A label",
      "link_text": "Link text",
      "link_uri": "A link URI",
      "number": "A numeric value",
      "operator": "An operator",
      "predictive": "Predictive text",
      "preproc": "A preprocessor directive",
      "primary": "A primary element",
      "property": "A property",
      "punctuation": "Punctuation",
      "punctuation.bracket": "A bracket",
      "punctuation.delimiter": "A delimiter",
      "punctuation.list_marker": "A list marker",
      "punctuation.special": "Special punctuation",
      "string": "A string literal",
      "string.escape": "An escaped character in a string",
      "string.regex": "A regular expression",
      "string.special": "A special string",
      "string.special.symbol": "A special symbol",
      "tag": "A tag",
      "tag.doctype": "A doctype (e.g., in HTML)",
      "text.literal": "Literal text",
      "title": "A title",
      "type": "A type",
      "variable": "A variable",
      "variable.special": "A special variable",
      "variant": "A variant"
    }
  }
}
```

This configuration:

- Looks for Tree-sitter parsers in the `./grammars` directory - Detects language
  contexts from file paths matching the pattern `languages/([^/]+)/[^/]+\.scm` -
  Validates capture names against Zed's standard highlight captures - [Check the
  language server documentation for more information on configuration
  options](https://github.com/ribru17/ts_query_ls/blob/master/README.md#configuration).

## Acknowledgements

- Based on the [tree-sitter-query](https://github.com/tree-sitter-grammars/tree-sitter-query) grammar
- Syntax highlighting and injection queries adapted from the official tree-sitter-query repository
