use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    IMG_0 = dep("crate://self/resources/images/0.png")
    IMG_1 = dep("crate://self/resources/images/1.png")
    IMG_2 = dep("crate://self/resources/images/2.png")
    IMG_3 = dep("crate://self/resources/images/3.png")
    IMG_4 = dep("crate://self/resources/images/4.png")
    IMG_5 = dep("crate://self/resources/images/5.png")
    IMG_6 = dep("crate://self/resources/images/6.png")
    IMG_7 = dep("crate://self/resources/images/7.png")
    IMG_8 = dep("crate://self/resources/images/8.png")
    IMG_9 = dep("crate://self/resources/images/9.png")
    IMG_10 = dep("crate://self/resources/images/10.png")
    IMG_11 = dep("crate://self/resources/images/11.png")

    RoundedImagesItem = {{RoundedImagesItem}} {
        width: Fill,
        height: Fit

        element = <Image> {
            source: (IMG_0),
            width: 60.0,
            height: 60.0,

            draw_bg: {
                instance radius: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        max(1.0, self.radius)
                    )
                    sdf.fill_keep(self.get_color())
                    return sdf.result
                }
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

        rounded_images_item = <RoundedImagesItem> {}
    }

    Row1 = <View> {
        flow: Right
        spacing: 15.0
        width: Fill
        height: Fit

        <Cell> { rounded_images_item = { element = { source: (IMG_0) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_1) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_2) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_3) } }}
    }

    Row2 = <View> {
        flow: Right
        spacing: 15.0
        width: Fill
        height: Fit

        <Cell> { rounded_images_item = { element = { source: (IMG_4) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_5) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_6) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_7) } }}
    }

    Row3 = <View> {
        flow: Right
        spacing: 15.0
        width: Fill
        height: Fit

        <Cell> { rounded_images_item = { element = { source: (IMG_8) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_9) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_10) } }}
        <Cell> { rounded_images_item = { element = { source: (IMG_11) } }}
    }

    Grid = <View> {
        flow: Down
        spacing: 15.0
        width: Fill
        height: Fit
        
        padding: 5.0
    }

    RoundedImages = <View> {
        width: Fill,
        height: Fill,
        flow: Down

        padding: 20.0

        <Grid> {
            <Row1> {}
            <Row2> {}
            <Row3> {}
            <Row1> {}
            <Row2> {}
            <Row3> {}
            <Row1> {}
            <Row2> {}
            <Row3> {}
        }

        <View> { width: Fill, height: Fill }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct RoundedImagesItem {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for RoundedImagesItem {
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

impl RoundedImagesItemRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}

impl RoundedImagesItemSet {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        for mut item in self.iter() {
            item.restart_animation(cx);
        }
    }
}