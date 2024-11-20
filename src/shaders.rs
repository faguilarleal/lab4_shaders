use nalgebra_glm::{atan2, dot, mat4_to_mat3, Mat3, Vec2, Vec3, Vec4};
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
// sun_shader(fragment, uniforms)
// earth_shader(fragment, uniforms)
// vibrant_blue_planet_shader(fragment, uniforms)
planet_exotic_shader(fragment, uniforms)
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

  let color1 = Color::new(255, 255, 255); // Amarillo muy claro
  let color2 = Color::new(255, 230, 28); // Amarillo pastel
  let color3 = Color::new(255, 178, 51);  // Amarillo intenso
  let color4 = Color::new(204, 102, 0);   // Naranja oscuro

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


fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.vertex_position.z;

  let theta = (y / 0.5).asin(); // Latitud
  let phi = z.atan2(x);         // Longitud
  let u = (phi / (2.0 * PI)) + 0.5; // Coordenada u [0, 1]
  let v = (theta / PI) + 0.5;      // Coordenada v [0, 1]

  let scale = 7.2;
  let noise = ((u * scale).sin() * (v * scale).cos()).abs();
  let continent_threshold = 0.55;

  let land_color = Color::new(34, 139, 34); // Verde
  let ocean_color = Color::new(0, 105, 148); // Azul
  let base_color = if noise > continent_threshold { land_color } else { ocean_color };

  // Agregar polos
  let circle1_center = Vec2::new(0.5, 0.7); // Coordenadas UV del primer círculo
  let circle2_center = Vec2::new(0.2, 0.3); // Coordenadas UV del segundo círculo
  let circle_radius = 0.1;                 // Radio de los círculos

  let frag_position = Vec2::new(u, v);
  let distance_to_circle1 = (frag_position - circle1_center).norm();
  let distance_to_circle2 = (frag_position - circle2_center).norm();

  // Dibujar círculos: Si la distancia es menor al radio, aplica un color
  let circle_color = Color::new(255, 255, 255); // Blanco para los círculos
  let is_in_circle1 = distance_to_circle1 < circle_radius;
  let is_in_circle2 = distance_to_circle2 < circle_radius;

  // Simular nubes en la atmósfera
  let time = uniforms.time as f32 * 0.1; // Escala del tiempo para velocidad
  let cloud_scale = 8.0;          // Escala de las nubes
  let cloud_pattern = ((u * cloud_scale + time).sin() * (v * cloud_scale + time).cos()).abs();
  let cloud_intensity = (cloud_pattern - 0.5).clamp(0.0, 1.0) * 0.3; // Intensidad y opacidad de las nubes
  let cloud_color = Color::new(255, 255, 255) * cloud_intensity;

  // Nubes en movimiento circular hacia afuera
  let cloud_radius = 0.9; // Límite de la atmósfera con nubes
  let distance_from_center = frag_position.norm();
  let is_in_atmosphere = distance_from_center < cloud_radius;

  // Combinar colores
  let planet_color = if is_in_circle1 || is_in_circle2 {
      circle_color
  } else {
      base_color
  };

  let final_color = if is_in_atmosphere {
      planet_color * (1.0 - cloud_intensity) + cloud_color
  } else {
      planet_color
  };

  final_color
}

pub fn vibrant_blue_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base del planeta
  let color_azul_oscuro = Color::new(10, 20, 50);        // Azul oscuro
  let color_azul_medio = Color::new(30, 60, 120);        // Azul medio
  let color_turquesa_brillante = Color::new(50, 200, 255); // Turquesa brillante

  // Posición normalizada del fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let distancia_centro = (x * x + y * y).sqrt(); // Distancia al centro (para mantener forma esférica)

  // Patrón 1: Líneas curvas basadas en coordenadas
  let scale = 5.0;
  let patron1 = ((x * scale).sin() * (y * scale).cos()).abs();

  // Patrón 2: Variaciones más sutiles hacia el centro
  let patron2 = (1.0 - distancia_centro).clamp(0.0, 1.0);

  // Interpolación de colores para el cuerpo del planeta
  let mut color_final = color_azul_oscuro.lerp(&color_azul_medio, patron1);
  color_final = color_final.lerp(&color_turquesa_brillante, patron2);

  // Agregar brillo ambiental para simular atmósfera
  let glow_intensity = (1.0 - distancia_centro).clamp(0.0, 1.0) * 0.3;
  let glow_color = Color::new(100, 200, 255); // Azul brillante para el resplandor

  // Combinar los patrones con la atmósfera
  color_final * fragment.intensity + glow_color * glow_intensity
}

pub fn planet_exotic_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base
  let color_amarillo = Color::new(255, 223, 75);    // Amarillo
  let color_naranja = Color::new(255, 165, 0);      // Naranja
  let color_lila = Color::new(238, 130, 238);       // Lila
  let color_rosa = Color::new(255, 105, 180);       // Rosa
  let color_purpura = Color::new(75, 0, 130);       // Púrpura

  // Posición del fragmento y tiempo
  let position = fragment.vertex_position;
  let t = uniforms.time as f32 * 0.6;

  // Escala para patrones detallados
  let zoom = 150.0;

  // Función de ruido aproximada con varias capas
  let ruido_base = ((position.x * zoom + t).sin() * (position.y * zoom + t).cos()).abs();
  let ruido_detalle = ((position.x * zoom * 0.5).sin() * (position.y * zoom * 0.5).cos() * 0.5).abs();
  let ruido_fino = ((position.x * zoom * 2.0 + t * 0.5).sin() * (position.y * zoom * 2.0 + t * 0.5).cos()).abs();

  // Combinar las capas de ruido
  let ruido = (ruido_base + ruido_detalle * 0.5 + ruido_fino * 0.25).clamp(0.0, 1.0);

  // Generar patrones de color
  let patron1 = (ruido * 1.5 + (position.x * 0.5).sin() * 0.5).clamp(0.0, 1.0);
  let patron2 = ((position.y * 0.3 + ruido) * 2.0).sin().abs();

  // Luz ambiental
  let ambient_intensity = 0.3;
  let ambient_color = Color::new(30, 20, 60);

  // Interpolación de colores entre las zonas del planeta
  let mut color_final = color_amarillo.lerp(&color_naranja, patron1);
  color_final = color_final.lerp(&color_lila, patron2);
  color_final = color_final.lerp(&color_rosa, patron1 * patron2);
  color_final = color_final.lerp(&color_purpura, (1.0 - patron1) * 0.5);

  // Combinar con luz ambiental
  color_final * fragment.intensity + ambient_color * ambient_intensity
}
