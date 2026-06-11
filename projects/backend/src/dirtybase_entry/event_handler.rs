use dirtybase_contract::prelude::Observable;

use crate::dirtybase_entry::{
    TipConfig,
    email::EmailSender,
    model::{
        user::UserCreated,
        user_validation::{UserValidation, UserValidationRepo, ValidationPurpose},
    },
};

pub(crate) async fn setup() {
    UserCreated::subscribe(|event, ctx| async move {
        let mut user_validation_repo = if let Ok(repo) = ctx.get::<UserValidationRepo>().await {
            repo
        } else {
            tracing::error!("could not get user validation repo");
            return event;
        };

        let tip_config = if let Ok(c) = ctx.get_config::<TipConfig>("tip_config").await {
            c
        } else {
            tracing::error!("could not get application config");
            return event;
        };

        if !tip_config.email_enabled {
            return event;
        }

        if let Ok(email_sender) = ctx.get::<EmailSender>().await {
            // TODO: Some of these values should come from the configuration
            let content =
                if let Ok(mut content) = std::fs::read_to_string("email_template/signup.html") {
                    let validation_record = UserValidation::new(
                        event.user_ref().id.clone().unwrap(),
                        ValidationPurpose::Email,
                    );

                    let record =
                        if let Ok(record) = user_validation_repo.insert(validation_record).await {
                            record
                        } else {
                            tracing::error!("could not save user validation token");
                            return event;
                        };

                    let token = record.token();

                    let user_id = &event.user_ref().id.clone().unwrap_or_default();
                    let username = if let Some(actor) = &event.user_ref().actor {
                        actor.username().to_string()
                    } else {
                        "Unknown".to_string()
                    };
                    let validation_link =
                        format!("{}?token={}", &tip_config.user_validation_url, token);

                    let replace = [
                        ("{{username}}", username),
                        ("{{validation_link}}", validation_link),
                    ];
                    for (ph, to) in replace {
                        content = content.replace(ph, &to);
                    }
                    content
                } else {
                    tracing::error!("could not load signup email template");
                    return event;
                };

            // TODO: handle the error and log it!!!!
            _ = email_sender
                .send(
                    "noreply@mansartipping.com",
                    event.user_ref().email.as_str(),
                    "Welcome to the platform!!!!",
                    &content,
                )
                .await;
        }
        event
    })
    .await;
    // setup code here...
}
