use dirtybase_app::{
    db::types::{ArcUuid7, CreatedAtField, DeletedAtField, StringField, UpdatedAtField},
    db_macro::DirtyTable,
};

#[derive(Debug, Default, Clone, DirtyTable)]
#[dirty(soft_deletable, id_not_auto, timestamp)]
pub struct Country {
    id: Option<ArcUuid7>,
    game_code: StringField,
    name: StringField,
    short: StringField,
    image: StringField,
    created_at: CreatedAtField,
    updated_at: UpdatedAtField,
    deleted_at: DeletedAtField,
}

impl Country {
    pub fn new(game_code: &str, name: &str, short: &str, image: &str) -> Self {
        Self {
            id: Some(ArcUuid7::default()),
            game_code: game_code.to_string().into(),
            name: name.to_string().into(),
            short: short.to_string().into(),
            image: image.to_string().into(),
            ..Default::default()
        }
    }
}
