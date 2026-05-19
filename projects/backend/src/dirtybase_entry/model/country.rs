use dirtybase_app::{
    db::{
        base::paginate_builder::{PaginateBuilder, PaginateResult},
        types::{ArcUuid7, CreatedAtField, DeletedAtField, NameField, StringField, UpdatedAtField},
    },
    db_macro::DirtyTable,
};
use dirtybase_common::anyhow;
use serde::{Deserialize, Serialize};

use crate::dirtybase_entry::model::{
    group::{CountryGroup, CountryGroupRepo, Group},
    tournament::Tournament,
};

#[derive(Debug, Default, Clone, DirtyTable, Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
#[dirty(soft_deletable, id_not_auto, timestamp, id_not_auto)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    #[ts(type = "string")]
    pub(crate) id: Option<ArcUuid7>,
    pub(crate) name: StringField,
    #[ts(type = "string")]
    pub(crate) alpha2: NameField,
    #[ts(type = "string")]
    pub(crate) alpha3: NameField,
    #[serde(skip_deserializing)]
    #[dirty(rel(
        kind = "has_many",
        pivot = CountryGroup,
        pivot_through_col = "id",
        through_col: "tournament_id",
    ))]
    pub(crate) tournaments: Option<Vec<Tournament>>,
    #[serde(skip_deserializing)]
    #[dirty(rel(
        kind = "has_many_through",
        pivot = CountryGroup,
        pivot_through_col = "id",
        through_col: "group_id",
    ))]
    pub(crate) groups: Option<Vec<Group>>,
    #[serde(skip_deserializing)]
    #[dirty(rel(kind = "has_many"))]
    pub(crate) coutry_group: Option<Vec<CountryGroup>>,
    #[ts(type = "Date | null")]
    #[serde(skip_deserializing)]
    pub(crate) created_at: CreatedAtField,
    #[ts(type = "Date | null")]
    #[serde(skip_deserializing)]
    pub(crate) updated_at: UpdatedAtField,
    #[ts(type = "Date | null")]
    #[serde(skip_deserializing)]
    pub(crate) deleted_at: DeletedAtField,
}

impl Country {
    pub fn new(name: &str, alpha2: &str, alpha3: &str) -> Self {
        Self {
            id: Some(ArcUuid7::default()),
            name: name.to_string().into(),
            alpha2: alpha2.to_string().into(),
            alpha3: alpha3.to_string().into(),
            ..Default::default()
        }
    }
}

impl CountryRepo {
    pub async fn paginate_by_tournament(
        &mut self,
        tournament_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<Country> {
        self.with_coutry_group_where(|query| {
            query
                .query_mut()
                .is_eq(CountryGroupRepo::col_tournament_id(), tournament_id);
        });
        self.paginate(page).await
    }

    pub async fn by_tournament_and_id(
        &mut self,
        tournament_id: ArcUuid7,
        id: ArcUuid7,
    ) -> Result<Option<Country>, anyhow::Error> {
        self.with_coutry_group_where(|query| {
            query
                .query_mut()
                .is_eq(CountryGroupRepo::col_tournament_id(), tournament_id);
        });
        self.by_id(id).await
    }

    pub async fn paginate_still_in(
        &mut self,
        tournament_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<Country> {
        self.with_coutry_group_where(|query| {
            query
                .query_mut()
                .is_eq(CountryGroupRepo::col_tournament_id(), tournament_id)
                .is_eq(CountryGroupRepo::col_is_out(), false);
        });
        self.paginate(page).await
    }

    pub async fn paginate_out(
        &mut self,
        tournament_id: ArcUuid7,
        page: Option<PaginateBuilder>,
    ) -> PaginateResult<Country> {
        self.with_coutry_group_where(|query| {
            query
                .query_mut()
                .is_eq(CountryGroupRepo::col_tournament_id(), tournament_id)
                .is_eq(CountryGroupRepo::col_is_out(), true);
        });
        self.paginate(page).await
    }
}
