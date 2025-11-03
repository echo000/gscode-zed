; ============================================================================
; GSC Text Objects
; ============================================================================

;; Join up all the comments
(comment)+ @comment.around
(dev_block)+ @comment.around
(doc_comment)+ @comment.around

;; Functions - entire function including signature
(function_definition) @function.around

;; Functions - just the body content
(function_definition
  body: (_ "{" (_)* @function.inside "}"))

;; Constructor/Destructor
(constructor_definition) @function.around
(destructor_definition) @function.around

(constructor_definition
  body: (_ "{" (_)* @function.inside "}"))

(destructor_definition
  body: (_ "{" (_)* @function.inside "}"))

;; Classes - entire class definition
(class_definition) @class.around

;; Classes - just the body content
(class_definition
  body: (_ "{" (_)* @class.inside "}"))
