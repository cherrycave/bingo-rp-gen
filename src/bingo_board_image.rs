use photon_rs::{
    PhotonImage,
    multiple::watermark,
    native::open_image,
    transform::{SamplingFilter, resize},
};

pub fn generate_bingo_board(items: Vec<String>, board_size: u32) -> PhotonImage {
    let padding = 10;

    let mut img = open_image("background.png").expect("Failed to load backgroud image");

    let bg_width = img.get_width();
    let item_width = (bg_width - (padding * (board_size + 1))) / board_size;

    let images = items
        .iter()
        .map(|item| format!("textures/{}.png", item.to_lowercase()))
        .collect::<Vec<String>>()
        .iter()
        .map(|file_path| {
            let image = open_image(file_path).unwrap_or_else(|_| panic!("Failed to load image: {}", file_path));
            resize(&image, item_width, item_width, SamplingFilter::CatmullRom)
        })
        .collect::<Vec<PhotonImage>>();

    for i in 0..board_size {
        for j in 0..board_size {
            let x = (padding * (i + 1)) + (item_width * i);
            let y = (padding * (j + 1)) + (item_width * j);
            watermark(&mut img, &images[i as usize + (j * board_size) as usize], x as i64, y as i64);
        }
    }

    img
}
