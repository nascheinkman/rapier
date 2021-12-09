use rapier3d::na::RealField;
use rapier3d::prelude::*;
use rapier_testbed3d::Testbed;

pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut joints = JointSet::new();

    /*
     * The ground
     */
    let ground_size = 5.0;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::new_static()
        .translation(vector![0.0, -ground_height, 0.0])
        .build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size).build();
    colliders.insert_with_parent(collider, handle, &mut bodies);

    let pos = vector![0.0, 7.0, 0.0];
    let rb = RigidBodyBuilder::new_dynamic().translation(pos).build();
    let center_cube = bodies.insert(rb);
    let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5).build();
    colliders.insert_with_parent(collider, center_cube, &mut bodies);

    let distance = 5.0;

    for i in 0..4 {
        let angle_i = (i as Real) * Real::frac_pi_2();
        let dpos = vector![0.0, distance * angle_i.sin(), distance * angle_i.cos()];

        let rb = RigidBodyBuilder::new_dynamic()
            .translation(pos + dpos)
            .build();
        let cube = bodies.insert(rb);
        let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5).build();
        colliders.insert_with_parent(collider, cube, &mut bodies);

        /*
         * Spring Joint
         */
        let spring = SpringJoint::new(point![0.0, 0.0, 0.0], point![0.0, 0.0, 0.0], 4.0, 25.0, 0.0);
        let _spring = joints.insert(center_cube, cube, spring);
    }

    for i in 0..2 {
        let angle_i = (i as Real) * Real::pi();
        let rb = RigidBodyBuilder::new_dynamic()
            .translation(pos + vector![distance * angle_i.cos(), 0.0, 0.0])
            .build();
        let cube = bodies.insert(rb);
        let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5).build();
        colliders.insert_with_parent(collider, cube, &mut bodies);

        /*
         * Spring Joint
         */
        let spring = SpringJoint::new(point![0.0, 0.0, 0.0], point![0.0, 0.0, 0.0], 4.0, 25.0, 0.0);
        let _spring = joints.insert(center_cube, cube, spring);
    }

    /*
     * Set up the testbed.
     */
    testbed.set_world_with_params(bodies, colliders, joints, Vector::zeros(), ());
    testbed.look_at(point![15.0, 9.0, -6.0], point![0.0, 7.0, 0.0]);
}
