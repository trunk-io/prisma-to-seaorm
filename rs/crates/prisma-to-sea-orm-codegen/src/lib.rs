use std::io::Read;

mod codegen;
pub mod prisma_dmmf;

pub fn prisma_to_sea_orm_codegen(
    prisma_dmmf_datamodel: prisma_dmmf::Datamodel,
    module_name: impl AsRef<str>,
) -> anyhow::Result<String> {
    if !prisma_dmmf_datamodel.types.is_empty() {
        return Err(anyhow::anyhow!("Prisma composite types are unsupported."));
    }

    let tokens = codegen::module(&prisma_dmmf_datamodel, module_name);
    let item: syn::Item = syn::parse2(tokens)?;
    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![item],
    };

    Ok(prettyplease::unparse(&file))
}

pub fn parse_prisma_dmmf_datamodel(
    prisma_dmmf_datamodel_json: impl Read,
) -> anyhow::Result<prisma_dmmf::Datamodel> {
    Ok(serde_json::from_reader(prisma_dmmf_datamodel_json)?)
}
