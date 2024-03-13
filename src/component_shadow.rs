use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import crate::blur_image::*;

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

    SHADOW_BLUR_SIZE = 5.0
    SHADOW_BLUR_STD = 1.5

    ComponentShadowItem = {{ComponentShadowItem}} {
        width: 100.0,
        height: 100.0

        flow: Overlay

        align: {x: 0.5, y: 0.5}

        shadow = <RoundedView> {
            width: 90,
            height: 90,

            draw_bg: {
                radius: 10.0
                color: #000

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    sdf.box(
                        self.inset.x + self.border_width,
                        self.inset.y + self.border_width,
                        self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                        self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                        max(1.0, self.radius)
                    )
                    sdf.blur = 20.0;
                    sdf.fill_keep(self.get_color())
                    if self.border_width > 0.0 {
                        sdf.stroke(self.get_border_color(), self.border_width)
                    }
                    return sdf.result;
                }
            }
        }

        image = <Image> {
            source: (IMG_0),
            width: 75.0,
            height: 75.0,

            draw_bg: {
                instance radius: 10.0
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
                    return sdf.result;
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
                        image = { draw_bg: { radius: 15.0 } }
                        shadow = { draw_bg: { radius: 15.0 } }
                    }
                }
                init = {
                    from: {all: Snap}
                    apply: {
                        image = { draw_bg: { radius: 3.0 } }
                        shadow = { draw_bg: { radius: 3.0 } }
                    }
                }
            }
        }
    }

    Cell = <View> {
        width: Fit,
        height: Fit,

        component_shadow_item = <ComponentShadowItem> {}
    }

    Row1 = <View> {
        flow: Overlay
        width: Fill
        height: Fit

        <Cell> { component_shadow_item = { image = { source: (IMG_0) } }}
        <Cell> {
            margin: { left: 85.0 }
            component_shadow_item = { image = { source: (IMG_1) } }
        }
        <Cell> {
            margin: { left: 170.0 }
            component_shadow_item = { image = { source: (IMG_2) } }
        }
        <Cell> {
            margin: { left: 255.0 }
            component_shadow_item = { image = { source: (IMG_3) } }
        }
    }

    Row2 = <View> {
        flow: Overlay
        width: Fill
        height: Fit

        <Cell> { component_shadow_item = { image = { source: (IMG_4) } }}
        <Cell> {
            margin: { left: 85.0 }
            component_shadow_item = { image = { source: (IMG_5) } }
        }
        <Cell> {
            margin: { left: 170.0 }
            component_shadow_item = { image = { source: (IMG_6) } }
        }
        <Cell> {
            margin: { left: 255.0 }
            component_shadow_item = { image = { source: (IMG_7) } }
        }
    }

    Row3 = <View> {
        flow: Overlay
        width: Fill
        height: Fit

        <Cell> { component_shadow_item = { image = { source: (IMG_8) } }}
        <Cell> {
            margin: { left: 85.0 }
            component_shadow_item = { image = { source: (IMG_9) } }
        }
        <Cell> {
            margin: { left: 170.0 }
            component_shadow_item = { image = { source: (IMG_10) } }
        }
        <Cell> {
            margin: { left: 255.0 }
            component_shadow_item = { image = { source: (IMG_11) } }
        }
    }

    Grid = <View> {
        flow: Overlay

        width: Fill
        height: Fit

        <Row1> {}
        <Row2> { margin: { top: 82.0 }}
        <Row3> { margin: { top: 164.0 }}

        <Row1> { margin: { top: 246.0 }}
        <Row2> { margin: { top: 328.0 }}
        <Row3> { margin: { top: 410.0 }}

        <Row1> { margin: { top: 492.0 }}
        <Row2> { margin: { top: 574.0 }}
        <Row3> { margin: { top: 656.0 }}

        padding: 5.0
    }

    ComponentShadow = <View> {
        width: Fill,
        height: Fill,
        flow: Down

        padding: { top: 20.0, left: 5.0 }

        <Grid> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ComponentShadowItem {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for ComponentShadowItem {
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

impl ComponentShadowItemRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}

impl ComponentShadowItemSet {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        for mut item in self.iter() {
            item.restart_animation(cx);
        }
    }
}