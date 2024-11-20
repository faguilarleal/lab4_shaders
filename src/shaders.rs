use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
// planet1(fragment, uniforms)
sun_shader(fragment, uniforms)
  }

// planeta 1, planeta gaseoso
fn planet1(fragment: &Fragment, uniforms: &Uniforms) -> Color {
   let color1 = Color::new(134, 100, 35 );   
   let color2 = Color::new(169, 141, 86);   
 
   let stripe_width = 0.2;  // Width of each stripe
   let speed = 0.001;        // Speed of stripe movement
 
   let moving_y = fragment.vertex_position.y + uniforms.time as f32 * speed;
 
   let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;
 
   color1.lerp(&color2, stripe_factor) * fragment.intensity
}

fn sun_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
  // Coordenadas del fragmento normalizadas al rango [-1, 1]
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let center = (0.0, 0.0); // Centro del degradado
  let radius = ((x - center.0).powi(2) + (y - center.1).powi(2)).sqrt(); // Distancia al centro

  // Escala de colores del más claro al más oscuro
  let color1 = Color::new(255, 255, 255); // Amarillo muy claro
  let color2 = Color::new(255, 230, 28); // Amarillo pastel
  let color3 = Color::new(255, 178, 51);  // Amarillo intenso
  let color4 = Color::new(204, 102, 0);   // Naranja oscuro

  // Determina el intervalo del degradado según el radio
  let t = radius.clamp(0.0, 1.0); // Radio normalizado entre 0 y 1
  let blended_color = if t < 0.53 {
      // Mezcla entre el color1 y color2
      color1.lerp(&color2, t / 0.33)
  } else if t < 0.66 {
      // Mezcla entre el color2 y color3
      color2.lerp(&color3, (t - 0.33) / 0.33)
  } else {
      // Mezcla entre el color3 y color4
      color3.lerp(&color4, (t - 0.66) / 0.34)
  };

  blended_color
}
