use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use std::sync::Arc;

use super::MailResult;
use crate::common::utils;

const ENV_SMTP_IP: &str = "SMTP_HOST";
const ENV_SMTP_PORT: &str = "SMTP_PORT";
const ENV_SMTP_USER: &str = "SMTP_USERNAME";
const ENV_SMTP_PASSWORD: &str = "SMTP_PASSWORD";

#[derive(Debug, Clone)]
pub struct MailConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl MailConfig {
    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        Self {
            host,
            port,
            username,
            password,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MailService {
    config: Arc<MailConfig>,
}

impl MailService {
    const PLANORA_EMAIL: &str = "Planora <planora@arx.sbs>";

    pub fn new(config: MailConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    fn build_mailer(&self) -> AsyncSmtpTransport<Tokio1Executor> {
        let credentials =
            Credentials::new(self.config.username.clone(), self.config.password.clone());

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.config.host)
            .port(self.config.port)
            .credentials(credentials)
            .build();

        mailer
    }

    #[tracing::instrument(
        name = "mail.send_mail",
        skip(self),
        level = tracing::Level::DEBUG
    )]
    pub async fn send_mail(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str,
    ) -> MailResult<()> {
        let email = Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())?;

        let mailer = self.build_mailer();
        mailer.send(email).await?;
        Ok(())
    }

    #[inline]
    pub async fn send_mail_official(&self, to: &str, subject: &str, body: &str) -> MailResult<()> {
        self.send_mail(Self::PLANORA_EMAIL, to, subject, body).await
    }

    #[tracing::instrument(
        name = "mail.send_bulk_email",
        skip_all,
        level = tracing::Level::DEBUG
    )]
    pub async fn send_bulk_email(
        &self,
        recipients: Vec<(&str, &str, &str, &str)>,
    ) -> Vec<MailResult<()>> {
        use futures::future::join_all;

        let futures = recipients
            .into_iter()
            .map(|(from, to, subject, body)| self.send_mail(from, to, subject, body));

        join_all(futures).await
    }
}

#[tracing::instrument(
    name = "service.mail",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub fn init() -> MailService {
    let host = utils::get_env::<String>(ENV_SMTP_IP).unwrap().to_string();
    let port = utils::get_env::<u16>(ENV_SMTP_PORT).unwrap();
    let username = utils::get_env::<String>(ENV_SMTP_USER).unwrap();
    let password = utils::get_env::<String>(ENV_SMTP_PASSWORD).unwrap();

    let config = MailConfig::new(host, port, username, password);
    MailService::new(config)
}
