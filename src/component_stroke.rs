use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ComponentStrokeItem = {{ComponentStrokeItem}} {
        width: Fill,
        height: 80.0

        align: {x: 0.5, y: 0.5}

        element = <View> {
            flow: Overlay,
            width: 80.0,
            height: 80.0,

            <RoundedView> {
                width: Fill,
                height: Fill,
                margin: 4.0,

                draw_bg: {
                    color: #6666,
                    radius: 8.0
                }
            }

            <RoundedView> {
                width: Fill,
                height: Fill,

                draw_bg: {
                    border_color: #2f23,
                    border_width: 4.0
                    radius: 8.0
                    color: #0000
                }
            }

            <RoundedView> {
                width: Fill,
                height: Fill,

                margin: 4.0,

                draw_bg: {
                    radius: 0.0
                    border_color: #f22,
                    border_width: 1.0
                    color: #0000
                }
            }
        }

        size: 80.0,

        animator: {
            play = {
                default: init,
                show = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 1.0, end: 1.0}}
                    apply: {
                        size: 74.0
                    }
                }
                init = {
                    from: {all: Snap}
                    apply: {
                        size: 80.0
                    }
                }
            }
        }
    }

    Cell = <View> {
        width: Fill,
        height: Fit,

        component_stroke_item = <ComponentStrokeItem> {}
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

    ComponentStroke = <View> {
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
pub struct ComponentStrokeItem {
    #[deref]
    view: View,

    #[live]
    size: f64,

    #[animator]
    animator: Animator,
}

impl Widget for ComponentStrokeItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.view.apply_over(cx, live!{ element = { width: (self.size), height: (self.size)} });
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

impl ComponentStrokeItemRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}

impl ComponentStrokeItemSet {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        for mut item in self.iter() {
            item.restart_animation(cx);
        }
    }
}