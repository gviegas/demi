//! Blend-weight skinning.

// TODO: Skin instancing.

use std::io;
use std::mem;

use crate::linear::Mat4;

/// Skin.
#[derive(Debug)]
pub struct Skin {
    joints: Vec<Joint>,
    // This field defines the order of insertion for
    // joint nodes.
    jnt_hier: Vec<u16>,
}

impl Skin {
    /// Returns a reference to the skin's [`Joint`]s.
    pub fn joints(&self) -> &[Joint] {
        &self.joints
    }
}

/// Skin joint.
#[derive(Debug)]
pub struct Joint {
    name: String,
    jm: Mat4<f32>,
    ibm: Option<Mat4<f32>>,
    // Relative to `Skin.joints`.
    prev_slot: Option<u16>,
}

impl Joint {
    /// Returns the name of this joint.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the joint's matrix.
    pub fn joint_matrix(&self) -> &Mat4<f32> {
        &self.jm
    }

    /// Returns a reference to the inverse bind matrix,
    /// or `None` if it is the identity.
    pub fn inverse_bind_matrix(&self) -> Option<&Mat4<f32>> {
        self.ibm.as_ref()
    }

    /// Returns the slot containing the parent of this joint,
    /// or `None` if there is no previous joint.
    ///
    /// NOTE: This only pertains to direct connections between
    /// skin joints. Skins sourced from external node graphs
    /// may contain joint nodes that are connected indirectly
    /// through other nodes. Such relations are not preserved.
    pub fn prev_slot(&self) -> Option<u16> {
        self.prev_slot
    }
}

/// Skin builder.
pub struct Builder(Vec<Joint>);

impl Builder {
    /// Creates a new skin builder.
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Pushes a number of joints.
    ///
    /// A new joint is created for every element of the
    /// slice parameters, in order.
    /// All slices must have the same length.
    ///
    /// This method will fail if the total number of joints
    /// exceeds [`u16::MAX`] across all `push_joints`
    /// calls for a single skin.
    // TODO: Consider changing `name`'s type to `&[String]`.
    pub fn push_joints(
        &mut self,
        name: &[&str],
        jm: &[Mat4<f32>],
        ibm: &[Option<Mat4<f32>>],
        prev_slot: &[Option<u16>],
    ) -> io::Result<&mut Self> {
        let n = name.len();
        if n != jm.len() || n != ibm.len() || n != prev_slot.len() {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        } else if name.len() + self.0.len() > u16::MAX.into() {
            // TODO: More descriptive error.
            Err(io::Error::from(io::ErrorKind::Other))
        } else {
            for i in 0..n {
                self.0.push(Joint {
                    name: name[i].to_string(),
                    jm: jm[i],
                    ibm: ibm[i],
                    prev_slot: prev_slot[i],
                });
            }
            Ok(self)
        }
    }

