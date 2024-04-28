pub mod player;

pub mod col_layers {
    use bevy_rapier3d::geometry::Group;

    pub const PLAYERS: Group = Group::GROUP_1;
    pub const HURTBOXES: Group = Group::GROUP_2;
    pub const HITBOXES: Group = Group::GROUP_3;
    pub const ENVIRONEMENT: Group = Group::GROUP_4;
}
