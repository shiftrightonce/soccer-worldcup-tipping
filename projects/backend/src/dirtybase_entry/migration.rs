mod mig_1773373410_createapplicationtables;
mod mig_1778062247_create_default_admin;
mod mig_1779089663_insertdata;
/**
 * The following function is automatically modified
 * do not manually edit it
 */
pub(crate) fn setup() -> Option<dirtybase_contract::ExtensionMigrations> {
    dirtybase_contract::register_migration![
        mig_1779089663_insertdata::Mig1779089663InsertData,
        mig_1778062247_create_default_admin::Mig1778062247CreateDefaultAdmin,
        mig_1773373410_createapplicationtables::Mig1773373410CreateApplicationTables,
        //
    ]
}
