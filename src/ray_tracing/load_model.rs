use crate::{
	material::MaterialEnum,
	ray_tracing::primitives::{MeshData, MeshTriangle, PrimitiveEnum},
	texture::TextureEnum,
	utility::{vec::Vec3, Float},
};
use std::sync::Arc;

pub fn load_model(
	filepath: &str,
	material: &Arc<MaterialEnum<TextureEnum>>,
) -> Vec<PrimitiveEnum<MaterialEnum<TextureEnum>>> {
	let model = wavefront_obj::obj::parse(&std::fs::read_to_string(filepath).unwrap());

	let model = model.unwrap();

	let material = Arc::new(material);

	let mut primitives: Vec<PrimitiveEnum<MaterialEnum<TextureEnum>>> = Vec::new();

	for object in model.objects {
		let mesh_data: Arc<MeshData<MaterialEnum<TextureEnum>>> = Arc::new(MeshData::new(
			object
				.vertices
				.iter()
				.map(|vertex| vertex_to_vec3(*vertex))
				.collect(),
			object
				.normals
				.iter()
				.map(|normal| vertex_to_vec3(*normal))
				.collect(),
			&material,
		));

		for geometric_object in object.geometry {
			for shape in geometric_object.shapes {
				if let wavefront_obj::obj::Primitive::Triangle(i1, i2, i3) = shape.primitive {
					if i1.2.is_none() {
						panic!("Please export obj file with vertex normals!");
					}

					let triangle: PrimitiveEnum<MaterialEnum<TextureEnum>> =
						PrimitiveEnum::MeshTriangle(MeshTriangle::new(
							[i1.0, i2.0, i3.0],
							[i1.2.unwrap(), i2.2.unwrap(), i3.2.unwrap()],
							&material,
							&mesh_data,
						));

					primitives.push(triangle)
				}
			}
		}
	}
	primitives
}

fn vertex_to_vec3(vertex: wavefront_obj::obj::Vertex) -> Vec3 {
	Vec3::new(vertex.x as Float, vertex.y as Float, vertex.z as Float)
}
