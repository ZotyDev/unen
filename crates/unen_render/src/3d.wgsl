struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) vert_pos: vec3<f32>,
  @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(
  @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
  var out: VertexOutput;
  let x = f32(1 - i32(in_vertex_index)) * 0.5;
  let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
  out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
  out.vert_pos = out.clip_position.xyz;
  let t = f32(in_vertex_index) * 1.3;
  out.color = 0.5 + 0.5 * vec3<f32>(
    sin(t + 0.0),
    sin(t + 2.094), // +120°
    sin(t + 4.188)  // +240°
  );
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color.rgb, 1.0);
}
