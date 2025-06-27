use std::path::Path;

use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint + OnMouseEvent + OnResize + OnKeyPressed)]
pub struct PainterControl {
    drawing_char: Character,
    surface: Surface,
    scrollbars: ScrollBars,
}

impl PainterControl {
    pub fn new(layout: Layout) -> Self {
        let mut me = Self {
            base: ControlBase::with_focus_overlay(layout),
            drawing_char: Character::with_char('█'),
            surface: Surface::new(100, 100),
            scrollbars: ScrollBars::new(true),
        };
        me.set_components_toolbar_margins(3, 0);
        me
    }

    pub fn set_drawing_char(&mut self, ch: char) {
        self.drawing_char.code = ch;
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.drawing_char.foreground = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.drawing_char.background = color;
    }

    pub fn clear_surface(&mut self) {
        self.surface.clear(char!("' ',black,black"));
    }

    pub fn load_from_file(&mut self, file: &Path) -> Result<(), String> {
        if let Ok(surface) = Surface::from_file(file) {
            self.surface = surface;
            self.scrollbars
                .resize(self.surface.size().width as u64, self.surface.size().height as u64, &self.base);
            Ok(())
        } else {
            Err(format!("Failed to load surface from file '{}'", file.display()))
        }
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        self.surface
            .save(path)
            .map_err(|e| format!("Failed to save surface to file '{}': {}", path.display(), e))
    }
}

impl OnPaint for PainterControl {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        if self.has_focus() {
            self.scrollbars.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }
        let o = self.scrollbars.offset();
        surface.draw_surface(o.x, o.y, &self.surface);
    }
}

impl OnMouseEvent for PainterControl {
    fn on_mouse_event(&mut self, event: &MouseEvent) -> EventProcessStatus {
        if self.scrollbars.process_mouse_event(event) {
            return EventProcessStatus::Processed;
        }
        match event {
            MouseEvent::Drag(mouse_event_data) => {
                if mouse_event_data.button == MouseButton::Left {
                    let o = self.scrollbars.offset();
                    self.surface
                        .write_char(mouse_event_data.x - o.x, mouse_event_data.y - o.y, self.drawing_char);
                    EventProcessStatus::Processed
                } else {
                    EventProcessStatus::Ignored
                }
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnKeyPressed for PainterControl {
    fn on_key_pressed(&mut self, key: Key, _character: char) -> EventProcessStatus {
        match key.value() {
            key!("Up") => {
                self.scrollbars
                    .set_indexes(self.scrollbars.horizontal_index(), self.scrollbars.vertical_index().saturating_sub(1));
                EventProcessStatus::Processed
            }
            key!("Down") => {
                self.scrollbars
                    .set_indexes(self.scrollbars.horizontal_index(), self.scrollbars.vertical_index() + 1);
                EventProcessStatus::Processed
            }
            key!("Left") => {
                self.scrollbars
                    .set_indexes(self.scrollbars.horizontal_index().saturating_sub(1), self.scrollbars.vertical_index());
                EventProcessStatus::Processed
            }
            key!("Right") => {
                self.scrollbars
                    .set_indexes(self.scrollbars.horizontal_index() + 1, self.scrollbars.vertical_index());
                EventProcessStatus::Processed
            }
            _ => EventProcessStatus::Ignored,
        }
    }
}

impl OnResize for PainterControl {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.scrollbars.resize(100, 100, &self.base);
    }
}
