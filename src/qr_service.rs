use qrcodegen::*;
use lodepng::*;


pub struct QRService {

}

impl QRService {
    pub fn create_svg_data(self: &mut QRService, data: &str) -> Result<String, String> {
        let qr = match QrCode::encode_text(data,
                                     QrCodeEcc::Medium) {
            Ok(data) => data,
            Err(e) => {
                println!("{}", e);
                return Err(String::from("Data was too long!"));
            }
        };

        return Ok(qr.to_svg_string(1));
    }
}

