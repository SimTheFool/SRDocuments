use clap::Parser;
use std::{
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
}

static DEFAULT_OUTPUT: &str = "output";

fn cli() -> Result<(), anyhow::Error> {
    let Cli {
        output,
        file,
        root,
        schema,
    } = Cli::parse();
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
    let yml = app.compile_and_validate_yml(&file, schema.as_deref())?;

    let outdir_path = PathBuf::from(outdir);
    let outfile_path = Path::new(&outdir_path).join(&file).with_extension("yml");

    //create directory if not exists
    if !outdir_path.exists() {
        std::fs::create_dir_all(&outdir_path)
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
