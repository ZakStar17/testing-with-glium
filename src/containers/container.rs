
pub trait ObjectContainer<P, D> {
    fn new(display: &glium::Display) -> Self;

    fn draw(
        &self,
        target: &mut glium::Frame,
        programs: P,
        params: &glium::DrawParameters,
        data: D,
    );
}
