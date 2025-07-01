use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::{
    action::Action,
    camera::CameraCapture,
    components::{Component, fps::FpsCounter, home::Home},
    config::Config,
    tui::{Event, Tui},
};

pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: Vec<Box<dyn Component>>,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    camera_capture: Option<CameraCapture>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![Box::new(Home::new()), Box::new(FpsCounter::default())],
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            camera_capture: None,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }
        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        // Process actions with priority: UI actions first, then camera frames
        let mut camera_frames = Vec::new();
        let mut other_actions = Vec::new();

        // Separate camera frames from other actions for prioritized processing
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::CameraFrame(_, _, _) => camera_frames.push(action),
                _ => other_actions.push(action),
            }
        }

        // Process UI actions first for better responsiveness
        for action in other_actions {
            self.process_action(action, tui)?;
        }

        // Process only the latest camera frame to prevent backup
        if let Some(latest_frame) = camera_frames.into_iter().last() {
            self.process_action(latest_frame, tui)?;
        }

        Ok(())
    }

    fn process_action(&mut self, action: Action, tui: &mut Tui) -> Result<()> {
        if action != Action::Tick && action != Action::Render {
            debug!("{action:?}");
        }
        match action {
            Action::Tick => {
                self.last_tick_key_events.drain(..);
                // Capture camera frame on tick if camera is active
                // Only capture if no frame is currently being processed
                if let Some(ref mut camera) = self.camera_capture
                    && camera.is_active()
                {
                    // Try to capture frame, but don't block if it fails
                    let _ = camera.capture_frame();
                }
            }
            Action::Quit => self.should_quit = true,
            Action::Suspend => self.should_suspend = true,
            Action::Resume => self.should_suspend = false,
            Action::ClearScreen => tui.terminal.clear()?,
            Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
            Action::Render => self.render(tui)?,
            Action::ToggleCamera => {
                self.handle_camera_toggle()?;
            }
            Action::StartCamera => {
                // This action is sent to update the UI after camera starts
                // Don't trigger any camera logic here
            }
            Action::StopCamera => {
                // This action is sent to update the UI after camera stops
                // Don't trigger any camera logic here
            }
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.update(action.clone())? {
                self.action_tx.send(action)?
            };
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, frame.area()) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {err:?}")));
                }
            }
        })?;
        Ok(())
    }

    fn handle_camera_toggle(&mut self) -> Result<()> {
        debug!("handle_camera_toggle called");
        if self.camera_capture.is_none() {
            debug!("Creating new camera capture");
            let mut camera = CameraCapture::new();

            debug!(
                "Initializing camera with index: {}, resolution: {}x{}",
                self.config.camera.default_camera_index,
                self.config.camera.width,
                self.config.camera.height
            );

            match camera.initialize(
                self.config.camera.default_camera_index,
                self.config.camera.width,
                self.config.camera.height,
                self.action_tx.clone(),
            ) {
                Ok(()) => match camera.start() {
                    Ok(()) => {
                        info!("Camera started successfully");
                        self.camera_capture = Some(camera);
                        // Send StartCamera action to update UI
                        self.action_tx.send(Action::StartCamera)?;
                    }
                    Err(e) => {
                        error!("Failed to start camera: {e}");
                        self.action_tx
                            .send(Action::CameraError(format!("Failed to start camera: {e}")))?;
                    }
                },
                Err(e) => {
                    error!("Failed to initialize camera: {e}");
                    self.action_tx.send(Action::CameraError(format!(
                        "Failed to initialize camera: {e}"
                    )))?;
                }
            }
        } else if let Some(ref mut camera) = self.camera_capture {
            debug!(
                "Camera already exists, toggling state. Current active: {}",
                camera.is_active()
            );
            if !camera.is_active() {
                debug!("Starting existing camera");
                match camera.start() {
                    Ok(()) => {
                        info!("Camera restarted successfully");
                        self.action_tx.send(Action::StartCamera)?;
                    }
                    Err(e) => {
                        error!("Failed to restart camera: {e}");
                        self.action_tx.send(Action::CameraError(format!(
                            "Failed to restart camera: {e}"
                        )))?;
                    }
                }
            } else {
                debug!(
                    "Stopping camera - current active state: {}",
                    camera.is_active()
                );
                camera.stop();
                debug!(
                    "Camera stop() called - new active state: {}",
                    camera.is_active()
                );
                self.action_tx.send(Action::StopCamera)?;
            }
        }

        Ok(())
    }
}
