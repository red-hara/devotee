use super::config::Config;
use super::context::UpdateContext;
use super::Constructor;
use crate::util::vector::Vector;
use std::time::Duration;

/// Application setup structure.
/// Describes root node, title, resolution, etc.
pub struct Setup<Cfg>
where
    Cfg: Config,
{
    pub(super) title: String,
    pub(super) update_delay: Duration,
    pub(super) fullscreen: bool,
    pub(super) scale: u32,
    pub(super) resolution: Vector<usize>,
    pub(super) constructor: Constructor<Cfg::Node>,
    #[cfg(target_arch = "wasm32")]
    pub(super) element_id: Option<&'static str>,
}

impl<Cfg> Setup<Cfg>
where
    Cfg: Config,
{
    /// Create new setup with given `Node` constructor.
    /// Defaults to 30 frames per second update.
    pub fn new<F>(constructor: F) -> Self
    where
        F: 'static + FnOnce(&mut UpdateContext) -> Cfg::Node,
    {
        let title = String::new();
        let update_delay = Duration::from_secs_f64(1.0 / 30.0);
        let fullscreen = false;
        let scale = 1;
        let resolution = Vector::new(320, 240);
        let constructor = Box::new(constructor);
        Self {
            title,
            update_delay,
            fullscreen,
            scale,
            resolution,
            constructor,
            #[cfg(target_arch = "wasm32")]
            element_id: None,
        }
    }

    /// Set application title.
    pub fn with_title<T: Into<String>>(self, title: T) -> Self {
        Self {
            title: title.into(),
            ..self
        }
    }

    /// Set display scale.
    pub fn with_scale(self, scale: u32) -> Self {
        Self { scale, ..self }
    }

    /// Set fullscreen option.
    pub fn with_fullscreen(self, fullscreen: bool) -> Self {
        Self { fullscreen, ..self }
    }

    /// Set update delay.
    pub fn with_update_delay(self, update_delay: Duration) -> Self {
        Self {
            update_delay,
            ..self
        }
    }

    /// Set resolution.
    pub fn with_resolution<T: Into<Vector<usize>>>(self, resolution: T) -> Self {
        let resolution = resolution.into();
        Self { resolution, ..self }
    }

    #[cfg(target_arch = "wasm32")]
    /// Set target element id for canvas holding on wasm32 target
    pub fn with_element_id(self, element_id: &'static str) -> Self {
        let element_id = Some(element_id);
        Self { element_id, ..self }
    }
}

impl<Cfg> Default for Setup<Cfg>
where
    Cfg: Config,
    Cfg::Node: Default,
{
    fn default() -> Self {
        Self::new(|_| Cfg::Node::default())
    }
}
