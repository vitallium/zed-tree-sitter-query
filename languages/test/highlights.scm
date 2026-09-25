; Adapted from https://github.com/tree-sitter-grammars/tree-sitter-test/blob/master/queries/test/highlights.scm
;
(name) @constant @title

(separator) @punctuation.delimiter

(input) @embedded

[
  ":cst"
  ":error"
  ":fail-fast"
  ":language"
  ":platform"
  ":skip"
] @attribute

(attribute
  language: (parameter) @string)

(attribute
  platform: (parameter) @constant.builtin)

[
  "("
  ")"
] @punctuation.bracket
