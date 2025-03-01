use schema::mod_text::{
    Block, BlockArgs, Document, DocumentArgs, DocumentSpan, DocumentSpanArgs, Fragment,
    FragmentArgs, FragmentType, Processor, ProcessorArgs,
};

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let symbol_strings = vec![
        builder.create_string("Hello"),
        builder.create_string("%Text"),
        builder.create_string("Document"),
    ];
    let symbols = builder.create_vector(&symbol_strings);

    let proc_0_name = builder.create_string("format");
    let proc_0 = Processor::create(
        &mut builder,
        &ProcessorArgs {
            name: Some(proc_0_name),
        },
    );
    let processors = builder.create_vector(&vec![proc_0]);
    // TODO(akesling): These should _really_ be represented by sub-blocks of processor type
    // "symbol" which take an index paremeter.
    let root_template = builder.create_string("%symbol{0} %symbol{1} %symbol{2}!");
    let root_span = DocumentSpan::create(
        &mut builder,
        &DocumentSpanArgs {
            value: Some(root_template),
        },
    );
    let root_body = Fragment::create(
        &mut builder,
        &FragmentArgs {
            value: Some(root_span.as_union_value()),
            value_type: FragmentType::DocumentSpan,
        },
    );
    let root_fragments = builder.create_vector(&vec![root_body]);
    let root = Block::create(
        &mut builder,
        &BlockArgs {
            processor: 0,
            parameters: None,
            body: Some(root_fragments),
        },
    );
    let doc = Document::create(
        &mut builder,
        &DocumentArgs {
            processors: Some(processors),
            symbols: Some(symbols),
            root: Some(root),
        },
    );

    builder.finish(doc, None);
    let buf = builder.finished_data();

    let doc = flatbuffers::root::<Document>(buf).unwrap();
    println!("Hello, flatbuffers!: {:?}", doc);
}
