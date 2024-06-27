use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    Item = <View> {
        width: Fill,
        height: Fit,

        show_bg: true,
        draw_bg: {
            color: #4f6,
        }

        <View> { width: Fill, height: 1.0 }
        label = <Label> {
            padding: 10.0,
            text: "THIN MATERIAL"
            draw_text: {
                color: #000,
                wrap: Word,
                text_style: {
                    font_size: 12.0
                }
            }
        }
        <View> { width: Fill, height: 1.0 }
    }

    Transparency = {{Transparency}} {
        show_bg: false

        width: Fill,
        height: Fill,
        flow: Down

        padding: { top: 0, left: 20, right: 20, bottom: 20 }

        <Item> { label = { text: "THINMATERIAL" }, draw_bg: { color: #fff8 } }
        <Item> { label = { text: "THICKMATERIAL" }, draw_bg: { color: #fffc } }
        <Item> { label = { text: "REGULARMATERIAL" }, draw_bg: { color: #fffa } }
        <Item> { label = { text: "ULTRATHINMATERIAL" }, draw_bg: { color: #fff5 } }
        <Item> { label = { text: "ULTRATHICKMATERIAL" }, draw_bg: { color: #ffff } }

        animator: {
            play = {
                default: init,
                show = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 2.0, end: 1.0}}
                    apply: { padding: {top: 500.0} }
                }
                init = {
                    from: {all: Snap}
                    apply: { padding: {top: 100.0} }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Transparency {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for Transparency {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.animator.need_init() || self.animator_in_state(cx, id!(play.init)) {
            self.animator_play(cx, id!(play.show));
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl TransparencyRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}