pub trait ObjectContainer<P, D> {
    fn draw(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        programs: P,
        params: &glium::DrawParameters,
        data: D,
    );
}
