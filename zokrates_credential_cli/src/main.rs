use std::process;

use clap::{App, Arg, SubCommand};
use zokrates_credential_core::issuer::{create_credential, setup};
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
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("issuer") {
        if let Some(create_matches) = matches.subcommand_matches("create_credential") {
            // 인자 추출은 이제 create_matches를 사용
            let credential_id = create_matches
                .value_of("credential_id")
                .expect("Missing credential_id");
            println!("Credential ID: {:?}", credential_id); // 이제 올바른 위치에 있음

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
            let signature_save_path = create_matches
                .value_of("signature_save_path")
                .expect("Missing signature_save_path");

            create_credential(
                credential_id,
                name,
                age,
                student_number,
                department,
                signature_save_path,
            );
        } else if matches.subcommand_matches("setup").is_some() {
            setup();
        }
    } else {
        eprintln!("Invalid command");
        process::exit(1);
    }
}
