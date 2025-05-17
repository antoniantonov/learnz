use image::{ImageBuffer, Luma, Rgb};
use qrcode::QrCode;

fn main() {
    // Generate the first QR code
    let image1 = QrCode::new(b"01234567")
        .unwrap()
        .render::<Luma<u8>>()
        .max_dimensions(300, 300)
        .build();
    image1.save("/tmp/qr1.png").unwrap();

    // Generate the second QR code
    let image2 = QrCode::new(b"01234568")
        .unwrap()
        .render::<Luma<u8>>()
        .max_dimensions(300, 300)
        .build();
    image2.save("/tmp/qr2.png").unwrap();

    // Get the dimensions of the QR codes
    let width = image1.width();
    let height = image1.height();

    // Create an output image buffer
    let mut output_image = ImageBuffer::new(width, height);

    // Compare the two QR codes and generate the output image
    for y in 0..height {
        for x in 0..width {
            let pixel1 = image1.get_pixel(x, y);
            let pixel2 = image2.get_pixel(x, y);

            let color = match (pixel1[0], pixel2[0]) {
                (0, 0) => Rgb([0u8, 0u8, 0u8]),       // Black if both are black
                (0, 0xff) => Rgb([255u8, 0u8, 0u8]),     // Red if only in the first
                (0xff, 0) => Rgb([0u8, 255u8, 0u8]),     // Green if only in the second
                (0xff, 0xff) => Rgb([255u8, 255u8, 255u8]), // White if neither
                _ => Rgb([128u8, 128u8, 128u8]),      // Default: Gray for unexpected cases
            };

            output_image.put_pixel(x, y, color);
        }
    }

    // Save the output image
    output_image.save("/tmp/qr_comparison.png").unwrap();
    println!("Comparison QR code saved to /tmp/qr_comparison.png");
}
