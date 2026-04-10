use std::{env, fs, path::Path};

use prisma_to_sea_orm_codegen::{parse_prisma_dmmf_datamodel, prisma_to_sea_orm_codegen};

pub fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    const SCHEMA_JSON: &[u8] = include_bytes!("./schema.json");
    println!("cargo::rerun-if-changed=schema.json");

    let prisma_dmmf_datamodel = parse_prisma_dmmf_datamodel(SCHEMA_JSON).unwrap();
    let sea_orm_codegen =
        prisma_to_sea_orm_codegen(prisma_dmmf_datamodel, "example_db", "public").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("example_db_codegen.rs");
    fs::write(dest_path, sea_orm_codegen).unwrap();
}
