; https://github.com/tree-sitter-grammars/tree-sitter-query/blob/master/queries/query/injections.scm

((predicate
  name: (identifier) @_name
  parameters:
    (parameters
      (string) @injection.content))
  (#any-of? @_name "match" "not-match" "vim-match" "not-vim-match")
  (#set! language "regex")
  (#offset! @injection.content 0 1 0 -1))

((predicate
  name: (identifier) @_name
  parameters:
    (parameters
      (string) @injection.content))
  (#any-of? @_name "lua-match" "not-lua-match")
  (#set! language "luap")
  (#offset! @injection.content 0 1 0 -1))

((predicate
  name: (identifier) @_name
  parameters:
    (parameters
      (string) @injection.content
      .
      (string) .))
  (#any-of? @_name "gsub" "not-gsub")
  (#set! language "luap")
  (#offset! @injection.content 0 1 0 -1))

((comment) @injection.content
  (#set! language "comment"))
