"as" @keyword
"enum" @keyword
"export" @keyword
"flags" @keyword
"func" @keyword
"import" @keyword
"include" @keyword
"interface" @keyword
"package" @keyword
"record" @keyword
"resource" @keyword
"type" @keyword
"use" @keyword
"variant" @keyword
"with" @keyword
"world" @keyword

"{" @punctuation.brackets.curly
"}" @punctuation.brackets.curly
"(" @punctuation.brackets.round
")" @punctuation.brackets.round
"->" @keyword.operator.arrow
"." @punctuation.accessor.dot
"," @punctuation.comma
";" @punctuation.semi

"static" @storage.modifier

(ty) @type

(named_type name: (identifier)) @variable.parameter

(block_comment) @comment.block
(doc_comment) @comment.documentation
(slash_comment) @comment.line.double-slash

(semver) @constant.other

(enum_item name: (identifier)) @entity.name.type.enum
(flags_item name: (identifier)) @entity.name.type.enum
(interface_item name: (identifier)) @entity.name.type.interface
(record_item name: (identifier)) @entity.name.type.struct
(resource_item name: (identifier)) @entity.name.type.interface
(variant_item name: (identifier)) @entity.name.type.enum
(world_item name: (identifier)) @entity.name.type.interface

(type_item name: (identifier)) @type

"constructor" @function.method
(resource_method (func_item name: (identifier))) @function.method
(static_method name: (identifier)) @function.method

(func_item name: (identifier)) @function

(attribute) @meta.attribute
