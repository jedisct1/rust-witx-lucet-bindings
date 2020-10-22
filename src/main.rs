#[macro_use]
extern crate clap;

use clap::Arg;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("output_file")
                .short("-o")
                .long("--output")
                .value_name("output_file")
                .multiple(false)
                .help("Output file, or - for the standard output"),
        )
        .arg(
            Arg::with_name("witx_file")
                .multiple(false)
                .required(true)
                .help("Witx file"),
        )
        .get_matches();

    let mut writer: Box<dyn Write> = match matches.value_of("output_file") {
        None | Some("-") => Box::new(std::io::stdout()),
        Some(file) => Box::new(File::create(file).unwrap()),
    };
    let witx_file = matches.value_of("witx_file").unwrap();

    let mut bindings_modules: HashMap<String, HashMap<String, String>> = HashMap::new();
    let document = witx::load(&[witx_file]).unwrap();
    for module in document.modules() {
        let module = module.as_ref();
        let module_name = module.name.as_str();
        let mut bindings_functions: HashMap<String, String> = HashMap::new();
        for func in module.funcs() {
            let func = func.as_ref();
            let name = func.name.as_str();
            let lucet_name = format!("hostcall_{}_{}", module_name, name);
            bindings_functions.insert(name.to_string(), lucet_name.to_string());
        }
        bindings_modules.insert(module_name.to_string(), bindings_functions);
    }
    let json = json::stringify_pretty(bindings_modules, 4);
    writer.write_all(json.as_bytes()).unwrap();
}
