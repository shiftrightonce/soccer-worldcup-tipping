use std::sync::Arc;

use dirtybase_common::anyhow;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, SmtpTransport, Tokio1Executor, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::{authentication::Credentials, client::Tls},
};

#[async_trait::async_trait]
pub trait Transporter {
    async fn do_send(&self, msg: Message) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct EmailSender {
    transporter: Arc<dyn Transporter + Send + Sync + 'static>,
}

impl EmailSender {
    pub fn new(transporter: impl Transporter + Send + Sync + 'static) -> Self {
        Self {
            transporter: Arc::new(transporter),
        }
    }

    pub async fn send(
        self,
        from: &str,
        to: &str,
        subject: &str,
        message: &str,
    ) -> anyhow::Result<()> {
        let message = Message::builder()
            .from(Mailbox::new(
                Some("MansarTipping".to_string()),
                from.parse()
                    .map_err(|e| anyhow::anyhow!("could not parse 'from' email: {}", e))?,
            ))
            // .reply_to(Mailbox::new(
            //     Some("JJ Mansaray".to_string()),
            //     "jj@example.com".parse().unwrap(),
            // ))
            .to(Mailbox::new(
                None,
                to.parse()
                    .map_err(|e| anyhow::anyhow!("could not parse 'to' email: {}", e))?,
            ))
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(message.to_string())
            .map_err(|e| anyhow::anyhow!("error build email message: {}", e))?;

        self.transporter.do_send(message).await
    }
}

pub struct SmtpWrapper;

#[async_trait::async_trait]
impl Transporter for SmtpWrapper {
    async fn do_send(&self, msg: Message) -> anyhow::Result<()> {
        // TODO:  The transporter should be injected
        let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());
        // let mailer = SmtpTransport::relay("mail")
        //     .unwrap()
        //     .port(1025)
        //     .tls(Tls::None)
        //     // .credentials(creds)
        //     .build();

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay("mail")
                .unwrap()
                .port(1025)
                .tls(Tls::None)
                // .credentials(creds)
                .build();

        match mailer.send(msg).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
}
