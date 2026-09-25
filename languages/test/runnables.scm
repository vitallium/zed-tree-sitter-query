; run all tests in a file
((file
  .
  (test) @run) @_tree-sitter-test-file
  (#set! tag tree-sitter-test-file))

; run a specific test
((test
  (header
    (name) @run)) @_tree-sitter-test
  (#set! tag tree-sitter-test))
