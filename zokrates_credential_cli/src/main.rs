use std::process;

use clap::{App, Arg, SubCommand};
use zokrates_credential_core::{issuer, prover, verifier};

use std::env;

fn to_absolute_path(relative_path: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let absolute_path = current_dir.join(relative_path);

    match absolute_path.canonicalize() {
        Ok(canonical_path) => canonical_path.to_str().unwrap().to_string(),
        Err(_) => {
            absolute_path.to_str().unwrap().to_string()
        },
    }
}

fn main() {
    let matches = App::new("ZoKrates Credential Issuer")
        .version("1.0")
        .author("ham3798 <5023798@naver.com>")
        .about("Manages credentials with ZoKrates")
        .subcommand(
            SubCommand::with_name("issuer")
                .about("Issues credentials")
                .subcommand(
                    SubCommand::with_name("create_credential")
                        .about("Creates a new credential")
                        .arg(
                            Arg::with_name("credential_id")
                                .help("The ID of the credential")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("name")
                                .help("Name of the credential owner")
                                .required(true)
                                .index(2),
                        )
                        .arg(
                            Arg::with_name("age")
                                .help("Age of the credential owner")
                                .required(true)
                                .index(3),
                        )
                        .arg(
                            Arg::with_name("student_number")
                                .help("Student number of the credential owner")
                                .required(true)
                                .index(4),
                        )
                        .arg(
                            Arg::with_name("department")
                                .help("Department of the credential owner")
                                .required(true)
                                .index(5),
                        )
                        .arg(
                            Arg::with_name("signature_save_path")
                                .help("Path to save the signature")
                                .required(true)
                                .index(6),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("setup")
                        .about("Performs setup operations for the issuer"),
                ),
        )
        .subcommand(
            SubCommand::with_name("verifier")
                .about("Functions for verifier")
                .subcommand(
                    SubCommand::with_name("get_proving_key")
                        .about("Copies the proving key to a specified destination")
                        .arg(
                            Arg::with_name("destination_path")
                                .help("The destination path to copy the proving key")
                                .required(true)
                                .index(1),
                        ),
                )
                .subcommand(SubCommand::with_name("setup").about("Runs the ZoKrates setup"))
                .subcommand(
                    SubCommand::with_name("get_verify_contract")
                        .about("Exports a verifier smart contract")
                        .arg(
                            Arg::with_name("destination_path")
                                .help("The destination path to export the verifier contract")
                                .required(true)
                                .index(1),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("prover")
                .about("Functions for the prover")
                .subcommand(
                    SubCommand::with_name("create_proof")
                        .about("Generates a proof for a given credential and signature")
                        .arg(
                            Arg::with_name("credential_path")
                                .help("Path to the credential file")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("signature_path")
                                .help("Path to the signature file")
                                .required(true)
                                .index(2),
                        )
                        .arg(
                            Arg::with_name("proving_key_path")
                                .help("Path to the proving key")
                                .required(true)
                                .index(3),
                        )
                        .arg(
                            Arg::with_name("destination_path")
                                .help("Destination path for the generated proof")
                                .required(true)
                                .index(4),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("setup")
                        .about("Performs setup operations for the prover"),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("issuer") {
        if let Some(create_matches) = matches.subcommand_matches("create_credential") {
            let credential_id = create_matches
                .value_of("credential_id")
                .expect("Missing credential_id");
            println!("Credential ID: {:?}", credential_id);

            let name = create_matches.value_of("name").expect("Missing name");
            let age = create_matches
                .value_of("age")
                .expect("Missing age")
                .parse::<u8>()
                .expect("Age must be a number");
            let student_number = create_matches
                .value_of("student_number")
                .expect("Missing student_number");
            let department = create_matches
                .value_of("department")
                .expect("Missing department");
            let signature_save_path = &to_absolute_path(
                create_matches
                    .value_of("signature_save_path")
                    .expect("Missing signature_save_path"),
            );

            issuer::create_credential(
                credential_id,
                name,
                age,
                student_number,
                department,
                signature_save_path,
            );
        } else if matches.subcommand_matches("setup").is_some() {
            issuer::setup();
        }
    } else if let Some(matches) = matches.subcommand_matches("verifier") {
        if let Some(get_pk_matches) = matches.subcommand_matches("get_proving_key") {
            let destination_path = &to_absolute_path(
                get_pk_matches
                    .value_of("destination_path")
                    .expect("Missing destination path"),
            );
            verifier::get_proving_key(destination_path);
            println!("Copying proving key to: {}", destination_path);
        } else if matches.subcommand_matches("setup").is_some() {
            verifier::setup();
        } else if let Some(cv_contract_matches) = matches.subcommand_matches("get_verify_contract")
        {
            let destination_path = &to_absolute_path(
                cv_contract_matches
                    .value_of("destination_path")
                    .expect("Missing destination path for contract export"),
            );
            println!("Exporting verifier contract to: {}", destination_path);
            verifier::get_verify_contract(destination_path);
        }
    } else if let Some(matches) = matches.subcommand_matches("prover") {
        if let Some(proof_matches) = matches.subcommand_matches("create_proof") {
            let credential_path =
                to_absolute_path(proof_matches.value_of("credential_path").unwrap());
            let signature_path =
                to_absolute_path(proof_matches.value_of("signature_path").unwrap());
            let proving_key_path =
                to_absolute_path(proof_matches.value_of("proving_key_path").unwrap());
            let destination_path =
                to_absolute_path(proof_matches.value_of("destination_path").unwrap());

            prover::create_proof(
                &credential_path,
                &signature_path,
                &proving_key_path,
                &destination_path,
            );
        } else if matches.subcommand_matches("setup").is_some() {
            prover::setup();
        }
    } else {
        eprintln!("Invalid command");
        process::exit(1);
    }
}
