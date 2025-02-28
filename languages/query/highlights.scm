; From https://github.com/helix-editor/helix/blob/master/runtime/queries/tsq/highlights.scm

((program
  .
  (comment)*
  .
  (comment) @keyword)
  (#match? @keyword "^;+ *inherits *:"))

((parameters
  (identifier) @constant)
  (#match? @constant "^[-+]?[0-9]+(.[0-9]+)?$"))

"_" @constant

":" @punctuation.delimiter

[
  "["
  "]"
  "("
  ")"
] @punctuation.bracket

"." @operator

(quantifier) @operator

(comment) @comment

(negated_field
  "!" @operator
  (identifier) @variable)

(field_definition
  name: (identifier) @variable)

(named_node
  name: (identifier) @tag)

(predicate name: (identifier) @error)
((predicate
   "#" @function.builtin
   name: (identifier) @function.builtin @_name
   type: (predicate_type) @function.builtin)
 (#any-of? @_name "eq" "not-eq" "match" "not-match" "any-of" "not-any-of" "is" "is-not" "not-same-line" "not-kind-eq" "set" "select-adjacent" "strip"))

(capture) @label

(escape_sequence) @constant

(string) @string
