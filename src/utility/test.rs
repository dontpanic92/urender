use super::*;

#[test]
fn vector3d_mul() {
    let v = Vector3D(1, 2, 3);
    let v2 = Vector3D(0, 0, 0);
    assert_eq!(v * 3, Vector3D(3, 6, 9));
    assert_eq!(v * 0, Vector3D(0, 0, 0));
    assert_eq!(3 * v2, Vector3D(0, 0, 0));
}

#[test]
fn vector3d_dot() {
    assert_eq!(Vector3D(1, 2, 3).dot(Vector3D(4, 5, 6)), 32.);
    assert_eq!(Vector3D(0, 0, 0).dot(Vector3D(4, 5, 6)), 0.);  
}

#[test]
fn vector3d_cross() {
    assert_eq!(Vector3D(1, 2, 3).cross(Vector3D(4, 5, 6)), Vector3D(-3, 6, -3));
    assert_eq!(Vector3D(0, 0, 0).cross(Vector3D(4, 5, 6)), Vector3D(0, 0, 0));  
}

#[test]
fn point3d_add() {
    assert_eq!(Point3D(1, 2, 3) + Vector3D(4, 5, 6), Point3D(5, 7, 9));
}

#[test]
fn point3d_sub() {
    assert_eq!(Point3D(1, 2, 3) - Point3D(4, 5, 6), Vector3D(-3, -3, -3));
}