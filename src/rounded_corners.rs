use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    RoundedCornersCell = {{RoundedCornersCell}} {
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

    Row = <View> {
        flow: Right
        spacing: 15.0
        width: Fill
        height: Fit
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
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
            <Row> {
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
                <RoundedCornersCell> {}
            }
        }

        <View> { width: Fill, height: Fill }
    }
}

#[derive(Live)]
pub struct RoundedCornersCell {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl LiveHook for RoundedCornersCell {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, RoundedCornersCell);
    }
}

impl Widget for RoundedCornersCell {
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
        if self.animator.need_init() {
            self.animator_play(cx, id!(play.show));
        }

        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}