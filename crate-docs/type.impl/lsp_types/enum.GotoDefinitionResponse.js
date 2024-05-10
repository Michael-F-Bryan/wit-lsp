(function() {var type_impls = {
"lsp_types":[["<section id=\"impl-StructuralPartialEq-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#impl-StructuralPartialEq-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section>","StructuralPartialEq","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#impl-Debug-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CVec%3CLocation%3E%3E-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2602-2606\">source</a><a href=\"#impl-From%3CVec%3CLocation%3E%3E-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"lsp_types/struct.Location.html\" title=\"struct lsp_types::Location\">Location</a>&gt;&gt; for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2603-2605\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(locations: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"lsp_types/struct.Location.html\" title=\"struct lsp_types::Location\">Location</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Vec<Location>>","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Deserialize%3C'de%3E-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#impl-Deserialize%3C'de%3E-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.deserialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#method.deserialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"serde/de/trait.Deserialize.html#tymethod.deserialize\" class=\"fn\">deserialize</a>&lt;__D&gt;(__deserializer: __D) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Self, __D::<a class=\"associatedtype\" href=\"serde/de/trait.Deserializer.html#associatedtype.Error\" title=\"type serde::de::Deserializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"serde/de/trait.Deserializer.html\" title=\"trait serde::de::Deserializer\">Deserializer</a>&lt;'de&gt;,</div></h4></section></summary><div class='docblock'>Deserialize this value from the given Serde deserializer. <a href=\"serde/de/trait.Deserialize.html#tymethod.deserialize\">Read more</a></div></details></div></details>","Deserialize<'de>","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#impl-PartialEq-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CVec%3CLocationLink%3E%3E-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2608-2612\">source</a><a href=\"#impl-From%3CVec%3CLocationLink%3E%3E-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"lsp_types/struct.LocationLink.html\" title=\"struct lsp_types::LocationLink\">LocationLink</a>&gt;&gt; for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2609-2611\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(locations: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"lsp_types/struct.LocationLink.html\" title=\"struct lsp_types::LocationLink\">LocationLink</a>&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Vec<LocationLink>>","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CLocation%3E-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2596-2600\">source</a><a href=\"#impl-From%3CLocation%3E-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"lsp_types/struct.Location.html\" title=\"struct lsp_types::Location\">Location</a>&gt; for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2597-2599\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(location: <a class=\"struct\" href=\"lsp_types/struct.Location.html\" title=\"struct lsp_types::Location\">Location</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Location>","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Serialize-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#impl-Serialize-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.serialize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#method.serialize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"serde/ser/trait.Serialize.html#tymethod.serialize\" class=\"fn\">serialize</a>&lt;__S&gt;(&amp;self, __serializer: __S) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;__S::<a class=\"associatedtype\" href=\"serde/ser/trait.Serializer.html#associatedtype.Ok\" title=\"type serde::ser::Serializer::Ok\">Ok</a>, __S::<a class=\"associatedtype\" href=\"serde/ser/trait.Serializer.html#associatedtype.Error\" title=\"type serde::ser::Serializer::Error\">Error</a>&gt;<div class=\"where\">where\n    __S: <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</div></h4></section></summary><div class='docblock'>Serialize this value into the given Serde serializer. <a href=\"serde/ser/trait.Serialize.html#tymethod.serialize\">Read more</a></div></details></div></details>","Serialize","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-GotoDefinitionResponse\" class=\"impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#impl-Clone-for-GotoDefinitionResponse\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/lsp_types/lib.rs.html#2588\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"enum\" href=\"lsp_types/enum.GotoDefinitionResponse.html\" title=\"enum lsp_types::GotoDefinitionResponse\">GotoDefinitionResponse</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","lsp_types::request::GotoDeclarationResponse","lsp_types::request::GotoTypeDefinitionResponse","lsp_types::request::GotoImplementationResponse"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()