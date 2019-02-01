use qrcodegen::*;

pub struct QRService {}

impl QRService {
    pub fn new() -> QRService {
        QRService {}
    }

    pub fn create_svg_data(self: &mut QRService, data: &str) -> Result<String, String> {
        let qr = match QrCode::encode_text(data, QrCodeEcc::Medium) {
            Ok(data) => data,
            Err(e) => {
                info!("{}", e);
                return Err(String::from("Data was too long!"));
            }
        };

        return Ok(qr.to_svg_string(1));
    }
}
