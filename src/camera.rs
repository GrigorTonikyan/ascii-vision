use std::time::Duration;

use color_eyre::Result;
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{ApiBackend, CameraIndex, RequestedFormat, RequestedFormatType, Resolution},
};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::action::Action;

pub struct CameraCapture {
    camera: Option<Camera>,
    is_active: bool,
    frame_sender: Option<mpsc::UnboundedSender<Action>>,
    last_frame_time: std::time::Instant,
    frame_skip_threshold: Duration,
}

impl CameraCapture {
    pub fn new() -> Self {
        Self {
            camera: None,
            is_active: false,
            frame_sender: None,
            last_frame_time: std::time::Instant::now(),
            frame_skip_threshold: Duration::from_millis(50), // ~20 FPS max for better responsiveness
        }
    }

    /// Initialize camera with specified index and resolution
    pub fn initialize(
        &mut self,
        camera_index: u32,
        width: u32,
        height: u32,
        frame_sender: mpsc::UnboundedSender<Action>,
    ) -> Result<()> {
        info!(
            "Initializing camera {} with resolution {}x{}",
            camera_index, width, height
        );

        let index = CameraIndex::Index(camera_index);
        let requested =
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);

        debug!(
            "Creating camera with index: {:?}, format: {:?}",
            index, requested
        );

