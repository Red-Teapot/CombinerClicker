use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageType};

pub struct GameAssets {
    pub font: Handle<Font>,

    pub coin: Handle<Image>,
    pub spot: Handle<Image>,

    pub miner: Handle<Image>,
    pub collector: Handle<Image>,

    pub adder: Handle<Image>,
    pub multiplicator: Handle<Image>,
}

pub fn load_assets(mut commands: Commands,
                   mut font_assets: ResMut<Assets<Font>>,
                   mut image_assets: ResMut<Assets<Image>>) {
    commands.insert_resource(GameAssets {
        font: make_font_ttf(&mut font_assets, include_bytes!("../assets/VarelaRound/VarelaRound-Regular.ttf")),
        coin: make_image_png(&mut image_assets, include_bytes!("../assets/coin.png")),
        spot: make_image_png(&mut image_assets, include_bytes!("../assets/spot.png")),
        miner: make_image_png(&mut image_assets, include_bytes!("../assets/miner.png")),
        collector: make_image_png(&mut image_assets, include_bytes!("../assets/collector.png")),
        adder: make_image_png(&mut image_assets, include_bytes!("../assets/adder.png")),
        multiplicator: make_image_png(&mut image_assets, include_bytes!("../assets/multiplicator.png")),
    });
}

fn make_font_ttf(font_assets: &mut ResMut<Assets<Font>>, data: &[u8]) -> Handle<Font> {
    font_assets.add(Font::try_from_bytes(data.to_vec()).unwrap())
}

fn make_image_png(image_assets: &mut ResMut<Assets<Image>>, data: &[u8]) -> Handle<Image> {
    image_assets.add(Image::from_buffer(data, ImageType::Extension("png"), CompressedImageFormats::NONE, false).unwrap())
}
