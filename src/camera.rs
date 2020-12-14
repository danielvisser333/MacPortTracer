use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Quaternion;

//Reimplementation of the camera
enum CameraMode{
    Free,
    ThirdPerson,
    FirstPerson,
}
pub struct Camera{
    current_mode : CameraMode,
    position : Vector3<f32>,
    direction_x : Quaternion<f32>,
    direction_y : Quaternion<f32>,
    direction_z : Quaternion<f32>,
    velocity : Vector3<f32>,
    resolution : Vector2<f32>,
    alpha : f32,
    beta : f32,
    gamma : f32,
    radv : f32,
    fov : f32,
    focus : f32,
    bokeh : f32,
    size : f32,
    radius : f32,
    smooth : f32,
    aspect_ratio : f32,
    i_frame : i32,
}
impl Camera{
    pub fn default(resolution : Vector2<f32>) -> Self{
        return Self{
            current_mode : CameraMode::Free,
            position : Vector3::new(0.0,0.0,0.0),
            direction_x : Quaternion::new(0.0,1.0,0.0,0.0),
            direction_y : Quaternion::new(0.0,0.0,1.0,0.0),
            direction_z : Quaternion::new(0.0,0.0,0.0,1.0),
            velocity : Vector3::new(0.0,0.0,0.0),
            resolution : resolution,
            alpha : 0.0,
            beta : 0.0,
            gamma : 0.0,
            radv : 1.0,
            fov : 75.0,
            focus : 1e10,
            bokeh : 0.0,
            size : 0.0,
            radius : 1.0,
            smooth : 0.3,
            aspect_ratio : 1.0,
            i_frame : 0,
        }
    }
}