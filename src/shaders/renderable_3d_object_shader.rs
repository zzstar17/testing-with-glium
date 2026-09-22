use crate::shaders::common::Vertex3d;
use glium::{Display, VertexBuffer};

use crate::shaders::common::{load_srgb_texture, Material};
use obj::{load_obj, Obj};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

pub struct Renderable3dObjectShader {
    pub vertex_buffer: VertexBuffer<Vertex3d>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub material: Material,
}

impl Renderable3dObjectShader {
    pub fn new(
        display: &Display,
        model_path: &Path,
        texture_bytes: &dyn std::convert::AsRef<[u8]>,
    ) -> Self {
        println!(
            "path: {:?} {}",
            model_path.as_os_str(),
            fs::exists(model_path).unwrap()
        );
        let input = BufReader::new(File::open(model_path).unwrap());
        let obj: Obj<Vertex3d> = load_obj(input).unwrap();

        Self {
            vertex_buffer: obj.vertex_buffer(display).unwrap(),
            index_buffer: obj.index_buffer(display).unwrap(),
            material: Material {
                diffuse: load_srgb_texture(display, texture_bytes, image::ImageFormat::Png),
                specular: load_srgb_texture(
                    display,
                    &include_bytes!("../../assets/black_picture.png"),
                    image::ImageFormat::Png,
                ),
                shininess: 32.0,
            },
        }
    }
}
