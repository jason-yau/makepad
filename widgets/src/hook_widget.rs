use {
    crate::{
        makepad_derive_widget::*,
        makepad_draw::*,
        widget::*
    }
};
live_design!{
    import makepad_draw::shader::std::*;
    
    HookWidgetBase = {{HookWidget}} {}
}

#[derive(Live, LiveHook, LiveRegisterWidget)]
pub struct HookWidget {
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[rust] draw_state: DrawStateWrap<DrawState>,
}

#[derive(Clone)]
enum DrawState {
    Hook,
}

impl Widget for HookWidget{
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut WidgetScope) {
    }

    fn walk(&mut self, _cx:&mut Cx)->Walk{
        self.walk
    }
    
    fn redraw(&mut self, _cx:&mut Cx){}
    
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope:&mut WidgetScope, _walk: Walk) -> WidgetDraw {
        if self.draw_state.begin(cx, DrawState::Hook) {
            return WidgetDraw::hook_above();
        }
        WidgetDraw::done()
    }
}
