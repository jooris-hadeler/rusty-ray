use std::ops::Index;

use crate::{aabb::Aabb, interval::Interval, ray::Ray, scene::ObjectId};

#[derive(Debug)]
/// A bounding volume hierarchy for a scene.
pub struct Bvh {
    /// The nodes in the BVH.
    nodes: Vec<BvhNode>,
    /// The root node of the BVH.
    root: Option<NodeId>,
}

impl Bvh {
    /// Creates a new bounding volume hierarchy for the given bounding boxes.
    pub fn new(mut objects: Vec<(ObjectId, Aabb)>) -> Self {
        let mut nodes = Vec::new();

        let mut root = None;
        if !objects.is_empty() {
            let object_count = objects.len();

            // Recursively build the BVH tree
            root = Some(Self::build_tree(
                &mut nodes,
                objects.as_mut_slice(),
                0,
                object_count,
            ));
        }

        Self { nodes, root }
    }

    /// Builds the BVH by splitting the objects into two groups based on the axis with the largest extent.
    /// Returns the id of the created node.
    fn build_tree(
        nodes: &mut Vec<BvhNode>,
        objects: &mut [(ObjectId, Aabb)],
        start: usize,
        end: usize,
    ) -> NodeId {
        let span = end - start;

        match span {
            1 => {
                // Create a leaf node
                let (object_id, _) = objects[start];
                let node = BvhNode::Leaf(object_id);
                nodes.push(node);
                NodeId(nodes.len() - 1)
            }
            _ => {
                // Calculate the bounding box containing all objects
                let mut bounding_box = Aabb::EMPTY;
                for (_, bb) in objects[start..end].iter() {
                    bounding_box.grow(bb);
                }

                // Find the axis with the largest extent
                let axis = bounding_box.largest_axis();

                // Sort the objects based on the axis with the largest extent
                objects[start..end].sort_by(|(_, bb1), (_, bb2)| {
                    bb1.component(axis)
                        .start
                        .partial_cmp(&bb2.component(axis).start)
                        .unwrap()
                });

                // Split the objects into two groups
                let mid = start + span / 2;

                // Create a sub nodes
                let left = Self::build_tree(nodes, objects, start, mid);
                let right = Self::build_tree(nodes, objects, mid, end);

                // Create a node
                nodes.push(BvhNode::Branch {
                    left,
                    right,
                    bounding_box,
                });
                NodeId(nodes.len() - 1)
            }
        }
    }

    /// Checks for intersections between the ray and the objects in the scene.
    /// Returns a list of object IDs that were hit by the ray.
    pub fn hit(&self, ray: &Ray, time: Interval) -> Option<Vec<ObjectId>> {
        let mut hit_objects = Vec::new();

        // Start at the root node or return early if there is no root node
        let mut stack = vec![self.root?];

        while let Some(node_id) = stack.pop() {
            let node = &self[node_id];

            match node {
                BvhNode::Leaf(object_id) => {
                    hit_objects.push(*object_id);
                }
                BvhNode::Branch {
                    left,
                    right,
                    bounding_box,
                } => {
                    if bounding_box.hit(ray, time) {
                        stack.push(*left);
                        stack.push(*right);
                    }
                }
            }
        }

        if hit_objects.is_empty() {
            None
        } else {
            Some(hit_objects)
        }
    }
}

impl Index<NodeId> for Bvh {
    type Output = BvhNode;

    fn index(&self, idx: NodeId) -> &Self::Output {
        &self.nodes[idx.0]
    }
}

#[derive(Debug, Clone, Copy)]
/// Identifier for a node in the BVH.
struct NodeId(usize);

#[derive(Debug)]
/// A node in the BVH.
enum BvhNode {
    /// A leaf node containing an object ID.
    Leaf(ObjectId),
    /// A branch node containing two child nodes and a bounding box.
    Branch {
        left: NodeId,
        right: NodeId,
        bounding_box: Aabb,
    },
}
