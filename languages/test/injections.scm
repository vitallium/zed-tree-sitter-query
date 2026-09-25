; Adapted from https://github.com/tree-sitter-grammars/tree-sitter-test/blob/master/queries/test/injections.scm
;
; tests that set a language with :language(...)
(test
  (header
    (name)
    (attributes
      . ; skip over non-language attributes
      (attribute
        !language)*
      . ; select only the first language attribute
      (attribute
        language: (parameter) @injection.language)))
  (input) @injection.content)

; "regular" tests: not using :cst
((test
  [
    (header
      (separator)
      .
      (name)
      .
      (separator))
    (header
      (attributes
        .
        (attribute
          !cst)+ .))
  ]
  (output) @injection.content)
  (#set! injection.language "Tree-sitter Query"))

; TODO: support tests with :cst attribute (Zed has no CST extension)
; TODO: inject project grammar into tests without :language (Zed currently can't do this)
