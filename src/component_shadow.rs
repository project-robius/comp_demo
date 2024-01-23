use makepad_widgets::widget::WidgetCache;
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

    STD = 1.5
    SIZE = 3.0

    ComponentShadowItem = {{ComponentShadowItem}} {
        width: 100.0,
        height: 100.0

        flow: Overlay

        align: {x: 0.5, y: 0.5}

        step4 = <BlurStage>{
            width: Fill,
            height: Fill,
            draw_bg:{blury: 0.0, blurx: 10.0, blursize: (SIZE), blurstd: (STD)}
            step3 = <BlurStage>{
                width: Fill,
                height: Fill,
                draw_bg:{blury: 10.0, blurx: 0.0, blursize: (SIZE), blurstd: (STD)}
                step2 = <BlurStage>{
                    width: Fill,
                    height: Fill,
                    draw_bg:{blury: 7.07, blurx: 7.07, blursize: (SIZE), blurstd: (STD)}
                    step1 = <BlurStage>{
                        width: Fill,
                        height: Fill,
    
                        draw_bg:{blury: -7.07, blurx: 7.07, blursize: (SIZE), blurstd: (STD)}
                        
                        align: {x: 0.5, y: 0.5}
                        shadow = <RoundedView> {
                            width: 60,
                            height: 60,

                            draw_bg: {
                                radius: 10.0
                                color: #000
                            }
                        }
                    }
                }
            }
        }

        image = <Image> {
            source: (IMG_0),
            width: 60.0,
            height: 60.0,

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
                        image = { draw_bg: { radius: 15.0 }}
                        step4 = { step3 = { step2 = { step1 = { shadow = { draw_bg: {radius: 15.0 } }}}}}
                    }
                }
                init = {
                    from: {all: Snap}
                    apply: {
                        image = {draw_bg: { radius: 0.0 }}
                        step4 = { step3 = { step2 = { step1 = { shadow = { draw_bg: {radius: 0.0 } }}}}}
                    }
                }
            }
        }
    }

    // ComponentShadowItem = {{ComponentShadowItem}} {
    //     width: 70.0,
    //     height: 70.0

    //     element = <View> {
    //         width: 64.0,
    //         height: 64.0,
    //         align: {x: 0.5, y: 0.5},

    //         show_bg: true,
    //         draw_bg: {
    //             instance radius: 10.0

    //             fn gaussian(x: float, sigma: float) -> float {
    //                 return exp(-(x * x) / (2.0 * sigma * sigma)) / (sqrt(2.0 * PI) * sigma);
    //             }

    //             fn erf(x: vec2) -> vec2 {
    //                 let s = sign(x);
    //                 let a = abs(x);
    //                 x = 1.0 + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    //                 x *= x;
    //                 return s - s / (x * x);
    //             }

    //             // Return the blurred mask along the x dimension
    //             fn roundedBoxShadowX(x: float, y: float, sigma: float, corner: float, halfSize: vec2) -> float {
    //                 let delta = min(halfSize.y - corner - abs(y), 0.0);
    //                 let curved = halfSize.x - corner + sqrt(max(0.0, corner * corner - delta * delta));
    //                 let integral = 0.5 + 0.5 * erf((x + vec2(-curved, curved)) * (sqrt(0.5) / sigma));
    //                 return integral.y - integral.x;
    //             }

    //             // Return the mask for the shadow of a box from lower to upper
    //             fn roundedBoxShadow(lower: vec2, upper: vec2, point: vec2, sigma: float, corner: float) -> float {
    //                 // Center everything to make the math easier
    //                 let center = (lower + upper) * 0.5;
    //                 let halfSize = (upper - lower) * 0.5;
    //                 point -= center;
                
    //                 // The signal is only non-zero in a limited range, so don't waste samples
    //                 let low = point.y - halfSize.y;
    //                 let high = point.y + halfSize.y;
    //                 let start = clamp(-3.0 * sigma, low, high);
    //                 let end = clamp(3.0 * sigma, low, high);
                
    //                 // Accumulate samples (we can get away with surprisingly few samples)
    //                 let step = (end - start) / 4.0;
    //                 let y = start + step * 0.5;
    //                 let value = 0.0;

    //                 value += roundedBoxShadowX(point.x, point.y - y, sigma, corner, halfSize) * gaussian(y, sigma) * step;
    //                 y += step;

    //                 value += roundedBoxShadowX(point.x, point.y - y, sigma, corner, halfSize) * gaussian(y, sigma) * step;
    //                 y += step;

    //                 value += roundedBoxShadowX(point.x, point.y - y, sigma, corner, halfSize) * gaussian(y, sigma) * step;
    //                 y += step;

    //                 value += roundedBoxShadowX(point.x, point.y - y, sigma, corner, halfSize) * gaussian(y, sigma) * step;
    //                 y += step;
                
    //                 return value;
    //             }

                
    //             fn pixel(self) -> vec4 {
    //                 // let sdf = Sdf2d::viewport(self.pos * self.rect_size);
    //                 // sdf.box(
    //                 //     1,
    //                 //     1,
    //                 //     self.rect_size.x - 2.0,
    //                 //     self.rect_size.y - 2.0,
    //                 //     max(1.0, self.radius)
    //                 // )
    //                 let color = #0009;
    //                 color.a *= roundedBoxShadow(vec2(0.0), self.rect_size * 0.5manolo01
    //                 , self.pos * self.rect_size * 0.5, 1.0, self.radius * 1.2);
    //                 // sdf.fill_keep(color);
    //                 // return sdf.result;
    //                 return color;
    //             }

    //             // fn vertex(self) -> vec4 {
    //             //     return self.clip_and_transform_vertex(self.rect_pos, self.rect_size + 0.1);
    //             // }

    //             fn vertex(self) -> vec4 {
    //                 let clipped: vec2 = clamp(
    //                     self.geom_pos * vec2(self.rect_size.x + 320., self.rect_size.y + 320.) + self.rect_pos - vec2(160., 160.),
    //                     self.draw_clip.xy,
    //                     self.draw_clip.zw
    //                 );
    //                 self.pos = (clipped - self.rect_pos) / self.rect_size;
    //                 return self.camera_projection * (self.camera_view * (
    //                     self.view_transform * vec4(clipped.x, clipped.y, self.draw_depth + self.draw_zbias, 1.)
    //                 ));
    //             }
    //         },
        
    //         image = <Image> {
    //             source: (IMG_0),
    //             width: 60.0,
    //             height: 60.0,

    //             draw_bg: {
    //                 instance radius: 10.0
    //                 fn pixel(self) -> vec4 {
    //                     let sdf = Sdf2d::viewport(self.pos * self.rect_size);
    //                     sdf.box(
    //                         1,
    //                         1,
    //                         self.rect_size.x - 2.0,
    //                         self.rect_size.y - 2.0,
    //                         max(1.0, self.radius)
    //                     )
    //                     sdf.fill_keep(self.get_color())
    //                     //return #0000;
    //                     return sdf.result;
    //                 }
    //             }
    //         }
    //     }

    //     animator: {
    //         play = {
    //             default: init,
    //             show = {
    //                 redraw: true,
    //                 from: {all: BounceLoop {duration: 1.0, end: 1.0}}
    //                 apply: {
    //                     image = { draw_bg: { radius: 15.0 } }
    //                 }
    //             }
    //             init = {
    //                 from: {all: Snap}
    //                 apply: {
    //                     element = { image = {draw_bg: { radius: 0.0 }} }
    //                 }
    //             }
    //         }
    //     }
    // }

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
        <Row2> { margin: { top: 85.0 }}
        <Row3> { margin: { top: 170.0 }}

        <Row1> { margin: { top: 255.0 }}
        <Row2> { margin: { top: 340.0 }}
        <Row3> { margin: { top: 425.0 }}

        <Row1> { margin: { top: 510.0 }}
        <Row2> { margin: { top: 595.0 }}
        <Row3> { margin: { top: 680.0 }}
        
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