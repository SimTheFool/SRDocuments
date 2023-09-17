use clap::Parser;
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
    rc::Rc,
};
use yml_assembler::adapters::{ValidationSchemaFileSystemReader, YmlFileSystemReader};

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
    output: Option<String>,

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
    println!("{}", display_variables);

    let variables: HashMap<String, String> =
        HashMap::from_iter(vars.unwrap_or_default().into_iter());

    let outdir = output.as_deref().unwrap_or(DEFAULT_OUTPUT);

    println!("Working in: {}", root.display());
    println!("Assembling files: {}", format!("{}", file));
    if let Some(schema) = schema.as_deref() {
        println!("Validating from schema: {}", schema);
    }
    println!("Outputing in: {}", outdir);

    let yml_reader = YmlFileSystemReader::new(root.clone());
    let schema_reader = ValidationSchemaFileSystemReader::new(root.clone());

    let app = yml_assembler::App::new(Rc::new(yml_reader), Rc::new(schema_reader));
    let yml = app.compile_and_validate_yml(&file, schema.as_deref(), Some(variables))?;

    let outdir_path = PathBuf::from(outdir);
    let outfile_path = Path::new(&outdir_path).join(&file).with_extension("yml");
    let parent = outfile_path.parent().ok_or_else(|| {
        anyhow::anyhow!(format!(
            "Could not get parent directory of {}",
            outfile_path.display()
        ))
    })?;
    if !parent.exists() {
        std::fs::create_dir_all(&parent)
            .map_err(|e| anyhow::anyhow!(format!("Could not create output directory: {}", e)))?;
    }
    std::fs::write(
        outfile_path,
        serde_yaml::to_string(&yml)
            .map_err(|e| anyhow::anyhow!(format!("Could not serialize file: {}", e)))?,
    )
    .map_err(|e| anyhow::anyhow!(format!("Could not write file to output directory: {}", e)))?;

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
