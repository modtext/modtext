use schema::mod_text::{
    Block, BlockArgs, Document, DocumentArgs, DocumentSpan, DocumentSpanArgs, Fragment,
    FragmentArgs, FragmentType, Processor, ProcessorArgs,
};

fn processors_array<
    'bldr: 'args,
    'args: 'mut_bldr,
    'mut_bldr,
    A: flatbuffers::Allocator + 'bldr,
>(
    items: &[flatbuffers::WIPOffset<Processor<'bldr>>],
    builder: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
) -> flatbuffers::WIPOffset<
    flatbuffers::Vector<'bldr, flatbuffers::ForwardsUOffset<schema::mod_text::Processor<'bldr>>>,
> {
    builder.create_vector(items)
}

fn processor<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    name: &str,
    builder: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
) -> flatbuffers::WIPOffset<Processor<'bldr>> {
    let proc_name = builder.create_string(name);
    Processor::create(
        builder,
        &ProcessorArgs {
            name: Some(proc_name),
        },
    )
}

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let symbol_strings = vec![
        builder.create_string("Hello"),
        builder.create_string("%Text"),
        builder.create_string("Document"),
    ];
    let symbols = builder.create_vector(&symbol_strings);

    let processors = processors_array(
        &[
            processor("format", &mut builder),
            processor("style", &mut builder),
        ],
        &mut builder,
    );
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
    let root_fragments = builder.create_vector(&[root_body]);
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
    println!("Hello, flatbuffers!: {:#?}", doc);
}
