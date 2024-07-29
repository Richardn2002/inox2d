use glow::HasContext;

use super::OpenglRendererError;

unsafe fn upload_array_to_gl<T>(gl: &glow::Context, array: &[T], target: u32, usage: u32) {
	let bytes: &[u8] = core::slice::from_raw_parts(array.as_ptr() as *const u8, std::mem::size_of_val(array));
	let buffer = gl.create_buffer().unwrap();
	gl.bind_buffer(target, Some(buffer));
	gl.buffer_data_u8_slice(target, bytes, usage);
}

/// Create a vertex array. Initialize vertex, uv, deform and index buffers, upload content and attach them to the vertex array. Return the array.
///
/// # Errors
///
/// This function will return an error if it couldn't create a vertex array.
///
/// # Safety
///
/// No prerequisites.
pub unsafe fn setup_gl_buffers(
	gl: &glow::Context,
	verts: &[f32],
	uvs: &[f32],
	deforms: &[f32],
	indices: &[u16],
) -> Result<glow::VertexArray, OpenglRendererError> {
	let vao = gl.create_vertex_array().map_err(OpenglRendererError::Opengl)?;
	gl.bind_vertex_array(Some(vao));

	upload_array_to_gl(gl, verts, glow::ARRAY_BUFFER, glow::STATIC_DRAW);
	gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
	gl.enable_vertex_attrib_array(0);

	upload_array_to_gl(gl, uvs, glow::ARRAY_BUFFER, glow::STATIC_DRAW);
	gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 0, 0);
	gl.enable_vertex_attrib_array(1);

	upload_array_to_gl(gl, deforms, glow::ARRAY_BUFFER, glow::DYNAMIC_DRAW);
	gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 0, 0);
	gl.enable_vertex_attrib_array(2);

	upload_array_to_gl(gl, indices, glow::ELEMENT_ARRAY_BUFFER, glow::STATIC_DRAW);

	gl.bind_vertex_array(None);
	Ok(vao)
}

/// Upload full deform buffer content.
///
/// # Safety
///
/// The vertex array object created in `setup_gl_buffers()` must be bound and no new ARRAY_BUFFER is enabled.
pub unsafe fn upload_deforms_to_gl(gl: &glow::Context, deforms: &[f32]) {
	// if the above preconditions are met, deform is then the currently bound ARRAY_BUFFER.
	let bytes: &[u8] = core::slice::from_raw_parts(deforms.as_ptr() as *const u8, core::mem::size_of_val(deforms));
	gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, bytes);
}
