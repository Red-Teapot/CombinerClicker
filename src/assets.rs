use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ninepatch::NinePatchBuilder;

#[derive(Resource, AssetCollection)]
pub struct Images {
    #[asset(path = "panel.png")]
    pub panel: Handle<Image>,
    #[asset(path = "locked.png")]
    pub locked: Handle<Image>,

    #[asset(path = "coin.png")]
    pub coin: Handle<Image>,
    #[asset(path = "spot.png")]
    pub spot: Handle<Image>,

    #[asset(path = "conveyor-up.png")]
    pub conveyor_up: Handle<Image>,
    #[asset(path = "conveyor-down.png")]
    pub conveyor_down: Handle<Image>,
    #[asset(path = "conveyor-left.png")]
    pub conveyor_left: Handle<Image>,
    #[asset(path = "conveyor-right.png")]
    pub conveyor_right: Handle<Image>,

    #[asset(path = "miner.png")]
    pub miner: Handle<Image>,
    #[asset(path = "collector.png")]
    pub collector: Handle<Image>,

    #[asset(path = "adder.png")]
    pub adder: Handle<Image>,
    #[asset(path = "multiplier.png")]
    pub multiplier: Handle<Image>,
}

#[derive(Resource, AssetCollection)]
pub struct Fonts {
    #[asset(path = "VarelaRound/VarelaRound-Regular.ttf")]
    pub varela: Handle<Font>,
}

#[derive(Resource)]
pub struct NinePatches {
    pub panel: Handle<NinePatchBuilder>,
}

impl FromWorld for NinePatches {
    fn from_world(world: &mut World) -> Self {
        let world_cell = world.cell();

        let mut ninepatches = world_cell
            .get_resource_mut::<Assets<NinePatchBuilder>>()
            .unwrap();

        let mut make_uniform = |margin: u32| {
            ninepatches.add(NinePatchBuilder::by_margins(margin, margin, margin, margin))
        };

        NinePatches {
            panel: make_uniform(12),
        }
    }
}
