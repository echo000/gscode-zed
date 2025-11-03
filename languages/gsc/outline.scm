; ============================================================================
; GSC Outline/Symbol View Configuration
; ============================================================================

;; Functions - show name and parameters
(function_definition
    name: (identifier) @name
    parameters: (parameter_list) @context
) @item

;; Classes - show name and optionally parent class
(class_definition
    name: (identifier) @name
    parent: (identifier)? @context.extra
) @item

;; Constructors - show parameters as context
(constructor_definition
    parameters: (parameter_list) @context
) @item

;; Destructors
(destructor_definition) @item

;; Preprocessor namespace
(preprocessor_namespace
    name: (identifier) @name
) @item

;; Const declarations - show as constants in outline
(const_statement
    name: (identifier) @name
) @item

;; Preprocessor defines - show macro name
(preprocessor_define
    name: (identifier) @name
) @item

;; Doc comments as annotations for following items
(doc_comment) @annotation

;; Comment blocks as annotations
(comment) @annotation
