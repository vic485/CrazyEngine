use std::convert::TryInto;
use luminance::texture::Dim2;
use luminance::texture::Flat;
use luminance::framebuffer::Framebuffer;
use gl;
use luminance::context::GraphicsContext;
use luminance::state::GraphicsState;
pub use luminance::state::StateQueryError;
// pub use luminance_windowing::{CursorMode, Surface, WindowDim, WindowOpt};
use std::cell::RefCell;
use std::fmt;
use std::os::raw::c_void;
use std::rc::Rc;
use std::ffi::CString;
use std::ops::DerefMut;

// #[macro_use] extern crate log;

//Error that can be risen while creating a surface
#[derive(Debug)]
pub enum SurfaceError {
    GraphicsStateError(StateQueryError)
}

impl fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            SurfaceError::GraphicsStateError(sqe) => write!(f, "Failed to get graphics state: {}", sqe)
        }
    }
}

#[repr(C)]
pub struct Surface {
    pub window_size: (u32, u32),
    pub gfx_state: Rc<RefCell<GraphicsState>>,
}

unsafe impl GraphicsContext for Surface {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> {
        &self.gfx_state
    }
}

//TODO: Implement better error checking using error enum above (e.g. notify user that sdl2.dll is missing)
//TODO: Perhaps implement wrappers for a bunch of window-related functions?
impl Surface {
    pub fn new(window_size: (u32, u32)) -> Result<Self, SurfaceError> {
        // gl::load_with(|s| video.gl_get_proc_address(s) as *const c_void);
        // let swap_interval = if vsync {
        //     sdl2::video::SwapInterval::VSync
        // } else {
        //     sdl2::video::SwapInterval::Immediate
        // };
        //
        // video.gl_set_swap_interval(swap_interval).expect("Failed to set swap interval!");

        let gfx_state = GraphicsState::new().map_err(SurfaceError::GraphicsStateError)?;
        let surface = Self {
            window_size: window_size,
            gfx_state: Rc::new(RefCell::new(gfx_state)),
        };

        Ok(surface)
    }

    pub fn size(&self) -> (u32, u32) {
        self.window_size
    }

    pub fn size_array(&self) -> [u32; 2] {
        let size = self.size();
        [size.0.try_into().expect("Failed to turn size into i32"), size.1.try_into().expect("Failed to turn size into i32")]
    }

    pub fn width(&self) -> u32 {
        self.window_size.0
    }

    pub fn height(&self) -> u32 {
        self.window_size.1
    }

    pub fn back_buffer(&mut self) -> Result<Framebuffer<Flat, Dim2, (), ()>, SurfaceError> {
        Ok(Framebuffer::back_buffer(self, self.size_array()))
    }
}
