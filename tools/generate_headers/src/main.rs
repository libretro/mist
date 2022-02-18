fn main() {
    let path = std::path::PathBuf::from(std::env::args().nth(1).expect("include path"));
    let include_path = path.join("include");
    std::fs::write(
        include_path.join("mist_results.h"),
        mist::result::generate_header(),
    )
    .unwrap();

    // Callback codegen requires macro expansion
    std::env::set_var("RUSTC_BOOTSTRAP", "1");
    let bindings = cbindgen::Builder::new()
        .with_config(cbindgen::Config {
            no_includes: true,
            language: cbindgen::Language::C,
            export: cbindgen::ExportConfig {
                include: mist::callbacks::MistCallback::get_struct_idents(),
                item_types: vec![cbindgen::ItemType::Structs],
                ..Default::default()
            },
            parse: cbindgen::ParseConfig {
                expand: cbindgen::ParseExpandConfig {
                    crates: vec!["mist".into()],
                    all_features: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_crate_and_name(&path, "mist")
        .generate()
        .unwrap();

    let mut out = Vec::new();
    bindings.write(&mut out);
    let header = String::from_utf8_lossy(&out).to_string();

    let mut callback_struct_idents = std::collections::HashSet::new();

    for i in mist::callbacks::MistCallback::get_struct_idents() {
        callback_struct_idents.insert(i);
    }

    let mut callbacks_out = String::new();

    let mut pos = 0;
    let mut start = None;
    let mut callbacks = Vec::new();
    // Filter out headers we do not want
    for line in header.lines() {
        if line.starts_with("typedef struct ") {
            let end = line.find("{").unwrap() - 1;
            let ident: &str = &line["typedef struct ".len()..end];
            if callback_struct_idents.contains(ident) {
                start = Some(pos);
            }
        }

        pos += line.len() + 1;

        if line.starts_with("} ") {
            let end = line.find(";").unwrap();
            let ident = &line[2..end];

            if callback_struct_idents.contains(ident) {
                let text = header[start.unwrap()..pos].to_string();
                let callback_id = mist::callbacks::MistCallback::get_struct_callback(ident);
                callbacks.push((ident, callback_id));

                callbacks_out.push_str(&text);
                callbacks_out.push_str("\n");

                start = None;
            }
        }
    }

    callbacks_out.push_str("enum {\n");

    for (ident, value) in callbacks {
        let mut ident = ident.to_string();
        ident.insert("MistCallback".len(), '_');
        callbacks_out.push_str(&format!("  {} = {},\n", ident, value));
    }

    callbacks_out.push_str("};\n");

    std::fs::write(include_path.join("mist_callbacks.h"), callbacks_out).unwrap();
}
