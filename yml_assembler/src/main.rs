use clap::Parser;
use std::{collections::HashMap, error::Error, path::PathBuf, rc::Rc};
use yml_assembler::lib_infras::{
    assembly_fs_output::AssemblyFSOutput, assembly_part_fs_reader::PartFSReader,
    schema_fs_output::SchemaFSOutput, schema_fs_reader::SchemaFSReader,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The directory your yml files reside in
    #[arg(short, long)]
    root: PathBuf,

    /// The path in root to the yml file to assemble
    #[arg(short, long)]
    file: String,

    /// The path in root to the schema file to validate against
    #[arg(short, long)]
    schema: Option<String>,

    /// The path in root to the output folder
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Variables to insert in the yml file
    #[arg(short, long, value_parser = parse_key_val::<String, String>)]
    vars: Option<Vec<(String, String)>>,
}

static DEFAULT_OUTPUT: &str = "output";

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn cli() -> Result<(), anyhow::Error> {
    let Cli {
        output,
        file,
        root,
        schema,
        vars,
    } = Cli::parse();

    let display_variables = format!(
        "Using variables:\n{}",
        &vars
            .clone()
            .unwrap_or(vec![])
            .iter()
            .fold("".to_string(), |acc, (k, v)| format!(
                "{}{}={}\n",
                acc, k, v
            ))
    );
    let variables: HashMap<String, String> =
        HashMap::from_iter(vars.unwrap_or_default().into_iter());

    let outdir = PathBuf::from(DEFAULT_OUTPUT);
    let outdir = output.unwrap_or(outdir);

    println!("{}", display_variables);
    println!("Working in: {}", root.display());
    println!("Assembling files: {}", format!("{}", file));
    if let Some(schema) = schema.as_deref() {
        println!("Validating from schema: {}", schema);
    }
    println!("Outputing in: {}", outdir.display());

    let part_fs_reader = PartFSReader::new(root.clone());
    let schema_fs_reader = SchemaFSReader::new(root.clone());
    let assembly_fs_output = AssemblyFSOutput::new(outdir.clone());
    let schema_fs_output = SchemaFSOutput::new(outdir.clone());

    let app = yml_assembler::App::new(
        Rc::new(part_fs_reader),
        Rc::new(schema_fs_reader),
        Rc::new(assembly_fs_output),
        Rc::new(schema_fs_output),
    );

    app.compile_and_validate_yml(&file, schema.as_deref(), Some(variables))?;

    Ok(())
}

fn main() {
    match cli() {
        Ok(_) => println!("Assembling done!"),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
