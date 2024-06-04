use bevy::prelude::*;
use bitflags::bitflags;

bitflags! {
    // May need to upgrade this in the future.
    #[derive(Clone, Copy)]
    pub struct CollisionLayers: u8 {
        const NONE       = 0b00;
        const PLAYER     = 0b01;
        const PROJECTILE = 0b10;
    }
}

#[derive(TypePath, Asset, Clone, Copy)]
pub struct PhysMat {
    pub colision_layers: CollisionLayers,
    pub acceleration: f32,
    pub friction: f32,
    pub control: f32,
    pub perfect_control_max_angle: f32,

    pub dash_behaviour: SpecialBehaviour,
    pub jump_behaviour: SpecialBehaviour,
}

impl PhysMat {
    pub fn does_collide(&self, colision_layers: CollisionLayers) -> bool {
        (self.colision_layers & colision_layers).is_empty()
    }

    pub const DEFAULT_GROUND_PHYS_MAT: PhysMat = PhysMat {
        colision_layers: CollisionLayers::PLAYER | CollisionLayers::PROJECTILE,
        acceleration: 12.0,
        friction: 8.0,

        control: 150.0,
        perfect_control_max_angle: 0.0,

        dash_behaviour: SpecialBehaviour::RedirectAndBoost {
            normal_boost: Boost::SetIfLower(450.0),
            wish_boost: Boost::Add(20.0),
        },
        jump_behaviour: SpecialBehaviour::RedirectAndBoost {
            normal_boost: Boost::SetIfLower(450.0),
            wish_boost: Boost::None,
        },
    };

    pub const DEFAULT_WALL_PHYS_MAT: PhysMat = PhysMat {
        colision_layers: CollisionLayers::PLAYER | CollisionLayers::PROJECTILE,
        acceleration: 0.0,
        friction: 0.0,

        control: 150.0,
        perfect_control_max_angle: 0.0,

        dash_behaviour: SpecialBehaviour::RedirectAndBoost {
            normal_boost: Boost::SetIfLower(450.0),
            wish_boost: Boost::Add(20.0),
        },
        jump_behaviour: SpecialBehaviour::RedirectAndBoost {
            normal_boost: Boost::SetIfLower(450.0),
            wish_boost: Boost::None,
        },
    };

    pub const DEFAULT_AIR_PHYS_MAT: PhysMat = PhysMat {
        colision_layers: CollisionLayers::empty(),
        acceleration: 12.0,
        friction: 0.0,

        control: 150.0,
        perfect_control_max_angle: 0.0,

        dash_behaviour: SpecialBehaviour::RedirectOnly,
        jump_behaviour: SpecialBehaviour::BoostOnly {
            normal_boost: Boost::Add(225.0),
            wish_boost: Boost::None,
        },
    };
}

pub struct PhysMatBuilder {
    mat: PhysMat,
}

#[derive(Clone, Copy)]
enum SpecialBehaviour {
    // Only redirect the speed without modifying it.
    RedirectOnly,

    // Redirect the speed and add a bit to it modifying it.
    RedirectAndBoost {
        normal_boost: Boost,
        wish_boost: Boost,
    },

    BoostOnly {
        normal_boost: Boost,
        wish_boost: Boost,
    },
}

impl SpecialBehaviour {
    // Not sure.
    fn apply(&self, vel: Vec3, normal: Vec3, wish: Vec3) -> Vec3 {
        match *self {
            SpecialBehaviour::RedirectOnly => vel.length() * wish.normalize(),
            SpecialBehaviour::RedirectAndBoost {
                normal_boost,
                wish_boost,
            } => {
                let redirected = vel.length() * wish.normalize();
                normal_boost.apply(normal) + wish_boost.apply(redirected)
            }
            SpecialBehaviour::BoostOnly {
                normal_boost,
                wish_boost,
            } => normal_boost.apply(normal * normal.dot(vel)) + wish_boost.apply(vel),
        }
    }
}

#[derive(Clone, Copy)]
enum Boost {
    // Don't modify the speed at all
    None,
    Add(/* speed */ f32),
    SetIfLower(/* speed */ f32),
}

impl Boost {
    fn apply(&self, vel: Vec3) -> Vec3 {
        match *self {
            Boost::None => vel,
            Boost::Add(v) => vel + v * vel.normalize(),
            Boost::SetIfLower(v) => {
                if vel.length() < v {
                    vel.normalize() * v
                } else {
                    vel
                }
            }
        }
    }
}
