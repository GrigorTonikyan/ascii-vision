use color_eyre::Result;
use ratatui::{layout::Size, prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, error, info};

use super::Component;
use crate::{
    action::Action,
    ascii::{AsciiConverter, ColoredChar},
    camera::CameraCapture,
    config::Config,
};

pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    ascii_converter: AsciiConverter,
    current_frame: Vec<Vec<ColoredChar>>,
    camera_active: bool,
    camera_error: Option<String>,
    current_camera_index: u32,
    available_cameras: Vec<(u32, String)>,
    status_message: String,
}

impl Default for Home {
    fn default() -> Self {
        Self::new()
    }
}

impl Home {
    pub fn new() -> Self {
        let ascii_converter = AsciiConverter::new_dense(80, 24);
        Self {
            command_tx: None,
            config: Config::default(),
            ascii_converter,
            current_frame: Vec::new(),
            camera_active: false,
            camera_error: None,
            current_camera_index: 0,
            available_cameras: Vec::new(),
            status_message: "Press SPACE to start camera".to_string(),
        }
    }

    fn initialize_camera_list(&mut self) {
        info!("Initializing camera list...");
        match CameraCapture::list_cameras() {
            Ok(cameras) => {
                self.available_cameras = cameras.clone();
                if !self.available_cameras.is_empty() {
                    let camera_info = cameras
                        .iter()
                        .map(|(id, name)| format!("ID {}: {}", id, name))
                        .collect::<Vec<_>>()
                        .join(", ");

                    self.status_message = format!(
                        "Found {} camera(s): {}. Press SPACE to start.",
                        self.available_cameras.len(),
                        camera_info
                    );
                    info!("Camera list initialized: {}", camera_info);
                } else {
                    self.status_message = "No cameras found!".to_string();
                    info!("No cameras found");
                }
            }
            Err(e) => {
                let error_msg = format!("Error listing cameras: {}", e);
                self.status_message = error_msg.clone();
                error!("{}", error_msg);
            }
        }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn init(&mut self, area: Size) -> Result<()> {
        // Initialize ASCII converter with terminal dimensions
        // Leave space for UI elements (controls, status)
        let ascii_width = area.width.saturating_sub(4) as u32;
        let ascii_height = area.height.saturating_sub(6) as u32;
        self.ascii_converter.resize(ascii_width, ascii_height);

        // Initialize camera list
        self.initialize_camera_list();
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // Update status periodically
            }
            Action::Render => {
                // Nothing special on render
            }
            Action::CameraFrame(frame_data, width, height) => {
                debug!("Received camera frame: {}x{}", width, height);
                if self.camera_active {
                    // Use the optimized direct conversion method
                    self.current_frame =
                        self.ascii_converter
                            .convert_rgb_frame_direct(&frame_data, width, height);
                    self.camera_error = None;
                }
            }
            Action::CameraError(ref error) => {
                error!("Camera error received: {}", error);
                self.camera_error = Some(error.clone());
                self.camera_active = false;
            }
            Action::StartCamera => {
                info!("StartCamera action received");
                self.camera_active = true;
                self.camera_error = None;
                self.status_message = "Camera active".to_string();
            }
            Action::StopCamera => {
                info!("StopCamera action received");
                self.camera_active = false;
                self.current_frame.clear();
                self.status_message = format!(
                    "Camera stopped. Found {} camera(s). Press SPACE to restart.",
                    self.available_cameras.len()
                );
            }
            Action::ToggleCamera => {
                info!(
                    "ToggleCamera action received, current state: active={}",
                    self.camera_active
                );
                // The actual toggle logic is handled in app.rs
                // This just handles the UI state updates
                if self.camera_active {
                    self.status_message = "Stopping camera...".to_string();
                } else {
                    self.status_message = "Starting camera...".to_string();
                }
            }
            Action::NextCharacterSet => {
                let current = self.ascii_converter.character_set();
                self.ascii_converter.set_character_set(current.next());
                self.status_message = format!(
                    "Character set: {}",
                    self.ascii_converter.character_set().name()
                );
            }
            Action::PreviousCharacterSet => {
                let current = self.ascii_converter.character_set();
                self.ascii_converter.set_character_set(current.previous());
                self.status_message = format!(
                    "Character set: {}",
                    self.ascii_converter.character_set().name()
                );
            }
            Action::ToggleColor => {
                self.ascii_converter.toggle_color();
                self.status_message = format!(
                    "Color mode: {}",
                    if self.ascii_converter.color_enabled() {
                        "ON"
                    } else {
                        "OFF"
                    }
                );
            }
            Action::IncreaseScale => {
                self.ascii_converter.increase_scale();
                self.status_message = format!("Scale: {:.1}x", self.ascii_converter.scale_factor());
            }
            Action::DecreaseScale => {
                self.ascii_converter.decrease_scale();
                self.status_message = format!("Scale: {:.1}x", self.ascii_converter.scale_factor());
            }
            Action::Resize(width, height) => {
                // Update ASCII converter dimensions when terminal is resized
                let ascii_width = width.saturating_sub(4) as u32;
                let ascii_height = height.saturating_sub(6) as u32;
                self.ascii_converter.resize(ascii_width, ascii_height);
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Create layout: main area + status bar + controls
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Main ASCII display
                Constraint::Length(3), // Status bar
                Constraint::Length(2), // Controls
            ])
            .split(area);

