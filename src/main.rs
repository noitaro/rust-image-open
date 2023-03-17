use std::path::Path;
use image::GenericImageView;

fn main() {
    // 画像ファイルのパスを定義
    let image_path = Path::new("./image/FrWJHNxaYAQOG6I.jpg");

    // 画像を読み込み、イメージオブジェクトを作成
    let img = match image::open(image_path) {
        Ok(img) => img,
        Err(e) => panic!("Failed to load image: {}", e),
    };

    // 画像の幅と高さを取得
    let (width, height) = img.dimensions();
    println!("width: {}", width);
    println!("height: {}", height);

    // 画像のピクセルデータを取得
    let pixels = img.into_rgb8().into_vec();

    // 画像の処理
    // ...

    // ピクセルデータを元のフォーマットに戻す
    let img = image::RgbImage::from_vec(width, height, pixels).unwrap();

    // 画像を保存
    img.save("output.jpg").unwrap();
}