        match Camera::new(index, requested) {
            Ok(mut camera) => {
                debug!("Camera created successfully, setting resolution");
                // Try to set the requested resolution
                if let Err(e) = camera.set_resolution(Resolution::new(width, height)) {
                    warn!(
                        "Failed to set resolution {}x{}: {}, using default",
                        width, height, e
                    );
                }

                // Don't open stream here - wait for start() to be called
                let actual_resolution = camera.resolution();
                info!(
                    "Camera initialized successfully with resolution: {}x{} (stream not yet opened)",
                    actual_resolution.width(),
                    actual_resolution.height()
                );

                self.camera = Some(camera);
                self.frame_sender = Some(frame_sender);
                Ok(())
            }
            Err(e) => {
                error!("Failed to initialize camera: {}", e);
                Err(e.into())
            }
        }
    }

    /// Start capturing frames
    pub fn start(&mut self) -> Result<()> {
        debug!("start() called, current active state: {}", self.is_active);
        if self.camera.is_none() {
            error!("Cannot start: camera not initialized");
            return Err(color_eyre::eyre::eyre!("Camera not initialized"));
        }

        // Open the camera stream if not already active
        if !self.is_active {
            if let Some(ref mut camera) = self.camera {
                debug!("Opening camera stream");
                match camera.open_stream() {
                    Ok(()) => {
                        info!("Camera stream opened successfully");
                    }
                    Err(e) => {
                        error!("Failed to open camera stream: {}", e);
                        return Err(e.into());
                    }
                }
            }
        } else {
            debug!("Camera already active");
        }

        self.is_active = true;
        info!("Camera capture started successfully");
        Ok(())
    }

    /// Stop capturing frames
    pub fn stop(&mut self) {
        debug!("stop() called, current active state: {}", self.is_active);

        if !self.is_active {
            debug!("Camera already inactive");
            return;
        }

        // Properly stop the camera stream BEFORE setting inactive
        if let Some(ref mut camera) = self.camera {
            debug!("Stopping camera stream");
            match camera.stop_stream() {
                Ok(()) => {
                    info!("Camera stream stopped successfully");
                    // Only set inactive after successful stop
                    self.is_active = false;
                }
                Err(e) => {
                    error!("Error stopping camera stream: {}", e);
                    // Don't set inactive if stop failed - keep trying
                    return;
                }
            }
        } else {
            debug!("No camera to stop");
            self.is_active = false;
        }

        info!("Camera capture stopped");
    }

    /// Force stop the camera - use this for emergency shutdown
    #[allow(dead_code)]
    pub fn force_stop(&mut self) {
        debug!(
            "force_stop() called, current active state: {}",
            self.is_active
        );

        if let Some(ref mut camera) = self.camera {
            debug!("Force stopping camera stream");
            // Try multiple times if needed
            for attempt in 1..=3 {
                match camera.stop_stream() {
                    Ok(()) => {
                        info!(
                            "Camera stream force stopped successfully on attempt {}",
                            attempt
                        );
                        break;
                    }
                    Err(e) => {
                        error!(
                            "Attempt {} to force stop camera stream failed: {}",
                            attempt, e
                        );
                        if attempt < 3 {
                            std::thread::sleep(Duration::from_millis(100));
                        }
                    }
                }
            }
        }

        // Always set inactive in force stop
        self.is_active = false;
        info!("Camera force stopped");
    }

    /// Reset the camera completely - use when camera gets into bad state
    #[allow(dead_code)]
    pub fn reset(&mut self) -> Result<()> {
        info!("Resetting camera completely");

        // Force cleanup first
        self.cleanup();

        // Small delay to let the camera hardware reset
        std::thread::sleep(Duration::from_millis(500));

        info!("Camera reset completed");
        Ok(())
    }

    /// Check if camera is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Check if camera stream is actually running (not just our flag)
    #[allow(dead_code)]
    pub fn is_stream_open(&self) -> bool {
        if let Some(_camera) = &self.camera {
            // The nokhwa library doesn't provide a direct method to check stream status
            // So we rely on our is_active flag and hope it's accurate
            self.is_active
        } else {
            false
        }
    }

    /// Capture a single frame and send it via the action channel
    pub fn capture_frame(&mut self) -> Result<()> {
        if !self.is_active {
            return Ok(());
        }

        // Frame rate limiting - skip if too soon since last frame
        let now = std::time::Instant::now();
        if now.duration_since(self.last_frame_time) < self.frame_skip_threshold {
            return Ok(());
        }

        let camera = match &mut self.camera {
            Some(cam) => cam,
            None => return Err(color_eyre::eyre::eyre!("Camera not initialized")),
        };

        let frame_sender = match &self.frame_sender {
            Some(sender) => sender,
            None => return Err(color_eyre::eyre::eyre!("Frame sender not initialized")),
        };

        match camera.frame() {
            Ok(frame) => {
                self.last_frame_time = now;

                // Convert frame to RGB format first to get dimensions
                let rgb_frame = frame
                    .decode_image::<RgbFormat>()
                    .map_err(|e| color_eyre::eyre::eyre!("Failed to decode frame: {}", e))?;

                let width = rgb_frame.width();
                let height = rgb_frame.height();
                debug!("Captured frame: {}x{}", width, height);

                // Send frame data through action channel with dimensions
                // Use try_send to avoid blocking if the channel is full (frame skipping)
                if let Err(e) =
                    frame_sender.send(Action::CameraFrame(rgb_frame.into_raw(), width, height))
                {
                    // Channel full or closed - skip this frame to prevent backup
                    debug!("Skipped frame due to channel full: {}", e);
                }
                Ok(())
            }
            Err(e) => {
                error!("Failed to capture frame: {}", e);
                if let Err(send_err) =
                    frame_sender.send(Action::CameraError(format!("Frame capture failed: {e}")))
                {
                    error!("Failed to send camera error: {}", send_err);
                }
                Err(e.into())
            }
        }
    }

    /// Get available cameras
    pub fn list_cameras() -> Result<Vec<(u32, String)>> {
        debug!("Querying available cameras...");
        match nokhwa::query(ApiBackend::Auto) {
            Ok(cameras) => {
                debug!("Raw camera query returned {} cameras", cameras.len());
                let mut camera_list: Vec<(u32, String)> = Vec::new();
                let mut seen_names = std::collections::HashSet::new();

                for (i, info) in cameras.into_iter().enumerate() {
                    let name = info.human_name().to_string();
                    debug!(
                        "Raw camera {}: {} (desc: {:?})",
                        i,
                        name,
                        info.description()
                    );

                    // Filter out duplicate cameras and virtual cameras
                    if !seen_names.contains(&name)
                        && !name.to_lowercase().contains("virtual")
                        && !name.to_lowercase().contains("dummy")
                    {
                        // Keep the original system index, not the filtered position
                        camera_list.push((i as u32, name.clone()));
                        seen_names.insert(name.clone());
                        info!("Added camera with system ID {}: {}", i, name);
                    } else {
                        debug!("Filtered out camera {}: {} (duplicate or virtual)", i, name);
                    }
                }

                info!("Found {} cameras after filtering", camera_list.len());
                for (index, name) in &camera_list {
                    info!("Camera {}: {}", index, name);
                }

                Ok(camera_list)
            }
            Err(e) => {
                error!("Failed to query cameras: {}", e);
                Err(e.into())
            }
        }
    }

    /// Get current camera resolution
    #[allow(dead_code)]
    pub fn get_resolution(&self) -> Option<(u32, u32)> {
        self.camera.as_ref().map(|cam| {
            let res = cam.resolution();
            (res.width(), res.height())
        })
    }

    /// Cleanup camera resources
    pub fn cleanup(&mut self) {
        debug!("cleanup() called, current active state: {}", self.is_active);

        if let Some(mut camera) = self.camera.take() {
            debug!("Cleaning up camera, stopping stream");
            match camera.stop_stream() {
                Ok(()) => {
                    info!("Camera stream stopped during cleanup");
                }
                Err(e) => {
                    error!("Error stopping camera stream during cleanup: {}", e);
                }
            }
        }

        self.is_active = false;
        self.frame_sender = None;
        self.last_frame_time = std::time::Instant::now();
        info!("Camera cleanup completed");
    }
}

impl Drop for CameraCapture {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Async camera capture loop
#[allow(dead_code)] // Potential future use for async camera handling
pub async fn camera_capture_loop(mut camera: CameraCapture, fps: f64) {
    let frame_duration = Duration::from_secs_f64(1.0 / fps);
    let mut interval = tokio::time::interval(frame_duration);

    info!("Starting camera capture loop at {} FPS", fps);

    loop {
        interval.tick().await;

        if !camera.is_active() {
            // Small delay when camera is not active to prevent busy loop
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        }

        if let Err(e) = camera.capture_frame() {
            error!("Error in camera capture loop: {}", e);
            // Don't break the loop, just log the error and continue
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