        // Draw ASCII video feed or placeholder
        self.draw_ascii_video(frame, chunks[0])?;

        // Draw status bar
        self.draw_status_bar(frame, chunks[1])?;

        // Draw controls
        self.draw_controls(frame, chunks[2])?;

        Ok(())
    }
}

impl Home {
    fn draw_ascii_video(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let block = Block::default()
            .title("ASCII Vision")
            .borders(Borders::ALL)
            .border_style(if self.camera_active {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            });

        if let Some(ref error) = self.camera_error {
            // Display error message
            let error_text = Paragraph::new(format!("Camera Error: {}", error))
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center)
                .block(block);
            frame.render_widget(error_text, area);
        } else if self.current_frame.is_empty() {
            // Display placeholder
            let placeholder = if self.camera_active {
                "Starting camera..."
            } else {
                "Press SPACE to start camera\n\nControls:\n- SPACE: Toggle camera\n- C: Toggle color\n- S/A: Change character set\n- +/-: Adjust scale"
            };

            let text = Paragraph::new(placeholder)
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center)
                .block(block);
            frame.render_widget(text, area);
        } else {
            // Display ASCII video
            let inner = block.inner(area);
            frame.render_widget(block, area);

            // Render ASCII frame
            for (y, line) in self.current_frame.iter().enumerate() {
                if y >= inner.height as usize {
                    break;
                }

                for (x, colored_char) in line.iter().enumerate() {
                    if x >= inner.width as usize {
                        break;
                    }

                    let cell_area = Rect {
                        x: inner.x + x as u16,
                        y: inner.y + y as u16,
                        width: 1,
                        height: 1,
                    };

                    let char_widget =
                        Paragraph::new(colored_char.ch.to_string()).style(colored_char.style);
                    frame.render_widget(char_widget, cell_area);
                }
            }
        }

        Ok(())
    }

    fn draw_status_bar(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let camera_status = if self.camera_active { "ON" } else { "OFF" };
        let color_status = if self.ascii_converter.color_enabled() {
            "ON"
        } else {
            "OFF"
        };

        let status_text = format!(
            "Camera: {} | Character Set: {} | Color: {} | Scale: {:.1}x | {}",
            camera_status,
            self.ascii_converter.character_set().name(),
            color_status,
            self.ascii_converter.scale_factor(),
            self.status_message
        );

        let status_bar = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Status"));

        frame.render_widget(status_bar, area);
        Ok(())
    }

    fn draw_controls(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let controls_text = "SPACE: Camera | C: Color | S/A: Charset | +/-: Scale | Q: Quit";

        let controls = Paragraph::new(controls_text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Controls"));

        frame.render_widget(controls, area);
        Ok(())
    }
}
