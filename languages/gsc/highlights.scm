; ============================================================================
; GSC/CSC Tree-sitter Highlighting
; Based on the TextMate grammar from https://github.com/Blakintosh/gscode
; ============================================================================

; ============================================================================
; Comments
; ============================================================================

(comment) @comment
(dev_block) @comment.doc

; Documentation comments with internal syntax
(doc_comment) @comment.doc

(doc_type) @type.builtin
(doc_param_marker) @variable
(doc_gscode_directive) @keyword
(doc_text) @comment.doc

; ============================================================================
; Keywords - Storage Types
; ============================================================================

[
  "function"
  "class"
  "constructor"
  "destructor"
] @keyword

[
  "var"
  "const"
] @keyword

; ============================================================================
; Keywords - Storage Modifiers
; ============================================================================

[
  "private"
  "autoexec"
] @keyword

; ============================================================================
; Keywords - Control Flow
; ============================================================================

[
  "if"
  "else"
  "while"
  "for"
  "foreach"
  "in"
  "do"
  "switch"
  "case"
  "default"
  "break"
  "continue"
  "return"
  "wait"
  "waitrealtime"
  "waittill"
  "waittillmatch"
  "waittillframeend"
  "endon"
  "notify"
  "thread"
  "isdefined"
  "new"
] @keyword

; Line continuation in macros

; ============================================================================
; Preprocessor Directives
; ============================================================================

[
  "#using"
  "#insert"
  "#namespace"
  "#define"
  "#precache"
  "#using_animtree"
  "#if"
  "#elif"
  "#else"
  "#endif"
] @keyword

(preprocessor_using
  path: (preprocessor_path) @string)

(preprocessor_insert
  path: (preprocessor_path) @string)

(preprocessor_namespace
  name: (identifier) @namespace)

(preprocessor_define
  name: (identifier) @constant)

(macro_parameter_list) @variable

; ============================================================================
; Constants and Literals
; ============================================================================

(boolean_literal) @constant
(undefined_literal) @constant
(animtree_literal) @constant

(number) @number

(string_literal) @string
(istring_literal) @string
(hash_string_literal) @string
(localized_string_literal) @string

(escape_sequence) @string.escape

(anim_identifier) @constant

(anim_reference
  (anim_identifier) @constant
  "::" @punctuation.delimiter-resolution
  animation: (identifier) @constant)

(vector_literal) @number

; ============================================================================
; Built-in Variables
; ============================================================================

(builtin_variable) @variable.special
; ============================================================================
; Functions - Definitions
; ============================================================================

(function_definition
  name: (identifier) @function)

(function_definition
  modifier: _ @keyword)

(constructor_definition) @function
(destructor_definition) @function

; ============================================================================
; Functions - Calls
; ============================================================================

(call_expression
  function: (identifier) @function)

(call_expression
  function: (member_expression
    property: (identifier) @function))

; Direct function dereference calls: [[func]](args)
(call_expression
  function: (function_dereference) @function)

; Pointer calls (object method_name(args))
(pointer_call_expression
  object: (identifier) @variable)

(pointer_call_expression
  function: (identifier) @function)

(pointer_call_expression
  function: (namespace_call
    namespace: (identifier) @namespace
    function: (identifier) @function))

; Namespace resolution calls
(namespace_call
  namespace: (identifier) @namespace
  function: (identifier) @function)

(call_expression
  function: (namespace_call
    namespace: (identifier) @namespace
    function: (identifier) @function))

; ============================================================================
; Function Pointers
; ============================================================================

(function_pointer
  "&" @operator-pointer)

(function_pointer
  (identifier) @type-resolution
  "::" @punctuation.delimiter-resolution
  (identifier) @function)

(function_pointer
  (identifier) @function)

; Function dereference [[func]] or [ [ func ] ]
(function_dereference
  "[" @punctuation.bracket
  "]" @punctuation.bracket)

; ============================================================================
; Classes
; ============================================================================

(class_definition
  name: (identifier) @type)

(class_definition
  parent: (identifier) @type-class)

(new_expression
  class: (identifier) @type)

