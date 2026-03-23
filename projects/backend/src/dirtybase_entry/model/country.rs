use chrono;
use dirtybase_app::{
    db::types::{ArcUuid7, CreatedAtField, DeletedAtField, StringField, UpdatedAtField},
    db_macro::DirtyTable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, DirtyTable, Serialize, Deserialize)]
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

    pub fn id(&self) -> Option<&ArcUuid7> {
        self.id.as_ref()
    }

    pub fn set_game_code(&mut self, game_code: &str) {
        self.game_code = game_code.to_string().into();
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string().into();
    }

    pub fn set_short(&mut self, short: &str) {
        self.short = short.to_string().into();
    }

    pub fn set_image(&mut self, image: &str) {
        self.image = image.to_string().into();
    }

    pub fn game_code(&self) -> &str {
        self.game_code.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn short(&self) -> &str {
        self.short.as_str()
    }

    pub fn image(&self) -> &str {
        self.image.as_str()
    }

    pub fn created_at(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.created_at.as_ref()
    }

    pub fn updated_at(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.updated_at.as_ref()
    }

    pub fn deleted_at(&self) -> Option<&chrono::DateTime<chrono::Utc>> {
        self.deleted_at.as_ref()
    }
}
