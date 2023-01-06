use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy_ninepatch::{NinePatch, NinePatchBuilder};

#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub panel: (Handle<Image>, Handle<NinePatchBuilder>),
    pub locked: Handle<Image>,

    pub coin: Handle<Image>,
    pub spot: Handle<Image>,

    pub conveyor_up: Handle<Image>,
    pub conveyor_down: Handle<Image>,
    pub conveyor_left: Handle<Image>,
    pub conveyor_right: Handle<Image>,

    pub miner: Handle<Image>,
    pub collector: Handle<Image>,

    pub adder: Handle<Image>,
    pub multiplicator: Handle<Image>,
}

pub fn load_assets(mut commands: Commands,
                   mut font_assets: ResMut<Assets<Font>>,
                   mut image_assets: ResMut<Assets<Image>>,
                   mut nine_patches: ResMut<Assets<NinePatchBuilder>>) {
    commands.insert_resource(GameAssets {
        font: make_font_ttf(&mut font_assets, include_bytes!("../assets/VarelaRound/VarelaRound-Regular.ttf")),
        panel: make_9patch(&mut image_assets, &mut nine_patches, include_bytes!("../assets/panel.png"), 12),
        locked: make_image_png(&mut image_assets, include_bytes!("../assets/locked.png")),
        coin: make_image_png(&mut image_assets, include_bytes!("../assets/coin.png")),
        spot: make_image_png(&mut image_assets, include_bytes!("../assets/spot.png")),
        conveyor_up: make_image_png(&mut image_assets, include_bytes!("../assets/conveyor-up.png")),
        conveyor_down: make_image_png(&mut image_assets, include_bytes!("../assets/conveyor-down.png")),
        conveyor_left: make_image_png(&mut image_assets, include_bytes!("../assets/conveyor-left.png")),
        conveyor_right: make_image_png(&mut image_assets, include_bytes!("../assets/conveyor-right.png")),
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
    image_assets.add(Image::from_buffer(data, ImageType::Extension("png"), CompressedImageFormats::NONE, true).unwrap())
}

fn make_9patch(image_assets: &mut ResMut<Assets<Image>>,
               nine_patches: &mut ResMut<Assets<NinePatchBuilder>>,
               data: &[u8],
               offsets: u32) -> (Handle<Image>, Handle<NinePatchBuilder>) {
    let image = make_image_png(image_assets, data);

    let ninepatch = nine_patches.add(
        NinePatchBuilder::by_margins(offsets, offsets, offsets, offsets));

    (image, ninepatch)
}
