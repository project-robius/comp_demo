use makepad_widgets::widget::WidgetCache;
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

        padding: 20.0
        spacing: 5.0

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

#[derive(Live)]
pub struct Transparency {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl LiveHook for Transparency {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Transparency);
    }
}

impl Widget for Transparency {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.redraw(cx);
        }
        self.view.handle_widget_event_with(cx, event, dispatch_action);
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        if self.animator.need_init() || self.animator_in_state(cx, id!(play.init)) {
            self.animator_play(cx, id!(play.show));
        }

        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

#[derive(Clone, PartialEq, WidgetRef, Debug)]
pub struct TransparencyRef(pub WidgetRef);

impl TransparencyRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}