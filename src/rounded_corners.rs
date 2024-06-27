use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    RoundedCornersItem = {{RoundedCornersItem}} {
        width: Fill,
        height: Fit

        element = <RoundedView> {
            width: 75.0,
            height: 75.0,
            draw_bg: {
                color: #30c04f
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

        rounded_corners_item = <RoundedCornersItem> {}
    }

    Row = <View> {
        flow: Right
        spacing: 10.0
        width: Fill
        height: Fit

        <Cell> {}
        <Cell> {}
        <Cell> {}
        <Cell> {}
    }

    Grid = <View> {
        flow: Down
        spacing: 10.0
        width: Fill
        height: Fit

        padding: 5.0
    }

    RoundedCorners = <View> {
        width: Fill,
        height: Fill,
        flow: Down

        padding: { top: 0, left: 20, right: 20, bottom: 20 }

        <Grid> {
            <Row> {}
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

#[derive(Live, LiveHook, Widget)]
pub struct RoundedCornersItem {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for RoundedCornersItem {
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

impl RoundedCornersItemRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}

impl RoundedCornersItemSet {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        for mut item in self.iter() {
            item.restart_animation(cx);
        }
    }
}