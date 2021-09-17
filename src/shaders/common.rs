use num_traits::FromPrimitive;
use obj::raw::object::Polygon;
use obj::FromRawVertex;
use obj::ObjResult;
use std::collections::hash_map::{Entry, HashMap};
use std::io::Cursor;

use cgmath::{Point3, Vector3};
use glium::{
    texture::{RawImage2d, SrgbTexture2d, Texture2d},
    Display,
};

pub struct Material {
    pub diffuse: glium::texture::SrgbTexture2d,
    pub specular: glium::texture::SrgbTexture2d,
    pub shininess: f32,
}

#[derive(Clone)]
pub struct DirectionalLight {
    pub direction: Vector3<f32>,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

#[derive(Clone)]
pub struct PointLight {
    pub position: Point3<f32>,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

#[derive(Clone)]
pub struct SpotLight {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub cut_off: f32,
    pub outer_cut_off: f32,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

#[derive(Copy, Clone)]
pub struct Vertex3d {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl<I: FromPrimitive + Copy> FromRawVertex<I> for Vertex3d {
    fn process(
        positions: Vec<(f32, f32, f32, f32)>,
        normals: Vec<(f32, f32, f32)>,
        tex_coords: Vec<(f32, f32, f32)>,
        polygons: Vec<Polygon>,
    ) -> ObjResult<(Vec<Self>, Vec<I>)> {
        let mut vb = Vec::with_capacity(polygons.len() * 3);
        let mut ib = Vec::with_capacity(polygons.len() * 3);
        {
            let mut cache = HashMap::new();
            let mut map = |pi: usize, ni: usize, ti: usize| {
                // Look up cache
                let index = match cache.entry((pi, ni, ti)) {
                    // Cache miss -> make new, store it on cache
                    Entry::Vacant(entry) => {
                        let p = positions[pi];
                        let n = normals[ni];
                        let t = tex_coords[ti];
                        let vertex = Vertex3d {
                            position: [p.0, p.1, p.2],
                            normal: [n.0, n.1, n.2],
                            tex_coords: [t.0, t.1],
                        };
                        let index = I::from_usize(vb.len())
                            .expect("Unable to convert the index from usize");
                        vb.push(vertex);
                        entry.insert(index);
                        index
                    }
                    // Cache hit -> use it
                    Entry::Occupied(entry) => *entry.get(),
                };
                ib.push(index)
            };

            for polygon in polygons {
                match polygon {
                    Polygon::P(_) => panic!("Tried to extract normal and texture data which are not contained in the model"),
                    Polygon::PT(_) => panic!("Tried to extract normal data which are not contained in the model"),
                    Polygon::PN(_) => panic!("Tried to extract texture data which are not contained in the model"),
                    Polygon::PTN(ref vec) if vec.len() == 3 => {
                        for &(pi, ti, ni) in vec { map(pi, ni, ti) }
                    }
                    _ => panic!("Model should be triangulated first to be loaded properly")
                }
            }
        }
        vb.shrink_to_fit();
        Ok((vb, ib))
    }
}

#[derive(Copy, Clone)]
pub struct PositionalVertex {
    pub position: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

pub fn load_srgb_texture(
    display: &Display,
    texture_bytes: &dyn std::convert::AsRef<[u8]>,
    image_format: image::ImageFormat,
) -> SrgbTexture2d {
    SrgbTexture2d::new(display, load_raw_image(texture_bytes, image_format)).unwrap()
}

pub fn load_texture(
    display: &Display,
    texture_bytes: &dyn std::convert::AsRef<[u8]>,
    image_format: image::ImageFormat,
) -> Texture2d {
    Texture2d::new(display, load_raw_image(texture_bytes, image_format)).unwrap()
}

fn load_raw_image(
    image_bytes: &dyn std::convert::AsRef<[u8]>,
    image_format: image::ImageFormat,
) -> RawImage2d<u8> {
    use std::time::Instant;

    let now = Instant::now();
    let image = image::load(Cursor::new(image_bytes), image_format)
        .unwrap()
        .to_rgba8();
    println!("Loaded image in {} milliseconds", now.elapsed().as_millis());

    let image_dimensions = image.dimensions();
    RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

#[allow(dead_code)]
pub fn main() {
    implement_vertex!(Vertex3d, position, normal, tex_coords);
    implement_vertex!(PositionalVertex, position);
    implement_vertex!(Vertex2d, position, tex_coords);
}
