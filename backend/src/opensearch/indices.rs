use serde_json::{json, Value};

pub struct OpensearchIndex {
    pub name: &'static str,
    get_mappings: fn() -> Value,
}

impl OpensearchIndex {
    pub const INDICES: [OpensearchIndex; 1] = [Self::EXTERNAL_BANK_INSTITUTIONS];

    pub const EXTERNAL_BANK_INSTITUTIONS: OpensearchIndex = OpensearchIndex {
        name: "external_bank_institutions",
        get_mappings: get_external_bank_institutions_mapping,
    };

    pub fn get_mapping(&self) -> Value {
        (self.get_mappings)()
    }
}

fn get_external_bank_institutions_mapping() -> Value {
    json!(include_str!("mappings/external_bank_institutions.json"))
}
