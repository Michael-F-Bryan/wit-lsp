(function() {var type_impls = {
"object":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-CoffSection%3C'data,+'file,+R,+Coff%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#271\">source</a><a href=\"#impl-Debug-for-CoffSection%3C'data,+'file,+R,+Coff%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'data, 'file, R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;, Coff: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"object/read/coff/trait.CoffHeader.html\" title=\"trait object::read::coff::CoffHeader\">CoffHeader</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"object/read/coff/struct.CoffSection.html\" title=\"struct object::read::coff::CoffSection\">CoffSection</a>&lt;'data, 'file, R, Coff&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#271\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","object::read::coff::section::CoffBigSection"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ObjectSection%3C'data%3E-for-CoffSection%3C'data,+'file,+R,+Coff%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#296-392\">source</a><a href=\"#impl-ObjectSection%3C'data%3E-for-CoffSection%3C'data,+'file,+R,+Coff%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'data, 'file, R: <a class=\"trait\" href=\"object/read/trait.ReadRef.html\" title=\"trait object::read::ReadRef\">ReadRef</a>&lt;'data&gt;, Coff: <a class=\"trait\" href=\"object/read/coff/trait.CoffHeader.html\" title=\"trait object::read::coff::CoffHeader\">CoffHeader</a>&gt; <a class=\"trait\" href=\"object/read/trait.ObjectSection.html\" title=\"trait object::read::ObjectSection\">ObjectSection</a>&lt;'data&gt; for <a class=\"struct\" href=\"object/read/coff/struct.CoffSection.html\" title=\"struct object::read::coff::CoffSection\">CoffSection</a>&lt;'data, 'file, R, Coff&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.RelocationIterator\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.RelocationIterator\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"object/read/trait.ObjectSection.html#associatedtype.RelocationIterator\" class=\"associatedtype\">RelocationIterator</a> = <a class=\"struct\" href=\"object/read/coff/struct.CoffRelocationIterator.html\" title=\"struct object::read::coff::CoffRelocationIterator\">CoffRelocationIterator</a>&lt;'data, 'file, R, Coff&gt;</h4></section></summary><div class='docblock'>An iterator for the relocations for a section. <a href=\"object/read/trait.ObjectSection.html#associatedtype.RelocationIterator\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.index\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#302-304\">source</a><a href=\"#method.index\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.index\" class=\"fn\">index</a>(&amp;self) -&gt; <a class=\"struct\" href=\"object/read/struct.SectionIndex.html\" title=\"struct object::read::SectionIndex\">SectionIndex</a></h4></section></summary><div class='docblock'>Returns the section index.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.address\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#307-309\">source</a><a href=\"#method.address\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.address\" class=\"fn\">address</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Returns the address of the section.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.size\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#312-315\">source</a><a href=\"#method.size\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.size\" class=\"fn\">size</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Returns the size of the section in memory.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.align\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#318-320\">source</a><a href=\"#method.align\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.align\" class=\"fn\">align</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u64.html\">u64</a></h4></section></summary><div class='docblock'>Returns the alignment of the section in memory.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.file_range\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#323-326\">source</a><a href=\"#method.file_range\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.file_range\" class=\"fn\">file_range</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u64.html\">u64</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u64.html\">u64</a>)&gt;</h4></section></summary><div class='docblock'>Returns offset and size of on-disk segment (if any).</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#328-330\">source</a><a href=\"#method.data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.data\" class=\"fn\">data</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;&amp;'data [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>]&gt;</h4></section></summary><div class='docblock'>Returns the raw contents of the section. <a href=\"object/read/trait.ObjectSection.html#tymethod.data\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.data_range\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#332-339\">source</a><a href=\"#method.data_range\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.data_range\" class=\"fn\">data_range</a>(&amp;self, address: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u64.html\">u64</a>, size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u64.html\">u64</a>) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'data [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>]&gt;&gt;</h4></section></summary><div class='docblock'>Return the raw contents of the section data in the given range. <a href=\"object/read/trait.ObjectSection.html#tymethod.data_range\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compressed_file_range\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#342-344\">source</a><a href=\"#method.compressed_file_range\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.compressed_file_range\" class=\"fn\">compressed_file_range</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;<a class=\"struct\" href=\"object/read/struct.CompressedFileRange.html\" title=\"struct object::read::CompressedFileRange\">CompressedFileRange</a>&gt;</h4></section></summary><div class='docblock'>Returns the potentially compressed file range of the section,\nalong with information about the compression.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.compressed_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#347-349\">source</a><a href=\"#method.compressed_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.compressed_data\" class=\"fn\">compressed_data</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;<a class=\"struct\" href=\"object/read/struct.CompressedData.html\" title=\"struct object::read::CompressedData\">CompressedData</a>&lt;'data&gt;&gt;</h4></section></summary><div class='docblock'>Returns the potentially compressed contents of the section,\nalong with information about the compression.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name_bytes\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#352-354\">source</a><a href=\"#method.name_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.name_bytes\" class=\"fn\">name_bytes</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>]&gt;</h4></section></summary><div class='docblock'>Returns the name of the section.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.name\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#357-362\">source</a><a href=\"#method.name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.name\" class=\"fn\">name</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;&amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.str.html\">str</a>&gt;</h4></section></summary><div class='docblock'>Returns the name of the section. <a href=\"object/read/trait.ObjectSection.html#tymethod.name\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.segment_name_bytes\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#365-367\">source</a><a href=\"#method.segment_name_bytes\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.segment_name_bytes\" class=\"fn\">segment_name_bytes</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>]&gt;&gt;</h4></section></summary><div class='docblock'>Returns the name of the segment for this section.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.segment_name\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#370-372\">source</a><a href=\"#method.segment_name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.segment_name\" class=\"fn\">segment_name</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.str.html\">str</a>&gt;&gt;</h4></section></summary><div class='docblock'>Returns the name of the segment for this section. <a href=\"object/read/trait.ObjectSection.html#tymethod.segment_name\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.kind\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#375-377\">source</a><a href=\"#method.kind\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.kind\" class=\"fn\">kind</a>(&amp;self) -&gt; <a class=\"enum\" href=\"object/enum.SectionKind.html\" title=\"enum object::SectionKind\">SectionKind</a></h4></section></summary><div class='docblock'>Return the kind of this section.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.relocations\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#379-385\">source</a><a href=\"#method.relocations\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.relocations\" class=\"fn\">relocations</a>(&amp;self) -&gt; <a class=\"struct\" href=\"object/read/coff/struct.CoffRelocationIterator.html\" title=\"struct object::read::coff::CoffRelocationIterator\">CoffRelocationIterator</a>&lt;'data, 'file, R, Coff&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"CoffRelocationIterator&lt;&#39;data, &#39;file, R, Coff&gt;\">ⓘ</a></h4></section></summary><div class='docblock'>Get the relocations for this section.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.flags\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/coff/section.rs.html#387-391\">source</a><a href=\"#method.flags\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#tymethod.flags\" class=\"fn\">flags</a>(&amp;self) -&gt; <a class=\"enum\" href=\"object/enum.SectionFlags.html\" title=\"enum object::SectionFlags\">SectionFlags</a></h4></section></summary><div class='docblock'>Section flags that are specific to each file format.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.uncompressed_data\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/object/read/traits.rs.html#396-398\">source</a><a href=\"#method.uncompressed_data\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"object/read/trait.ObjectSection.html#method.uncompressed_data\" class=\"fn\">uncompressed_data</a>(&amp;self) -&gt; <a class=\"type\" href=\"object/read/type.Result.html\" title=\"type object::read::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/alloc/borrow/enum.Cow.html\" title=\"enum alloc::borrow::Cow\">Cow</a>&lt;'data, [<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.u8.html\">u8</a>]&gt;&gt;</h4></section></summary><div class='docblock'>Returns the uncompressed contents of the section. <a href=\"object/read/trait.ObjectSection.html#method.uncompressed_data\">Read more</a></div></details></div></details>","ObjectSection<'data>","object::read::coff::section::CoffBigSection"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()