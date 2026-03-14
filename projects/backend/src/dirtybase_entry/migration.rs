mod mig_1773373410_createapplicationtables;
/**
 * The following function is automatically modified
 * do not manually edit it
 */
pub(crate) fn setup() -> Option<dirtybase_contract::ExtensionMigrations> {
    dirtybase_contract::register_migration![
        mig_1773373410_createapplicationtables::Mig1773373410CreateApplicationTables,
        //
    ]
}
