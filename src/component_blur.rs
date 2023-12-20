use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    IMG_0 = dep("crate://self/resources/images/0.png")
    IMG_1 = dep("crate://self/resources/images/1.png")
    IMG_2 = dep("crate://self/resources/images/2.png")
    IMG_3 = dep("crate://self/resources/images/3.png")

    BlurImage = {{BlurImage}} {
        blur_image_item = <Image> {
            width: 60.0,
            height: 60.0,

            draw_bg: {
                instance blur_radius: 10.0

                fn get_color_pos(self, pos: vec2) -> vec4 {
                    return sample2d(self.image, pos).xyzw;
                }

                fn coeffs(i: float) -> float {
                    if i - 0.1 < 0.0 { return 0.012318109844189502 }
                    else if i - 0.1 < 1.0 { return 0.014381474814203989 }
                    else if i - 0.1 < 2.0 { return 0.016623532195728208 }
                    else if i - 0.1 < 3.0 { return 0.019024086115486723 }
                    else if i - 0.1 < 4.0 { return 0.02155484948872149 }
                    else if i - 0.1 < 5.0 { return 0.02417948052890078 }
                    else if i - 0.1 < 6.0 { return 0.02685404941667096 }
                    else if i - 0.1 < 7.0 { return 0.0295279624870386 }
                    else if i - 0.1 < 8.0 { return 0.03214534135442581 }
                    else if i - 0.1 < 9.0 { return 0.03464682117793548 }
                    else if i - 0.1 < 10.0 { return 0.0369716985390341 }
                    else if i - 0.1 < 11.0 { return 0.039060328279673276 }
                    else if i - 0.1 < 12.0 { return 0.040856643282313365 }
                    else if i - 0.1 < 13.0 { return 0.04231065439216247 }
                    else if i - 0.1 < 14.0 { return 0.043380781642569775 }
                    else if i - 0.1 < 15.0 { return 0.044035873841196206 }
                    else if i - 0.1 < 16.0 { return 0.04425662519949865 }
                    else if i - 0.1 < 17.0 { return 0.044035873841196206 }
                    else if i - 0.1 < 18.0 { return 0.043380781642569775 }
                    else if i - 0.1 < 19.0 { return 0.04231065439216247 }
                    else if i - 0.1 < 20.0 { return 0.040856643282313365 }
                    else if i - 0.1 < 21.0 { return 0.039060328279673276 }
                    else if i - 0.1 < 22.0 { return 0.0369716985390341 }
                    else if i - 0.1 < 23.0 { return 0.03464682117793548 }
                    else if i - 0.1 < 24.0 { return 0.03214534135442581 }
                    else if i - 0.1 < 25.0 { return 0.0295279624870386 }
                    else if i - 0.1 < 26.0 { return 0.02685404941667096 }
                    else if i - 0.1 < 27.0 { return 0.02417948052890078 }
                    else if i - 0.1 < 28.0 { return 0.02155484948872149 }
                    else if i - 0.1 < 29.0 { return 0.019024086115486723 }
                    else if i - 0.1 < 30.0 { return 0.016623532195728208 }
                    else if i - 0.1 < 31.0 { return 0.014381474814203989 }
                    else if i - 0.1 < 32.0 { return 0.012318109844189502 }
                    else { return 0.0; }
                }

                fn pixel(self) -> vec4 {
                    let m = 16.0;
                    let n = 33.0;

                    let radius = self.blur_radius / 1000.0 * self.rect_size.x;

                    let color = vec4(0.0, 0.0, 0.0, 0.0);
                    let pos = self.pos;
                    let i = 0.0;
                    let j = 0.0;
                    for step in 0..33 {
                        j = 0.0;
                        for b in 0..33 {
                            pos = self.pos + vec2((i - m) / self.rect_size.x, (j - m) / self.rect_size.y) * radius;
                            color += self.get_color_pos(pos) * coeffs(i) * coeffs(j);
                            j += 1.0;
                        }
                        i += 1.0;
                    }
                    
                    return color;
                }
            }
        }

        animator: {
            play = {
                default: init,
                show = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 2.0, end: 1.0}}
                    apply: { blur_image_item = { draw_bg: {blur_radius: 10.0}} }
                }
                init = {
                    from: {all: Snap}
                    apply: { blur_image_item = { draw_bg: {blur_radius: 3.0}} }
                }
            }
        }
    }

    ComponentBlur = <View> {
        <View> {
            width: Fill,
            height: 300,
            padding: 20.0,
            <BlurImage> {
                blur_image_item = { source: (IMG_0) }
            }
            <BlurImage> {
                blur_image_item = { source: (IMG_1) }
            }
            <BlurImage> {
                blur_image_item = { source: (IMG_2) }
            }
            <BlurImage> {
                blur_image_item = { source: (IMG_3) }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BlurImage {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for BlurImage {
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

impl BlurImageRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}

impl BlurImageSet {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        for mut item in self.iter() {
            item.restart_animation(cx);
        }
    }
}