use std::any::type_name;
use std::collections::HashMap;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::{AssetServer, Commands, Component, Entity, Plugin, Res, Resource};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::App;
use crate::models::modifications::acid_puddle::AcidPuddle;
use crate::models::modifications::affects::affect_damage::AffectDamage;
use crate::models::modifications::affects::affect_health::AffectHealth;
use crate::models::modifications::affects::affect_hit_limit::AffectHitLimit;
use crate::models::modifications::affects::affect_move_speed::AffectMoveSpeed;
use crate::models::modifications::affects::affect_reload::AffectReload;
use crate::models::modifications::affects::affect_travel_range::AffectTravelRange;
use crate::models::modifications::affects::affect_unit_size::AffectUnitSize;
use crate::models::modifications::affects::projectile_affects::affect_projectile_damage::AffectProjectileDamage;
use crate::models::modifications::affects::projectile_affects::affect_projectile_hit_limit::AffectProjectileHitLimit;
use crate::models::modifications::affects::projectile_affects::affect_projectile_move_speed::AffectProjectileMoveSpeed;
use crate::models::modifications::affects::projectile_affects::affect_projectile_travel_range::AffectProjectileTravelRange;
use crate::models::modifications::affects::projectile_affects::affect_projectile_unit_size::AffectProjectileUnitSize;
use crate::models::modifications::blue_pill::BluePill;
use crate::models::modifications::burning_shot::BurningShot;
use crate::models::modifications::curve_shot::CurveShot;
use crate::models::modifications::death_ball::DeathBall;
use crate::models::modifications::descriptors::mod_name::ModName;
use crate::models::modifications::descriptors::mod_sprite_path::SpriteHandle;
use crate::models::modifications::descriptors::modification::Modification;
use crate::models::modifications::descriptors::price::Price;
use crate::models::modifications::descriptors::sprite_path_wrapper::SpritePathWrapper;
use crate::models::modifications::descriptors::tool_tip::ToolTip;
use crate::models::modifications::effects::effect_add_health::EffectAddHealth;
use crate::models::modifications::effects::effect_damage_health::EffectDamageHealth;
use crate::models::modifications::explosion_shot::ExplosionShot;
use crate::models::modifications::glass_cannon::GlassCannon;
use crate::models::modifications::gravity_shot::GravityShot;
use crate::models::modifications::grow_shot::GrowShot;
use crate::models::modifications::knock_back_shot::KnockBackShot;
use crate::models::modifications::lightning::Lightning;
use crate::models::modifications::low_cal::LowCal;
use crate::models::modifications::magnet::Magnet;
use crate::models::modifications::psy_rock::PsyRock;
use crate::models::modifications::radiation::Radiation;
use crate::models::modifications::shield::Shield;
use crate::models::modifications::slime::Slime;
use crate::models::modifications::split_shot::SplitShot;
use crate::models::modifications::sprinting::Sprinting;
use crate::models::modifications::turret::Turret;
use crate::models::unit_attributes::damage::Damage;
use crate::models::unit_attributes::health::Health;
use crate::models::unit_attributes::hit_limit::HitLimit;
use crate::models::unit_attributes::move_speed::MoveSpeed;
use crate::models::unit_attributes::travel_range::TravelRange;
use crate::util::read_file_to_string::read_file_to_string;

pub struct EntityBuilderPlugin;

