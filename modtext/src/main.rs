use schema::mod_text::{Block, BlockArgs, Document, DocumentArgs};

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let symbol_strings = vec![
        builder.create_string("Hello"),
        builder.create_string("%Text"),
        builder.create_string("Document")
    ];
    let symbols = builder.create_vector(&symbol_strings);

    let root = Block::create(&mut builder, &BlockArgs {
        processor: 0,
        parameters: None,
        body: None,
    });
    let doc = Document::create(&mut builder, &DocumentArgs {
        processors: None,
        symbols: Some(symbols),
        root: Some(root),
    });

    builder.finish(doc, None);
    let buf = builder.finished_data();

    let doc = flatbuffers::root::<Document>(buf).unwrap();
    println!("Hello, flatbuffers!: {:?}", doc);
}
