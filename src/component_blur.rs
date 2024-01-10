use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    FONT_LABEL = {
        font_size: 10.0,
        font: {
            path: dep("crate://self/resources/fonts/Chusung.ttf")
        }
    }

    IMG_0 = dep("crate://self/resources/images/0.png")
    IMG_1 = dep("crate://self/resources/images/1.png")
    IMG_2 = dep("crate://self/resources/images/3.png")
    IMG_3 = dep("crate://self/resources/images/4.png")

    BlurBitmapTextItem = {{BlurBitmapTextItem}} {
        width: Fill,
        height: 400

        element = <CachedView> {
            width: 300,
            height: 300,

            padding: 40.0,

            draw_bg: {
                texture image: texture2d
                instance blur_radius: 10.0

                varying radius: float
                varying initial_offset: vec2
                varying step_delta: vec2

                fn get_color_pos(self, pos: vec2) -> vec4 {
                    return sample2d_rt(self.image, pos);
                }

                fn coeffs(i: float) -> float {
                    if i - 0.1 < 0.0 { return 0.0369543295325685 }
                    else if i - 0.1 < 1.0 { return 0.05707225834646017 }
                    else if i - 0.1 < 2.0 { return 0.08056214824901288 }
                    else if i - 0.1 < 3.0 { return 0.10394046353380644 }
                    else if i - 0.1 < 4.0 { return 0.1279319631769404 }
                    else if i - 0.1 < 5.0 { return 0.14953785857076586 }
                    else if i - 0.1 < 6.0 { return 0.13014234492770832 }
                    else if i - 0.1 < 7.0 { return 0.11718098483901983 }
                    else if i - 0.1 < 8.0 { return 0.10394046353380644 }
                    else if i - 0.1 < 9.0 { return 0.0885838874611158 }
                    else if i - 0.1 < 10.0 { return 0.07253844158670234 }
                    else if i - 0.1 < 11.0 { return 0.04987059658718462 }
                    else { return 0.0; }
                }

                fn vertex(self) -> vec4 {
                    self.radius = self.blur_radius / 10000.0 * self.rect_size.x;
                    self.initial_offset = vec2(-16.0 / self.rect_size.x, -16.0 / self.rect_size.y);
                    self.step_delta = vec2(3.0 / self.rect_size.x, 3.0 / self.rect_size.y);

                    return self.clip_and_transform_vertex(self.rect_pos, self.rect_size * 2.0)
                }

                fn pixel(self) -> vec4 {
                    let color = vec4(0.0, 0.0, 0.0, 0.0);
                    let pos = self.pos;
                    let i = 0.0;
                    let j = 0.0;

                    let posx = self.initial_offset.x;
                    let posy = self.initial_offset.y;
                    for step in 0..11 {
                        j = 0.0;
                        posy = self.initial_offset.y;
                        for b in 0..11 {
                            pos = self.pos + vec2(posx, posy) * self.radius;
                            color += self.get_color_pos(pos) * coeffs(i) * coeffs(j);

                            posy += self.step_delta.x;
                            j += 1.0;
                        }
                        posx += self.step_delta.y;
                        i += 1.0;
                    }
                    
                    return color;
                }
            }

            <Label> {
                text: "长恨歌\n作者：白居易\n汉皇重色思倾国，御宇多年求不得。\n杨家有女初长成，养在深闺人未识。\n天生丽质难自弃，一朝选在君王侧。\n回眸一笑百媚生，六宫粉黛无颜色。\n春寒赐浴华清池，温泉水滑洗凝脂。\n侍儿扶起娇无力，始是新承恩泽时。\n云鬓花颜金步摇，芙蓉帐暖度春宵。..."
                draw_text: {
                    wrap: Word,
                    text_style: <FONT_LABEL> {
                        font_size: 12
                    }

                    fn get_color(self) -> vec4 {
                        return #FFFFFF
                    }
                }
            }
        }

        animator: {
            play = {
                default: init,
                show = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 2.0, end: 1.0}}
                    apply: { element = { draw_bg: {blur_radius: 6.0}} }
                }
                init = {
                    from: {all: Snap}
                    apply: { element = { draw_bg: {blur_radius: 2.0}} }
                }
            }
        }
    }

    BlurImage = {{BlurImage}} {
        image: <Image> {
            width: 60.0,
            height: 60.0,

            draw_bg: {
                instance blur_radius: 10.0
                varying radius: float
                varying initial_offset: vec2
                varying step_delta: vec2

                fn get_color_pos(self, pos: vec2) -> vec4 {
                    return sample2d(self.image, pos).xyzw;
                }

                fn coeffs(i: float) -> float {
                    if i - 0.1 < 0.0 { return 0.0369543295325685 }
                    else if i - 0.1 < 1.0 { return 0.05707225834646017 }
                    else if i - 0.1 < 2.0 { return 0.08056214824901288 }
                    else if i - 0.1 < 3.0 { return 0.10394046353380644 }
                    else if i - 0.1 < 4.0 { return 0.1279319631769404 }
                    else if i - 0.1 < 5.0 { return 0.14953785857076586 }
                    else if i - 0.1 < 6.0 { return 0.13014234492770832 }
                    else if i - 0.1 < 7.0 { return 0.11718098483901983 }
                    else if i - 0.1 < 8.0 { return 0.10394046353380644 }
                    else if i - 0.1 < 9.0 { return 0.0885838874611158 }
                    else if i - 0.1 < 10.0 { return 0.07253844158670234 }
                    else if i - 0.1 < 11.0 { return 0.04987059658718462 }
                    else { return 0.0; }
                }

                fn pixel(self) -> vec4 {
                    let color = vec4(0.0, 0.0, 0.0, 0.0);
                    let pos = self.pos;
                    let i = 0.0;
                    let j = 0.0;

                    let posx = self.initial_offset.x;
                    let posy = self.initial_offset.y;
                    for step in 0..11 {
                        j = 0.0;
                        posy = self.initial_offset.y;
                        for b in 0..11 {
                            pos = self.pos + vec2(posx, posy) * self.radius;
                            color += self.get_color_pos(pos) * coeffs(i) * coeffs(j);

                            posy += self.step_delta.x;
                            j += 1.0;
                        }
                        posx += self.step_delta.y;
                        i += 1.0;
                    }
                    
                    return color;
                }

                fn clip_and_transform_vertex(self, rect_pos:vec2, rect_size:vec2) -> vec4 {
                    let clipped: vec2 = clamp(
                        self.geom_pos * rect_size + rect_pos,
                        self.draw_clip.xy,
                        self.draw_clip.zw
                    )
                    //clipped = self.geom_pos * rect_size + rect_pos;
                    self.pos = (clipped - rect_pos) / rect_size
                    // only pass the clipped position forward
                    return self.camera_projection * (self.camera_view * (self.view_transform * vec4(
                        clipped.x,
                        clipped.y,
                        self.draw_depth + self.draw_zbias,
                        1.
                    )))
                }
                
                fn vertex(self) -> vec4 {
                    self.radius = self.blur_radius / 1000.0 * self.rect_size.x;
                    self.initial_offset = vec2(-16.0 / self.rect_size.x, -16.0 / self.rect_size.y);
                    self.step_delta = vec2(3.0 / self.rect_size.x, 3.0 / self.rect_size.y);

                    return self.clip_and_transform_vertex(self.rect_pos, self.rect_size)
                }
            }
        }

        animator: {
            play = {
                default: init,
                show = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 2.0, end: 1.0}}
                    apply: { image: { draw_bg: {blur_radius: 8.0}} }
                }
                init = {
                    from: {all: Snap}
                    apply: { image: { draw_bg: {blur_radius: 2.0}} }
                }
            }
        }
    }

    ComponentBlur = <View> {
        flow: Down,
        align: {x: 0.5, y: 0.0},

        <View> {
            flow: Right

            width: Fill,
            height: 100.0,
            padding: 20.0,
            margin: 0.0

            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    image: { source: (IMG_0) }
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    image: { source: (IMG_1) }
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    image: { source: (IMG_2) }
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    image: { source: (IMG_3) }
                }
            }
        }

        <View> {
            width: 330.0,
            height: 260.0,

            show_bg: true,
            draw_bg: {
                color: #ccc3
            }

            blur_bitmap_text_item = <BlurBitmapTextItem> {}
        }

        <View> {
            width: 330.0,
            height: 260.0,

            show_bg: true,
            draw_bg: {
                color: #ccc3
            }

            margin: { top: 40.0 },
            padding: 40.0,

            <Label> {
                text: "长恨歌\n作者：白居易\n汉皇重色思倾国，御宇多年求不得。\n杨家有女初长成，养在深闺人未识。\n天生丽质难自弃，一朝选在君王侧。\n回眸一笑百媚生，六宫粉黛无颜色。\n春寒赐浴华清池，温泉水滑洗凝脂。\n侍儿扶起娇无力，始是新承恩泽时。\n云鬓花颜金步摇，芙蓉帐暖度春宵。..."
                draw_text: {
                    wrap: Word,
                    text_style: <FONT_LABEL> {
                        font_size: 12
                    }

                    fn get_color(self) -> vec4 {
                        return #FFFFFF
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BlurImage {
    #[live] #[wrap]
    image: Image,

    #[animator]
    animator: Animator,
}

impl Widget for BlurImage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.image.redraw(cx);
        }
        self.image.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.animator.need_init() || self.animator_in_state(cx, id!(play.init)) {
            self.animator_play(cx, id!(play.show));
        }

        self.image.draw_walk(cx, walk)
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

#[derive(Live, LiveHook, Widget)]
pub struct BlurBitmapTextItem {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for BlurBitmapTextItem {
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

impl BlurBitmapTextItemRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}