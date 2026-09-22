use std::path::Path;

use crate::objects::renderable_3d_object::HasRenderable3dObject;
use crate::shaders::renderable_3d_object_shader::Renderable3dObjectShader;
use cgmath::{Matrix4, Point3, Vector3};
use glium::Surface;

use crate::shaders::{
    common::{DirectionalLight, PointLight, SpotLight},
    programs,
};

pub struct Renderable3dObjectContainer<Obj: HasRenderable3dObject> {
    pub shader: Renderable3dObjectShader,
    pub objects: Vec<Obj>,
}

pub struct Renderable3dObjectContainerDrawData<'a, 'b, 'c, 'd> {
    pub projection_view: &'a Matrix4<f32>,
    pub camera_pos: Point3<f32>,
    pub spot_light: &'b SpotLight,
    pub point_lights: &'c [&'d PointLight; 4],
    pub directional_light_intensity: f32,
}

impl<Obj: HasRenderable3dObject> Renderable3dObjectContainer<Obj> {
    pub fn new(
        display: &glium::Display,
        model_path: &Path,
        texture_bytes: &dyn std::convert::AsRef<[u8]>,
    ) -> Self {
        Self {
            shader: Renderable3dObjectShader::new(display, model_path, texture_bytes),
            objects: Vec::new(),
        }
    }

    pub fn draw_objects(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        program: &programs::SimpleTexturedObjectProgram,
        params: &glium::DrawParameters,
        projection_view: &Matrix4<f32>,
        camera_pos: Point3<f32>,
        spot_light: &SpotLight,
        lights: &[&PointLight; 4],
        directional_light_intensity: f32,
    ) {
        for object in self.objects.iter() {
            let model_matrix = object.get_object().model_matrix;
            let matrix = projection_view * model_matrix;

            let directional_light = {
                let ambient = directional_light_intensity / 3.0;
                let diffuse = directional_light_intensity;
                let specular = directional_light_intensity * 0.4 + 0.4;
                DirectionalLight {
                    ambient: Vector3::new(ambient, ambient, ambient),
                    diffuse: Vector3::new(diffuse, diffuse, diffuse),
                    specular: Vector3::new(specular, specular, specular),
                    direction: Vector3::new(-0.2, -1.0, -0.3),
                }
            };

            let uniforms = programs::SimpleTexturedObjectProgram::get_uniforms(
                &matrix,
                &model_matrix,
                &self.shader.material,
                &directional_light,
                spot_light,
                &lights,
                &camera_pos,
            );

            target
                .draw(
                    &self.shader.vertex_buffer,
                    &self.shader.index_buffer,
                    &program.0,
                    &uniforms,
                    params,
                )
                .unwrap();
        }
    }
    pub fn draw(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        program: &programs::SimpleTexturedObjectProgram,
        params: &glium::DrawParameters,
        data: Renderable3dObjectContainerDrawData,
    ) {
        self.draw_objects(
            target,
            program,
            params,
            data.projection_view,
            data.camera_pos,
            data.spot_light,
            data.point_lights,
            data.directional_light_intensity,
        );
    }
}
