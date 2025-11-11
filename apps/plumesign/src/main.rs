use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

use clap::{Args, Subcommand};
use ldid2::certificate::Certificate;
use ldid2::provision;
use ldid2::signing::signer::Signer;
use ldid2::signing::signer_settings::{SignerSettings, SignerMode};

#[derive(Debug, Parser)]
#[command(author, version, about, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Sign(SignArgs),
}

#[derive(Debug, Args)]
pub struct SignArgs {
    #[arg(long = "pem", value_name = "PEM", num_args = 1.., required = true)]
    pub pem_files: Vec<PathBuf>,

    #[arg(long = "provision", value_name = "PROVISION", num_args = 1.., required = true)]
    pub provisioning_files: Vec<PathBuf>,

    #[arg(value_name = "PACKAGE", long = "package", required = false)]
    pub package: Option<PathBuf>,

    #[arg(value_name = "BUNDLE", long = "bundle", required = false)]
    pub bundle: Option<PathBuf>,

    #[arg(short = 'w', long = "shallow", default_value_t = false)]
    pub shallow: bool,

    #[arg(long = "bundle-id", value_name = "BUNDLE_ID")]
    pub bundle_identifier: Option<String>,

    #[arg(long = "name", value_name = "NAME")]
    pub name: Option<String>,

    #[arg(long = "version", value_name = "VERSION")]
    pub version: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    match &cli.command {
        Commands::Sign(args) => {
            if args.pem_files.len() < 2 {
                eprintln!("Error: At least two PEM files (certificate and key) are required via --pem.");
                exit(1);
            }
            
            let target = args.bundle.as_ref().or(args.package.as_ref()).cloned();
            if target.is_none() {
                eprintln!("Error: Either --bundle or --package must be specified.");
                exit(1);
            }

            let signing_key = Certificate::new(args.pem_files.clone().into()).unwrap_or_else(|e| {
                eprintln!("Failed to create Certificate: {e}");
                exit(1);
            });

            let provisioning_files = args.provisioning_files.iter()
                .map(provision::MobileProvision::new)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|e| {
                    eprintln!("Failed to load provisioning profiles: {e}");
                    exit(1);
                });

            let signer_settings = SignerSettings {
                sign_shallow: args.shallow,
                sign_mode: if provisioning_files.len() == 1 { 
                    SignerMode::Zsign
                } else {
                    SignerMode::Default
                },
                custom_name: args.name.clone(),
                custom_identifier: args.bundle_identifier.clone(),
                custom_build_version: args.version.clone(),
                ..Default::default()
            };

            let signer = Signer::new(Some(signing_key), signer_settings, provisioning_files);

            let target_path = target.unwrap();
            if let Err(e) = signer.sign(target_path.clone()) {
                eprintln!("Failed to sign target: {e}");
                exit(1);
            }
            
            println!("Signed target: {:?}", target_path);
        }
    }
}
