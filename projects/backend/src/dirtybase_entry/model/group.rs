use dirtybase_app::{
    db::{
        base::paginate_builder::{PaginateBuilder, PaginateResult},
        types::{
            ArcUuid7, CreatedAtField, DeletedAtField, IntegerField, StringField, UpdatedAtField,
        },
    },
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use serde::Serialize;

use crate::dirtybase_entry::model::{country::Country, tournament::Tournament};

#[derive(Debug, Default, Clone, DirtyTable, Serialize, ts_rs::TS)]
#[ts(export)]
#[dirty(soft_deletable, id_not_auto, timestamp, id_not_auto)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    pub(crate) name: StringField,
    #[ts(type = "Date")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date")]
    pub(crate) deleted_at: DeletedAtField,
}

impl Group {
    pub fn new(name: &str) -> Self {
        Self {
            id: Some(ArcUuid7::default()),
            name: name.to_string().into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, DirtyTable, Serialize, ts_rs::TS)]
#[ts(export)]
#[dirty(soft_deletable, id_not_auto, timestamp, id_not_auto)]
#[serde(rename_all = "camelCase")]
pub struct CountryGroup {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) tournament: Option<Tournament>,
    #[ts(type = "string")]
    pub(crate) tournament_id: ArcUuid7,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) group: Option<Group>,
    #[ts(type = "string")]
    pub(crate) group_id: ArcUuid7,
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) country: Option<Country>,
    #[ts(type = "string")]
    pub(crate) country_id: ArcUuid7,
    pub(crate) is_out: bool,
    #[ts(type = "number")]
    pub(crate) points: IntegerField,
    #[ts(type = "Date | null")]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date | null")]
    pub(crate) deleted_at: DeletedAtField,
}

impl CountryGroupRepo {
    pub async fn paginate_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<CountryGroup> {
        self.with_country().with_group().with_tournament();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.paginate(page).await
    }

    pub async fn all_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
    ) -> Result<Vec<CountryGroup>, anyhow::Error> {
        self.with_country().with_group().with_tournament();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.get().await
    }

    pub async fn by_tournament_and_id(
        &mut self,
        tournament_id: ArcUuid7,
        id: ArcUuid7,
    ) -> Result<Option<CountryGroup>, anyhow::Error> {
        self.with_country().with_group().with_tournament();
        self.builder.is_eq(Self::col_tournament_id(), tournament_id);
        self.by_id(id).await
    }
}
