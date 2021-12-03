use makepad_render::*;

live_register!{
    use makepad_render::shader_std::*;
    
    TabButton: {{TabButton}} {
        button_quad: {
            
            instance hover: float;
            instance selected: float;
            
            fn pixel(self) -> vec4 {
                let cx = Sdf2d::viewport(self.pos * self.rect_size);
                let mid = self.rect_size / 2.0;
                let size = (self.hover * 0.5 + 0.5) * 0.5 * length(self.rect_size) / 2.0;
                let min = mid - vec2(size);
                let max = mid + vec2(size);
                cx.move_to(min.x, min.y);
                cx.line_to(max.x, max.y);
                cx.move_to(min.x, max.y);
                cx.line_to(max.x, min.y);
                return cx.stroke(vec4(1.0) * (0.5 * self.hover + 0.5), 1.0);
            }
        }
        
        default_state: {
            from: {all: Play::Forward {duration: 0.2}}
            button_quad: {hover: 0.0}
        }
        
        hover_state: {
            from: {all: Play::Forward {duration: 0.1}}
            button_quad: {hover: [{time: 0.0, value: 1.0}]},
        }
        
        walk: {
            height: Height::Fixed(10.0),
            width: Width::Fixed(10.0),
            margin: Margin {
                l: 0.0,
                t: 0.0,
                r: 5.0,
                b: 0.0,
            },
        },
    }
}

#[derive(Live, LiveHook)]
pub struct TabButton {
    button_quad: DrawQuad,
    #[track(
        hover = default_state
    )]
    animator: Animator,
    default_state: Option<LivePtr>,
    hover_state: Option<LivePtr>,
    walk: Walk
}

impl TabButton {
    
    pub fn draw(&mut self, cx: &mut Cx) {
        self.button_quad.draw_walk(
            cx,
            self.walk
        );
    }
    
    pub fn handle_event_ret(&mut self, cx: &mut Cx, event: &mut Event,) -> TabButtonAction {
        let mut ret = TabButtonAction::None;
        self.handle_event(cx, event, &mut | _, a | ret = a);
        ret
    }
    
    pub fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &mut Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, TabButtonAction),
    ) {
        self.animator_handle_event(cx, event);
        match event.hits(cx, self.button_quad.draw_vars.area, HitOpt::default()) {
            Event::FingerHover(event) => {
                cx.set_hover_mouse_cursor(MouseCursor::Hand);
                match event.hover_state {
                    HoverState::In => {
                        self.animate_to(cx, id!(hover), self.hover_state.unwrap());
                        dispatch_action(cx, TabButtonAction::HoverIn)
                    }
                    HoverState::Out => {
                        self.animate_to(cx, id!(hover), self.default_state.unwrap());
                        dispatch_action(cx, TabButtonAction::HoverOut)
                    }
                    _ => {}
                }
            }
            Event::FingerDown(_) => dispatch_action(cx, TabButtonAction::WasPressed),
            _ => {}
        }
    }
}

pub enum TabButtonAction {
    None,
    WasPressed,
    HoverIn,
    HoverOut,
}
