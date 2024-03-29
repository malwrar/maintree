extern crate gl;
extern crate glm;

use std;
use std::ffi::{ CString };

// Creates a space-filled CString of the given length
fn create_whitespace_cstring(len: usize) -> CString {
    /* Create a buffer of bytes */
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);

    /* Fill buffer with spaces */
    buffer.extend([b' '].iter().cycle().take(len));

    /* Create new CString from */
    return CString::new(buffer).unwrap();
}

/// Represents a shader.
#[derive(Debug, PartialEq)]
pub struct Shader {
    id: gl::types::GLuint,
    kind: gl::types::GLenum
}

impl Shader {
    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }

    pub fn kind(&self) -> gl::types::GLenum {
        return self.kind;
    }

    /// Creates a Shader from a string containing GLSL source code.
    ///
    /// String must be a valid C string (no 0 bytes in the middle) as it will
    /// be converted and used as such.
    pub fn from_source(
        source: &str,
        kind: gl::types::GLenum
    ) -> Result<Shader, String> {
        let id = Shader::compile(source, kind)?;
        return Ok(Shader { id, kind });
    }

    pub fn from_vert_source(source: &str) -> Result<Shader, String> {
        return Shader::from_source(source, gl::VERTEX_SHADER);
    }

    pub fn from_frag_source(source: &str) -> Result<Shader, String> {
        return Shader::from_source(source, gl::FRAGMENT_SHADER);
    }

    /// Compiles a shader from a string containing its source code.
    ///
    /// String must be a valid C string (no 0 bytes in the middle) as it will
    /// be converted and used as such.
    fn compile(
        source: &str,
        kind: gl::types::GLenum
    ) -> Result<gl::types::GLuint, String> {
        /* Convert the source string to a c_string.  */
        let c_source = CString::new(source).unwrap();

        /* Create and compile shader */
        let shader_id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(shader_id, 1, &c_source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);
        }

        /* Verify that our shader compiled */
        let mut success: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            println!("BAD SHADER");
            /* Get size of error string */
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            /* Copy error into a rust-accessable string */
            let error = create_whitespace_cstring(len as usize);
            unsafe {
                gl::GetShaderInfoLog(shader_id, len, std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        return Ok(shader_id);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

/// Represents a shader program.
#[derive(Debug, PartialEq)]
pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
    /// Creates a new shader program using a vertex and fragment shader
    pub fn new_basic(vert_src: &str, frag_src: &str) -> Result<Self, String> {
        /* Create the shaders */
        let vert_shader = Shader::from_vert_source(vert_src)?;
        let frag_shader = Shader::from_frag_source(frag_src)?;

        /* Create the new program */
        return Self::from_shaders(&[ vert_shader, frag_shader ]);
    }

    /// Creates a new shader program from any number of shaders.
    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        /* Create program */
        let program_id = unsafe { gl::CreateProgram() };

        /* Attach shaders to program */
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        /* Link program */
        unsafe {
            gl::LinkProgram(program_id);
        }

        /* Make sure program was compiled correctly */
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            /* Get size of error string */
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            /* Copy error into a rust-accessable string */
            let error = create_whitespace_cstring(len as usize);
            unsafe {
                gl::GetProgramInfoLog(program_id, len, std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        /* Detach shaders */
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        return Ok(ShaderProgram { id: program_id })
    }

    pub fn get_uniform(&self, name: &str) -> Result<i32, String> {
        let c_name = CString::new(name).unwrap();

        let location = unsafe {
            gl::GetUniformLocation(self.id, c_name.as_ptr() as *const i8)
        };

        if location == -1 {
            return Err(String::from("Invalid uniform name."));
        }

        return Ok(location);
    }

    pub fn set_uniform_mat4f(&self, location: i32, value: &glm::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                location,
                1,
                gl::FALSE,
                value.as_array().as_ptr() as *const f32,
            );
        }
    }

    pub fn set_uniform_3f(&self, location: i32, value: &glm::Vector3<f32>) {
        unsafe {
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn deactivate(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
