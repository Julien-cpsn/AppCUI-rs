use appcui::prelude::*;

#[CustomControl(overwrite = OnPaint + OnMouseEvent + OnResize)]
pub struct PainterControl {
    drawing_char: Character,
    surface: Surface,
    scrollbars: ScrollBars,
}

impl PainterControl {
    pub fn new(layout: Layout) -> Self {
        Self {
            base: ControlBase::with_scrollbars(layout),
            drawing_char: Character::with_char('█'),
            surface: Surface::new(100, 100),
            scrollbars: ScrollBars::new(true),
        }
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

impl OnResize for PainterControl {
    fn on_resize(&mut self, _old_size: Size, _new_size: Size) {
        self.scrollbars.resize(100, 100, &self.base);
    }
}
