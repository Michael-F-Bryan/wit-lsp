(function() {var type_impls = {
"wit_compiler":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#277-292\">source</a><a href=\"#impl-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K: <a class=\"trait\" href=\"wit_compiler/access/trait.NodeKind.html\" title=\"trait wit_compiler::access::NodeKind\">NodeKind</a>&gt; <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.for_node\" class=\"method\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#278-286\">source</a><h4 class=\"code-header\">pub fn <a href=\"wit_compiler/access/struct.Pointer.html#tymethod.for_node\" class=\"fn\">for_node</a>(file: <a class=\"struct\" href=\"wit_compiler/queries/struct.FilePath.html\" title=\"struct wit_compiler::queries::FilePath\">FilePath</a>, node: K::<a class=\"associatedtype\" href=\"wit_compiler/access/trait.NodeKind.html#associatedtype.Ast\" title=\"type wit_compiler::access::NodeKind::Ast\">Ast</a>&lt;'_&gt;) -&gt; Self</h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.ast_node\" class=\"method\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#289-291\">source</a><h4 class=\"code-header\">pub fn <a href=\"wit_compiler/access/struct.Pointer.html#tymethod.ast_node\" class=\"fn\">ast_node</a>(self, tree: &amp;<a class=\"struct\" href=\"wit_compiler/struct.Tree.html\" title=\"struct wit_compiler::Tree\">Tree</a>) -&gt; K::<a class=\"associatedtype\" href=\"wit_compiler/access/trait.NodeKind.html#associatedtype.Ast\" title=\"type wit_compiler::access::NodeKind::Ast\">Ast</a>&lt;'_&gt;</h4></section></summary><div class=\"docblock\"><p>Get the <a href=\"wit_syntax/index.html\" title=\"mod wit_syntax\"><code>crate::ast</code></a> node from the AST <a href=\"wit_compiler/struct.Tree.html\" title=\"struct wit_compiler::Tree\"><code>Tree</code></a>.</p>\n</div></details></div></details>",0,"wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#294-306\">source</a><a href=\"#impl-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K&gt; <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.file\" class=\"method\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#295-297\">source</a><h4 class=\"code-header\">pub fn <a href=\"wit_compiler/access/struct.Pointer.html#tymethod.file\" class=\"fn\">file</a>(self) -&gt; <a class=\"struct\" href=\"wit_compiler/queries/struct.FilePath.html\" title=\"struct wit_compiler::queries::FilePath\">FilePath</a></h4></section><section id=\"method.range\" class=\"method\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#299-301\">source</a><h4 class=\"code-header\">pub fn <a href=\"wit_compiler/access/struct.Pointer.html#tymethod.range\" class=\"fn\">range</a>(self) -&gt; <a class=\"struct\" href=\"tree_sitter/struct.Range.html\" title=\"struct tree_sitter::Range\">Range</a></h4></section><section id=\"method.location\" class=\"method\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#303-305\">source</a><h4 class=\"code-header\">pub fn <a href=\"wit_compiler/access/struct.Pointer.html#tymethod.location\" class=\"fn\">location</a>(self) -&gt; <a class=\"struct\" href=\"wit_compiler/diagnostics/struct.Location.html\" title=\"struct wit_compiler::diagnostics::Location\">Location</a></h4></section></div></details>",0,"wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Hash-for-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#317-322\">source</a><a href=\"#impl-Hash-for-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> for <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#318-321\">source</a><a href=\"#method.hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hash.html#tymethod.hash\" class=\"fn\">hash</a>&lt;H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>&gt;(&amp;self, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;mut H</a>)</h4></section></summary><div class='docblock'>Feeds this value into the given <a href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hash.html#tymethod.hash\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hash_slice\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.3.0\">1.3.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/hash/mod.rs.html#238-240\">source</a></span><a href=\"#method.hash_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hash.html#method.hash_slice\" class=\"fn\">hash_slice</a>&lt;H&gt;(data: &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.slice.html\">[Self]</a>, state: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;mut H</a>)<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Feeds a slice of this type into the given <a href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\"><code>Hasher</code></a>. <a href=\"https://doc.rust-lang.org/1.77.2/core/hash/trait.Hash.html#method.hash_slice\">Read more</a></div></details></div></details>","Hash","wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#332-343\">source</a><a href=\"#impl-Debug-for-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#333-342\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"],["<section id=\"impl-Copy-for-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#330\">source</a><a href=\"#impl-Copy-for-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section>","Copy","wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"],["<section id=\"impl-Eq-for-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#308\">source</a><a href=\"#impl-Eq-for-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section>","Eq","wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#310-315\">source</a><a href=\"#impl-PartialEq-for-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#311-314\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Self</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Pointer%3CK%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#324-328\">source</a><a href=\"#impl-Clone-for-Pointer%3CK%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;K&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"wit_compiler/access/struct.Pointer.html\" title=\"struct wit_compiler::access::Pointer\">Pointer</a>&lt;K&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/wit_compiler/access.rs.html#325-327\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Self</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","wit_compiler::access::WorldPtr","wit_compiler::access::InterfacePtr","wit_compiler::access::RecordPtr","wit_compiler::access::TypeAliasPtr","wit_compiler::access::EnumPtr","wit_compiler::access::FlagsPtr","wit_compiler::access::ResourcePtr","wit_compiler::access::VariantPtr","wit_compiler::access::FunctionPtr","wit_compiler::access::ConstructorPtr","wit_compiler::access::MethodPtr","wit_compiler::access::StaticMethodPtr","wit_compiler::access::RecordFieldPtr","wit_compiler::access::VariantCasePtr","wit_compiler::access::EnumCasePtr","wit_compiler::access::FlagsCasePtr","wit_compiler::access::ExportPtr","wit_compiler::access::ImportPtr"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()