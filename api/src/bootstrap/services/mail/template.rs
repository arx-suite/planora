pub fn email_verify(code: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify Your Email</title>
</head>
<body style="font-family: Arial, sans-serif; background-color: #f4f4f4; padding: 20px; margin: 0;">
    <table role="presentation" cellspacing="0" cellpadding="0" border="0" align="center" width="100%" style="max-width: 600px; background-color: #ffffff; border-radius: 8px; overflow: hidden;">
        <tr>
            <td style="padding: 40px 30px; text-align: center;">
                <h1 style="font-size: 24px; color: #333333; margin: 0 0 20px;">Verify Your Email</h1>
                <p style="font-size: 16px; color: #555555; margin: 0 0 30px;">Use the code below to complete your verification.</p>
                
                <div style="background-color: #f8f9fa; border: 1px solid #ddd; padding: 20px; border-radius: 4px; display: inline-block;">
                    <span style="font-size: 32px; font-weight: bold; color: #007bff; letter-spacing: 5px;">${code}</span>
                </div>
                
                <p style="font-size: 14px; color: #888888; margin: 30px 0 0;">This code will expire in 15 minutes.</p>
            </td>
        </tr>
        <tr>
            <td style="background-color: #eeeeee; padding: 20px; text-align: center; font-size: 12px; color: #777777;">
                If you did not request this, please ignore this email.
            </td>
        </tr>
    </table>
</body>
</html>
"#
    )
}