; ============================================================================
; Variables and Parameters
; ============================================================================

; Function parameters
(parameter_list
  (identifier) @variable.parameter)

(parameter_default
  name: (identifier) @variable.parameter)

(vararg) @operator

; Function call arguments
(argument_list
  (identifier) @variable)

; Variable declarations
(variable_declaration
  (identifier) @variable)

(const_statement
  name: (identifier) @constant)

(var_statement
  (identifier) @variable)

; Member access
(member_expression
  object: (identifier) @variable)

(member_expression
  property: (identifier) @property)

; Assignment expressions
(assignment_expression
  left: (identifier) @variable)

(assignment_expression
  left: (member_expression
    property: (identifier) @property))

(assignment_expression
  left: (subscript_expression) @variable)

; Subscript/array access
(subscript_expression
  "[" @punctuation.bracket
  "]" @punctuation.bracket)

(subscript_expression
  object: (identifier) @variable)

(subscript_expression
  index: (identifier) @variable)

; Identifiers in expressions
(binary_expression (identifier) @variable)
(unary_expression (identifier) @variable)
(update_expression (identifier) @variable)

; Ternary expression - capture all identifier fields
(ternary_expression condition: (identifier) @variable)
(ternary_expression consequence: (identifier) @variable)
(ternary_expression alternative: (identifier) @variable)

; Return statements
(return_statement (identifier) @variable)

; Control flow
(if_statement condition: (identifier) @variable)
(while_statement condition: (identifier) @variable)
(for_statement (identifier) @variable)
(foreach_statement element: (identifier) @variable)
(switch_statement value: (identifier) @variable)
(case_statement value: (identifier) @variable)

; Special expressions
(isdefined_expression (identifier) @variable)

; Expression statements - standalone identifiers
(expression_statement (identifier) @variable)



; ============================================================================
; Operators
; ============================================================================

; Assignment operators
[
  "="
  "+="
  "-="
  "*="
  "/="
  "%="
  "&="
  "|="
  "^="
  "<<="
  ">>="
] @operator

; Arithmetic operators
[
  "+"
  "-"
  "*"
  "/"
  "%"
] @operator

; Bitwise operators
[
  "<<"
  ">>"
  "&"
  "|"
  "^"
  "~"
] @operator

; Comparison operators
[
  "=="
  "==="
  "!="
  "!=="
  "<"
  ">"
  "<="
  ">="
] @operator

; Logical operators
[
  "&&"
  "||"
  "!"
] @operator

; Increment/decrement
[
  "++"
  "--"
] @operator

; Ternary operators
[
  "?"
  ":"
] @operator

; Scope resolution
"::" @punctuation.delimiter-resolution

; Member access operators
(member_expression
  "." @punctuation.delimiter)

(member_expression
  "->" @punctuation.delimiter)

; ============================================================================
; Punctuation
; ============================================================================

[
  "("
  ")"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  "{"
  "}"
] @punctuation.bracket

[
  ","
] @punctuation.delimiter

[
  ";"
] @punctuation.delimiter

; ============================================================================
; Special Statement Highlights
; ============================================================================

; Notify/endon/waittill event strings get special highlighting
(notify_statement
  event: (string_literal) @string)

(notify_statement
  event: (binary_expression
    (string_literal) @string))

(endon_statement
  event: (string_literal) @string)

(endon_statement
  event: (binary_expression
    (string_literal) @string))

(waittill_statement
  event: (string_literal) @string)

(waittill_statement
  event: (binary_expression
    (string_literal) @string))

; Built-in variables in special statements
(notify_statement
  object: (builtin_variable) @variable.special)

(endon_statement
  object: (builtin_variable) @variable.special)

(waittill_statement
  object: (builtin_variable) @variable.special)

; Precache strings
(preprocessor_precache
  (string_literal) @string)

; Using animtree strings
(preprocessor_using_animtree
  (string_literal) @string)

; ============================================================================
; Catch-all for identifiers (lowest priority)
; ============================================================================

; Removed - this was overriding all function highlighting
; (identifier) @variable
