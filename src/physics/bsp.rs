use bevy::{prelude::{*, shape::Box}, render::render_resource::PrimitiveTopology, utils::HashMap};

use crate::{
    geometry::intersections::Intersection,
    geometry::shapes::Plane,
};

use super::physmat::PhysMat;

pub type BspNodeId = usize;
pub type PlaneId = usize;

#[derive(Debug, Clone)]
pub enum BspNode {
    SolidLeaf(Handle<PhysMat>),
    EmptyLeaf(Handle<PhysMat>),
    Parent {
        plane_id: PlaneId,
        front: BspNodeId,
        back: BspNodeId,
    },
}

impl BspNode {
    fn is_leaf(&self) -> bool {
        matches!(self, Self::SolidLeaf(_) | Self::EmptyLeaf(_))
    }
}

pub enum BspCollision {
    Hit {
        empty_mats: Vec<Handle<PhysMat>>,
        solid_mat: Handle<PhysMat>,
        normal: Vec3,
        point: Vec3,
    },
    Miss {
        empty_mats: Vec<Handle<PhysMat>>,
    },
}

#[derive(Resource)]
pub struct MapBsp {
    map_size: f32,
    nodes: Vec<BspNode>,
    planes: Vec<Plane>,
}

impl MapBsp {
    // Return an empty BspNone with only one empty node handling a physical material.
    pub fn empty(mat: Handle<PhysMat>, size: f32) -> (Self, BspNodeId) {
        let mut mats = HashMap::new();
        mats.insert(0, mat.clone());
        (
            Self {
                nodes: vec![BspNode::EmptyLeaf(mat.clone())],
                planes: vec![],
                map_size: size,
            },
            0,
        )
    }

    fn intersect<'s, 'o, I, O>(&'s self, other: O) -> I
    where
        I: Intersection<'s, 'o, Lhs = &'s Self, Rhs = O>,
        O: 'o,
    {
        I::intersection(self, other)
    }

    // By convention the root node is at index 0.
    pub fn root_node_id(&self) -> BspNodeId {
        0
    }

    // Retrive a node by its index.
    pub fn get_node(&self, id: BspNodeId) -> Option<&BspNode> {
        self.nodes.get(id)
    }

    pub fn get_plane(&self, id: PlaneId) -> Plane {
        *self
            .planes
            .get(id)
            .expect("A plane was referenced but does not exist anymore.")
    }

    fn insert_node(&mut self, node: BspNode) -> BspNodeId {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    fn insert_plane(&mut self, plane: Plane) -> PlaneId {
        let id = self.planes.len();
        self.planes.push(plane);
        id
    }

    fn swap_nodes(&mut self, id1: BspNodeId, id2: BspNodeId) {
        self.nodes.swap(id1, id2);
    }

    pub fn update_node<F>(&mut self, id: BspNodeId, update_fn: F)
    where
        F: FnOnce(&mut BspNode),
    {
        self.nodes.get_mut(id).map(update_fn);
    }

    // Cuts the given leaf node in two returning the front and back nodes id. The original leaf
    // node is set as the front node.
    pub fn cut(
        &mut self,
        id: BspNodeId,
        plane: Plane,
        back_mat: Handle<PhysMat>,
    ) -> Option<(BspNodeId, BspNodeId)> {
        let front_node_id = {
            // Check if node to cut is a leaf node and retrieve it.
            let node = self.nodes.get(id).filter(|n| n.is_leaf())?;

            // Creates the new node.
            let front_node = match node {
                BspNode::EmptyLeaf(_) => BspNode::SolidLeaf(back_mat),
                BspNode::SolidLeaf(_) => BspNode::EmptyLeaf(back_mat),
                _ => unreachable!(),
            };
            self.insert_node(front_node)
        };

        // Replaces the current node by the provided parent and insert the old node as the new parent
        // back child. Returns the child node new id.
        let new_parent = BspNode::Parent {
            plane_id: self.insert_plane(plane),
            front: front_node_id,
            back: 0, // Resolved just below.
        };
        let back_node_id = self.insert_node(new_parent);
        self.nodes.swap(id, back_node_id);

        self.update_node(id, |n| match n {
            BspNode::Parent { back, .. } => *back = back_node_id,
            _ => unreachable!(),
        });

        Some((front_node_id, back_node_id))
    }

    pub fn plane(air_mat: Handle<PhysMat>, ground_mat: Handle<PhysMat>, size: f32) -> Self {
        let (mut bsp, root_id) = MapBsp::empty(air_mat, Box::new(size, size, size));
        let (_ground, _air) = bsp
            .cut(
                root_id,
                Plane {
                    origin: Vec3::ZERO,
                    normal: Vec3::Y,
                },
                ground_mat,
            )
            .unwrap();
        bsp
    }
}

pub fn setup_map(
    mut commands: Commands,
    mut phys_mats: ResMut<Assets<PhysMat>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_mat = phys_mats.add(PhysMat::DEFAULT_GROUND_PHYS_MAT);
    let air_mat = phys_mats.add(PhysMat::DEFAULT_AIR_PHYS_MAT);

    commands.insert_resource(MapBsp::plane(air_mat, ground_mat, 200.0));
    let mesh = meshes.add(
        shape::Plane {
            size: 100.0,
            subdivisions: 0,
        }
        .into(),
    );
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });
    commands.spawn((PbrBundle {
        mesh,
        material,
        ..default()
    },));
}

#[derive(Clone, Copy)]
enum PlaneSide {
    Front(usize),
    Back(usize),
}

// Algorithm to generate the mesh from the bsp tree.
// Perform a PostOrder Traversal. Keep the oriented planes from the height above leafs.
// Flip the normal for each plane depending on the side it has or discard it if it only has
// EmptyLeafs or SolidLeafs.
// Clip the plane by the Map bounding box. Then clip the polygon going up the Bsp.
// Once at root its finished.
impl Into<Mesh> for &MapBsp {
    fn into(self) -> Mesh {
        let mut nodes = vec![(vec![], self.root_node_id())];
        let mut surfaces = vec![];
        while let Some((mut planes, node)) = nodes
            .pop()
            .and_then(|(ps, nid)| Some((ps, self.get_node(nid)?)))
        {
            match node {
                BspNode::Parent {
                    plane_id,
                    front,
                    back,
                } => {
                    let mut fp = planes.clone();
                    let mut bp = planes;
                    fp.push(PlaneSide::Front(*plane_id));
                    bp.push(PlaneSide::Back(*plane_id));
                    nodes.push((fp, *front));
                    nodes.push((bp, *back));
                }
                BspNode::EmptyLeaf(_) => {}
                BspNode::SolidLeaf(_) => {
                    surfaces.push(planes.reverse());
                }
            }
        }

        let mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh
    }
}