    /// Creates the skin.
    ///
    /// This method consumes every pushed joint to create
    /// the skin. The order which a given joint was pushed
    /// identifies its slot in the [`Skin`].
    ///
    /// Fails if no joint has been pushed yet.
    pub fn create(&mut self) -> io::Result<Skin> {
        if !self.0.is_empty() {
            let jnt_hier = self.make_hier();
            Ok(Skin {
                joints: mem::take(&mut self.0),
                jnt_hier,
            })
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }

    /// Creates the joint hierarchy such that every joint
    /// comes before any of its decendants.
    ///
    /// This must be called before consuming `self.0`.
    fn make_hier(&self) -> Vec<u16> {
        #[derive(Copy, Clone)]
        struct Map {
            weight: u16,
            index: u16,
        }
        // `Map.weight == 0` means not yet seen.
        let mut map = vec![
            Map {
                weight: 0,
                index: 0
            };
            self.0.len()
        ];
        // Use an auxiliar stack to prevent deep,
        // reverse-sorted hierarchies from
        // degenerating the algorithm.
        let mut stack = vec![];

        for (i, x) in self.0.iter().enumerate() {
            let mut weight = 1u16;
            let mut prev_slot = x.prev_slot;
            while let Some(prev) = prev_slot {
                let prev = prev as usize;
                if map[prev].weight != 0 {
                    weight += map[prev].weight;
                    break;
                }
                stack.push(prev as u16);
                prev_slot = self.0[prev].prev_slot;
                weight += 1;
            }
            map[i].weight = weight;
            map[i].index = i as u16;
            for &i in &stack {
                weight -= 1;
                map[i as usize].weight = weight;
            }
            stack.clear();
        }

        map.sort_unstable_by(|&a, &b| a.weight.cmp(&b.weight));
        map.into_iter().map(|Map { index, .. }| index).collect()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Skin {
        fn check(
            &self,
            n: usize,
            name: &[&str],
            jm: &[Mat4<f32>],
            ibm: &[Option<Mat4<f32>>],
            prev_slot: &[Option<u16>],
        ) {
            assert_eq!(self.joints.len(), n);
            assert_eq!(self.jnt_hier.len(), n);
            for (i, x) in self.joints().iter().enumerate() {
                assert_eq!(x.name(), name[i]);
                assert_eq!(x.joint_matrix(), &jm[i]);
                assert_eq!(x.inverse_bind_matrix(), ibm[i].as_ref());
                assert_eq!(x.prev_slot(), prev_slot[i]);
            }
            self.check_hier()
        }

        // NOTE: `check` calls this.
        fn check_hier(&self) {
            let mut seen = vec![false; self.joints.len()];
            for &i in &self.jnt_hier {
                assert!(match self.joints[i as usize].prev_slot {
                    None => true,
                    Some(j) => seen[j as usize],
                });
                seen[i as usize] = true;
            }
        }
    }

    #[test]
    fn create_skin() {
        const N: usize = 10;

        let jm: [Mat4<f32>; N] = [
            Mat4::from(1.0),
            Mat4::from(2.0),
            Mat4::from(3.0),
            Mat4::from(4.0),
            Mat4::from(5.0),
            Mat4::from(6.0),
            Mat4::from(7.0),
            Mat4::from(8.0),
            Mat4::from(9.0),
            Mat4::from(10.0),
        ];

        let ibm: [Option<Mat4<f32>>; N] = [
            None,
            Some(Mat4::from(11.0)),
            Some(Mat4::from(12.0)),
            None,
            None,
            Some(Mat4::from(13.0)),
            None,
            Some(Mat4::from(14.0)),
            None,
            None,
        ];

        struct Case {
            name: [&'static str; N],
            prev_slot: [Option<u16>; N],
        }

        let cases = [
            Case {
                name: [
                    "a-b-a-a", "a-b-a", "a-b", "a-a", "a-a-a", "a", "b-a", "b-b", "b", "b-b-a",
                ],
                prev_slot: [
                    Some(1),
                    Some(2),
                    Some(5),
                    Some(5),
                    Some(3),
                    None,
                    Some(8),
                    Some(8),
                    None,
                    Some(7),
                ],
            },
            Case {
                name: [
                    "a", "b", "a-a", "a-b", "b-a", "b-b", "a-a-a", "a-b-a", "b-b-a", "a-b-a-a",
                ],
                prev_slot: [
                    None,
                    None,
                    Some(0),
                    Some(0),
                    Some(1),
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(5),
                    Some(7),
                ],
            },
            Case {
                name: [
                    "a-b-a-a", "b-b-a", "a-b-a", "a-a-a", "b-b", "b-a", "a-b", "a-a", "b", "a",
                ],
                prev_slot: [
                    Some(2),
                    Some(4),
                    Some(6),
                    Some(7),
                    Some(8),
                    Some(8),
                    Some(9),
                    Some(9),
                    None,
                    None,
                ],
            },
            Case {
                name: ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"],
                prev_slot: [None; N],
            },
            Case {
                name: [
                    "a",
                    "a-a",
                    "a-a-a",
                    "a-a-a-a",
                    "a-a-a-a-a",
                    "a-a-a-a-a-a",
                    "a-a-a-a-a-a-a",
                    "a-a-a-a-a-a-a-a",
                    "a-a-a-a-a-a-a-a-a",
                    "a-a-a-a-a-a-a-a-a-a",
                ],
                prev_slot: [
                    None,
                    Some(0),
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(7),
                    Some(8),
                ],
            },
            Case {
                name: [
                    "a-a-a-a-a-a-a-a-a-a",
                    "a-a-a-a-a-a-a-a-a",
                    "a-a-a-a-a-a-a-a",
                    "a-a-a-a-a-a-a",
                    "a-a-a-a-a-a",
                    "a-a-a-a-a",
                    "a-a-a-a",
                    "a-a-a",
                    "a-a",
                    "a",
                ],
                prev_slot: [
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(6),
                    Some(7),
                    Some(8),
                    Some(9),
                    None,
                ],
            },
        ];

        for i in cases {
            let skin = Builder::new()
                .push_joints(&i.name, &jm, &ibm, &i.prev_slot)
                .unwrap()
                .create()
                .unwrap();

            skin.check(N, &i.name, &jm, &ibm, &i.prev_slot);

            let skin = Builder::new()
                .push_joints(&i.name[..1], &jm[..1], &ibm[..1], &i.prev_slot[..1])
                .unwrap()
                .push_joints(&i.name[1..], &jm[1..], &ibm[1..], &i.prev_slot[1..])
                .unwrap()
                .create()
                .unwrap();

            skin.check(N, &i.name, &jm, &ibm, &i.prev_slot);

            let skin = Builder::new()
                .push_joints(
                    &i.name[..N - 1],
                    &jm[..N - 1],
                    &ibm[..N - 1],
                    &i.prev_slot[..N - 1],
                )
                .unwrap()
                .push_joints(
                    &i.name[N - 1..],
                    &jm[N - 1..],
                    &ibm[N - 1..],
                    &i.prev_slot[N - 1..],
                )
                .unwrap()
                .create()
                .unwrap();

            skin.check(N, &i.name, &jm, &ibm, &i.prev_slot);

            let skin = Builder::new()
                .push_joints(
                    &i.name[..N / 2],
                    &jm[..N / 2],
                    &ibm[..N / 2],
                    &i.prev_slot[..N / 2],
                )
                .unwrap()
                .push_joints(
                    &i.name[N / 2..],
                    &jm[N / 2..],
                    &ibm[N / 2..],
                    &i.prev_slot[N / 2..],
                )
                .unwrap()
                .create()
                .unwrap();

            skin.check(N, &i.name, &jm, &ibm, &i.prev_slot);
        }
    }

    impl Builder {
        fn push_sorted_joints(&mut self, mut depth: u16) -> &mut Self {
            depth = depth.clamp(1, u16::MAX);

            let mut name = Vec::with_capacity(depth as usize);
            let mut jm = Vec::with_capacity(depth as usize);
            let mut ibm = Vec::with_capacity(depth as usize);
            let mut prev_slot = Vec::with_capacity(depth as usize);

            name.push("");
            jm.push(Mat4::from(1.0));
            ibm.push(None);
            prev_slot.push(None);

            for i in 1..depth {
                name.push("");
                jm.push(Mat4::from(1.0));
                ibm.push(None);
                prev_slot.push(Some(i - 1));
            }

            self.push_joints(&name, &jm, &ibm, &prev_slot).unwrap()
        }

        fn push_reversed_joints(&mut self, mut depth: u16) -> &mut Self {
            depth = depth.clamp(1, u16::MAX);

            let mut name = Vec::with_capacity(depth as usize);
            let mut jm = Vec::with_capacity(depth as usize);
            let mut ibm = Vec::with_capacity(depth as usize);
            let mut prev_slot = Vec::with_capacity(depth as usize);

            for i in 0..depth - 1 {
                name.push("");
                jm.push(Mat4::from(1.0));
                ibm.push(None);
                prev_slot.push(Some(i + 1));
            }

            name.push("");
            jm.push(Mat4::from(1.0));
            ibm.push(None);
            prev_slot.push(None);

            self.push_joints(&name, &jm, &ibm, &prev_slot).unwrap()
        }
    }

    #[test]
    fn create_skin_sorted() {
        for i in [
            1, 2, 3, 4, 15, 31, 64, 127, 255, 256, 999, 2048, 32767, 65534,
        ] {
            let skin = Builder::new().push_sorted_joints(i).create().unwrap();
            skin.check_hier();
        }
    }

    #[test]
    fn create_skin_reversed() {
        for i in [
            1, 2, 3, 4, 15, 31, 64, 127, 255, 256, 999, 2048, 32767, 65534,
        ] {
            let skin = Builder::new().push_reversed_joints(i).create().unwrap();
            skin.check_hier();
        }
    }
}
