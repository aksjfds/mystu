pub fn random_code() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::rng();

    (0..8)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/* --------------------------------- // 邮件发送 -------------------------------- */
const EMAIL_PASSWORD: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().ok();

    let email_password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD is not Provided");
    email_password
});

pub fn stu<T>(to_email: &str, content: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: lettre::message::IntoBody,
{
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    // 配置邮件服务器
    let smtp_credentials = Credentials::new(
        String::from("22qyli13@stu.edu.cn"), // 你的邮箱地址
        EMAIL_PASSWORD.to_string(),          // 你的邮箱密码或应用专用密码
    );

    // 使用 STARTTLS 明确配置
    let mailer = SmtpTransport::starttls_relay("smtp.partner.outlook.cn")?
        .port(587)
        .credentials(smtp_credentials)
        .build();

    // 配置邮件内容
    let email = Message::builder()
        .from("AKSJFDS <22qyli13@stu.edu.cn>".parse()?)
        .to(to_email.parse()?)
        .subject("MyStu 验证码")
        .header(ContentType::TEXT_PLAIN)
        .body(content)?;

    // 发送邮件
    let result = mailer.send(&email);
    match result {
        Ok(_) => println!("邮件发送成功"),
        Err(e) => eprintln!("邮件发送失败: {:?}", e),
    }

    Ok(())
}
