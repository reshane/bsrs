// build.rs
use serde_json::Value;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/data/names.json");

    let data: Value = serde_json::from_str(
        include_str!("../src/data/names.json")
    ).unwrap();

    let names: Vec<String> = (data["names"])
        .as_array().unwrap().to_vec()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();

    let template = b"
#compdef _bsrs_completion bsrs

function _bsrs_completion {
    local line

    _arguments -C \
        \"-h[Show help information]\" \\
        \"--h[Show help information]\" \\
        \"1: :(";
    let template_middle = b")\" \\
        \"*::arg:->args\"

    case $line[1] in
";
    let template_end = b"
    esac
}
";

    let mut build_script: String = std::str::from_utf8(template).unwrap().to_string();
    for (i, name) in names.iter().enumerate() {
        build_script += &name.clone();
        if i < names.len()-1 {
            build_script += " ";
        }
    }
    build_script += std::str::from_utf8(template_middle).unwrap();

    let _format = b"
    case $line[1] in
        shane)
            _shane
        ;;
    ";
    for name in names.iter() {
        build_script += &format!("\t\t{})\n\t\t\t_{}\n\t\t;;\n", name.clone(), name.clone());
    }

    build_script += std::str::from_utf8(template_end).unwrap();

    for name in names.iter() {
        build_script += &format!("function _{} {{\n\t_arguments \\\n\t\t\"{}[says {}]\"\n}}\n",
            name.clone(),
            name.clone(),
            name.clone());
    }

    // write to a file
    let out_dir = "./scripts/";
    let dest_path = Path::new(&out_dir).join("_bsrs_completion");
    fs::write(
        &dest_path,
        &build_script
    ).unwrap();
}

