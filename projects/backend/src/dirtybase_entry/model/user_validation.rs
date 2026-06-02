use std::fmt::Display;

use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{
            ArcUuid7, CreatedAtField, DateTimeField, DeletedAtField, StringField, UpdatedAtField,
        },
    },
    db_macro::DirtyTable,
    helper::{base64, random::random_string, time::now},
};
use dirtybase_common::anyhow;

use crate::dirtybase_entry::model::user::User;

#[derive(Debug, Clone, DirtyTable)]
#[dirty(id_not_auto, timestamp, soft_deletable)]
pub struct UserValidation {
    #[dirty(rel(kind = "belongs_to"))]
    pub(crate) user: Option<User>,
    pub(crate) id: ArcUuid7,
    pub(crate) user_id: ArcUuid7,
    token: StringField, // The full token is generated "record_id|token" and base64 encoded
    pub(crate) purpose: StringField,
    pub(crate) expires: DateTimeField,
    pub(crate) created_at: CreatedAtField,
    pub(crate) updated_at: UpdatedAtField,
    pub(crate) deleted_at: DeletedAtField,
}

impl UserValidationRepo {
    pub async fn validate(&mut self, token: &str) -> anyhow::Result<bool> {
        let bytes =
            base64::url_decode(token).map_err(|e| anyhow::anyhow!("invalid token: {}", e))?;
        let token_string =
            String::from_utf8(bytes).map_err(|e| anyhow::anyhow!("invalid token: {}", e))?;

        let pieces = token_string
            .split('|')
            .filter(|e| e.trim().len() > 6)
            .take(2)
            .map(String::from)
            .collect::<Vec<String>>();

        if pieces.len() < 2 {
            return Ok(false);
        }

        self.builder
            .is_eq(Self::col_id(), pieces[0].clone())
            .is_eq(Self::col_token(), pieces[1].clone())
            .gt_or_eq(Self::col_expires(), now().as_datetime());

        match self.one().await {
            Ok(Some(record)) => {
                if let Err(e) = self.destroy(record).await {
                    tracing::error!("could not deleted validated user validation: {}", e);
                }
                return Ok(true);
            }
            Ok(None) => return Ok(false),
            Err(e) => {
                tracing::error!("error fetching user validation: {}", e);
                return Ok(false);
            }
        };
    }
}

impl Default for UserValidation {
    fn default() -> Self {
        Self {
            expires: now().add_hours(24).as_datetime(),
            user: None,
            id: ArcUuid7::default(),
            user_id: ArcUuid7::default(),
            token: random_string(6).into(),
            purpose: String::new().into(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

impl UserValidation {
    pub fn new(user_id: ArcUuid7, purpose: ValidationPurpose) -> Self {
        let mut instance = Self::default();

        instance.purpose = purpose.to_string().into();
        instance.user_id = user_id;

        instance
    }

    pub fn token(&self) -> String {
        let token = format!("{}|{}", &self.id, &self.token);
        base64::url_encode(token.as_bytes())
    }
}

#[derive(Debug, Default)]
pub enum ValidationPurpose {
    #[default]
    Email,
    PasswordReset,
}

impl Display for ValidationPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Email => "email",
                Self::PasswordReset => "password_reset",
            }
        )
    }
}

impl From<String> for ValidationPurpose {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "email" => Self::Email,
            "password_reset" => Self::PasswordReset,
            _ => Self::Email,
        }
    }
}

impl From<ValidationPurpose> for FieldValue {
    fn from(value: ValidationPurpose) -> Self {
        FieldValue::String(value.to_string())
    }
}

impl From<FieldValue> for ValidationPurpose {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(v) => v.into(),
            _ => Self::Email,
        }
    }
}
