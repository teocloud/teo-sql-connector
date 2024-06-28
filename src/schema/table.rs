use crate::stmts::create::table::SQLCreateTableStatement;
use crate::stmts::SQL;
use teo_runtime::model::Model;

impl From<&Model> for SQLCreateTableStatement {
    fn from(model: &Model) -> Self {
        let mut stmt = SQL::create().table(model.table_name());
        stmt.if_not_exists();
        for (_name, field) in model.fields() {
            if !field.r#virtual() {
                stmt.column(field.into());
            }
        }
        for (_name, property) in model.properties() {
            if property.cached() {
                stmt.column(property.into());
            }
        }
        if model.primary_index().unwrap().keys().len() > 1 {
            stmt.primary(model.primary_index().unwrap().clone());
        }
        stmt
    }
}
