use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    RoundedCornersItem = {{RoundedCornersItem}} {
        width: Fill,
        height: Fit

        element = <RoundedView> {
            width: 60.0,
            height: 60.0,
            draw_bg: {
                color: #4f6,
                radius: 10.0
            }
        }

        animator: {
            play = {
                default: init,
                show = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 1.0, end: 1.0}}
                    apply: {
                        element = { draw_bg: { radius: 15.0 } }
                    }
                }
                init = {
                    from: {all: Snap}
                    apply: {
                        element = { draw_bg: { radius: 0.0 } }
                    }
                }
            }
        }
    }

    Cell = <View> {
        width: Fill,
        height: Fit,

        rounded_corner_item = <RoundedCornersItem> {}
    }

    Row = <View> {
        flow: Right
        spacing: 15.0
        width: Fill
        height: Fit

        <Cell> {}
        <Cell> {}
        <Cell> {}
        <Cell> {}
    }

    Grid = <View> {
        flow: Down
        spacing: 15.0
        width: Fill
        height: Fit
        
        padding: 5.0
    }

    RoundedCorners = <View> {
        width: Fill,
        height: Fill,
        flow: Down

        padding: 20.0

        <Grid> {
            <Row> {}
            <Row> {}
            <Row> {}
            <Row> {}
            <Row> {}
            <Row> {}
            <Row> {}
            <Row> {}
        }

        <View> { width: Fill, height: Fill }
    }
}

#[derive(Live)]
pub struct RoundedCornersItem {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl LiveHook for RoundedCornersItem {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, RoundedCornersItem);
    }
}

impl Widget for RoundedCornersItem {
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
pub struct RoundedCornersItemRef(pub WidgetRef);

impl RoundedCornersItemRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}

#[derive(Clone, WidgetSet)]
pub struct RoundedCornersItemSet(WidgetSet);

impl RoundedCornersItemSet {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        for mut item in self.iter() {
            item.restart_animation(cx);
        }
    }
}