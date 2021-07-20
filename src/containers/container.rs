
pub trait ObjectContainer<P, D> {
    fn draw(
        &self,
        target: &mut glium::Frame,
        programs: P,
        params: &glium::DrawParameters,
        data: D,
    );
}