/// A [Plugin] to use the [EntityBuilder] and register the used types at his creation.
///
/// Every [Component] that has to get resolved needs to be registered.
impl Plugin for EntityBuilderPlugin {
    fn build(&self, app: &mut App) {
        let mut entity_builder = EntityBuilder::new();

        entity_builder.register_component::<ModName>();
        entity_builder.register_component::<ToolTip>();
        entity_builder.register_component::<Price>();

        entity_builder.register_component::<AffectMoveSpeed>();
        entity_builder.register_component::<AffectDamage>();
        entity_builder.register_component::<AffectHealth>();
        entity_builder.register_component::<AffectReload>();
        entity_builder.register_component::<AffectUnitSize>();
        entity_builder.register_component::<AffectTravelRange>();
        entity_builder.register_component::<AffectHitLimit>();

        entity_builder.register_component::<AffectProjectileDamage>();
        entity_builder.register_component::<AffectProjectileHitLimit>();
        entity_builder.register_component::<AffectProjectileMoveSpeed>();
        entity_builder.register_component::<AffectProjectileTravelRange>();
        entity_builder.register_component::<AffectProjectileUnitSize>();

        entity_builder.register_component::<EffectAddHealth>();
        entity_builder.register_component::<EffectDamageHealth>();

        entity_builder.register_component::<TravelRange>();
        entity_builder.register_component::<HitLimit>();
        entity_builder.register_component::<Health>();
        entity_builder.register_component::<Damage>();
        entity_builder.register_component::<MoveSpeed>();

        entity_builder.register_component::<Modification>();
        entity_builder.register_component::<CurveShot>();
        entity_builder.register_component::<GrowShot>();
        entity_builder.register_component::<SplitShot>();
        entity_builder.register_component::<GravityShot>();
        entity_builder.register_component::<KnockBackShot>();
        entity_builder.register_component::<ExplosionShot>();
        entity_builder.register_component::<Lightning>();
        entity_builder.register_component::<AcidPuddle>();
        entity_builder.register_component::<BurningShot>();

        entity_builder.register_component::<GlassCannon>();
        entity_builder.register_component::<LowCal>();
        entity_builder.register_component::<BluePill>();
        entity_builder.register_component::<Sprinting>();
        entity_builder.register_component::<Turret>();
        entity_builder.register_component::<Slime>();
        entity_builder.register_component::<DeathBall>();
        entity_builder.register_component::<PsyRock>();
        entity_builder.register_component::<Radiation>();
        entity_builder.register_component::<Shield>();
        entity_builder.register_component::<Magnet>();

        app.insert_resource::<EntityBuilder>(entity_builder);
    }
}

/// [EntityBuilder] that spawns entities by the given json file,
///
/// Every Component has to have the same key as the name of the struct
///
/// ```
/// #
/// {
///   "CurveShot": null,
///   "Health": {
///     "base_amount": 5.0,
///     "boost_amount": 0.0
///   }
///   ...
/// }
/// ```
///
/// Components have to get registered and it is recommended to do so in the constructor of the
/// [EntityBuilder]. Components have to get registered like this:
/// ```
/// fn example_method() {
///     let mut entity_builder = EntityBuilder::new();
///
///     entity_builder.register_component::<ModName>();
///     entity_builder.register_component::<ToolTip>();
/// }
/// ```
#[derive(Default, Resource)]
pub struct EntityBuilder {
    map: HashMap<String, Box<dyn Fn(&mut EntityCommands, &Value) + Send + Sync>>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        EntityBuilder { map: HashMap::new() }
    }

    pub fn spawn_entity(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        config_path: &str,
    ) -> Entity {
        let mut entity = commands.spawn_empty();

        let my_string = read_file_to_string(config_path);
        let mut component_data_map: HashMap<String, serde_json::Value> = serde_json::from_str(&my_string).unwrap();

        match component_data_map.remove("SpritePathWrapper") {
            None => {}
            Some(object_data) => {
                let wrapper: SpritePathWrapper = serde_json::from_value(object_data.clone()).expect("Not well formatted string: {:#?}");
                entity.insert(SpriteHandle { handle: asset_server.load(&wrapper.path) });
            }
        };

        for (component_key, object_data) in component_data_map.iter() {
            match self.map.get(component_key) {
                None => { panic!("Type is not registered in the EntityBuilder -> [{}]", component_key) }
                Some(function) => function(&mut entity, object_data),
            };
        }

        entity.id()
    }

    pub fn register_component<T: DeserializeOwned + Component>(&mut self) {
        self.map.insert(
            String::from(type_name::<T>().split("::").last().unwrap()),
            Box::new(deserialize_component::<T>),
        );
    }
}

fn deserialize_component<T: DeserializeOwned + Component>(
    entity_commands: &mut EntityCommands,
    object_data: &Value,
) {
    entity_commands.insert(get_component::<T>(object_data.clone()));
}

fn get_component<T: DeserializeOwned + Component>(
    value: serde_json::Value
) -> T {
    serde_json::from_value(value).expect("Not well formatted string: ")
}

