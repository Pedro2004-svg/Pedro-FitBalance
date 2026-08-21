use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use std::env;

pub async fn send_confirmation_email(to_email: &str, codigo: i32) -> Result<()> {
    let api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY no está definida");
    let resend = Resend::new(&api_key);

    let from = "FitBalance <onboarding@resend.dev>";
    let to = [to_email];
    let subject = "Tu código de verificación";
    let text = format!("Tu código de verificación es: {codigo}\n\nExpira en 10 minutos.");

    let email = CreateEmailBaseOptions::new(from, to, subject).with_text(&text);

    resend.emails.send(email).await?;

    Ok(())
}


pub async fn send_user_changes_email(to_email: &str) -> Result<()> {
    let api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY no está definida");
    let resend = Resend::new(&api_key);

    let from = "FitBalance <onboarding@resend.dev>";
    let to = [to_email];
    let subject = "Cambio de usuario en al cuenta";
    let text = format!("Tu usuario ha sido modificado. \n Si no has sido tu, porfavor contacta con el administrador");

    let email = CreateEmailBaseOptions::new(from, to, subject).with_text(&text);

    resend.emails.send(email).await?;

    Ok(())
}


pub async fn send_pass_changes_email(to_email: &str) -> Result<()> {
    let api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY no está definida");
    let resend = Resend::new(&api_key);

    let from = "FitBalance <onboarding@resend.dev>";
    let to = [to_email];
    let subject = "Cambio de contraseña en al cuenta";
    let text = format!("Tu contraseña ha sido modificada. \n Si no has sido tu, porfavor contacta con el administrador");

    let email = CreateEmailBaseOptions::new(from, to, subject).with_text(&text);

    resend.emails.send(email).await?;

    Ok(())
}


pub async fn send_email_changes_email(to_email: &str) -> Result<()> {
    let api_key = env::var("RESEND_API_KEY").expect("RESEND_API_KEY no está definida");
    let resend = Resend::new(&api_key);

    let from = "FitBalance <onboarding@resend.dev>";
    let to = [to_email];
    let subject = "Cambio de email en al cuenta";
    let text = format!("Tu email ha sido modificado. \n Si no has sido tu, porfavor contacta con el administrador");

    let email = CreateEmailBaseOptions::new(from, to, subject).with_text(&text);

    resend.emails.send(email).await?;

    Ok(())
}