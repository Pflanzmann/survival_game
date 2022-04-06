use bevy::prelude::*;
use crate::{CoinCount, Cointext};

pub fn spawn_text_system(
    mut commands: Commands,
    asset_loader: Res<AssetServer>,
    mut coin_counter: ResMut<CoinCount>,
) {
    coin_counter.number = 0;

    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Coins: ".to_string(),
            TextStyle {
                font: asset_loader.load("fonts/BodoniFLF-Roman.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    })
        .insert(Cointext)
        .id();
}

pub fn update_text_system(
    mut text_query: Query<&mut Text, With<Cointext>>,
    mut coin_counter: ResMut<CoinCount>,
) {
    if coin_counter.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{:.0}", coin_counter.number);
        }
    }
}