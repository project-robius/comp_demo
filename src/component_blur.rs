use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    FONT_LABEL = {
        font_size: 10.0,
        font: {
            path: dep("crate://self/resources/fonts/Chusung.ttf")
        }
    }

    BlurStage = <ViewBase> {
        optimize: Texture,
        draw_bg: {
            texture image: texture2d

            uniform blursize: 0.0,
            uniform blurstd: 1.5,
            uniform blurx: 1.0,
            uniform blury: 0.0,
            varying g1: float,
            varying g2: float,
            varying g3: float,
            varying g4: float,
            varying g5: float,

            varying gaussscale: float,

            varying o0: vec2,

            varying o1a: vec2,
            varying o2a: vec2,
            varying o3a: vec2,
            varying o4a: vec2,
            varying o5a: vec2,

            varying o1b: vec2,
            varying o2b: vec2,
            varying o3b: vec2,
            varying o4b: vec2,
            varying o5b: vec2,

            fn vertex(self) -> vec4
            {
                let x = self.blurx;
                let y = self.blury;
                let dpi = self.dpi_factor;
                let ceil_size = ceil(self.rect_size * dpi) / dpi
                let floor_pos = floor(self.rect_pos * dpi) / dpi

                let offset = 0.003 * self.blursize / max(x,y);
                let standard_deviation = 0.0001 + self.blurstd *0.003;
                let st_dev_sqr = standard_deviation * standard_deviation;

                let off1 = offset;
                let off2 = 2.0*offset;
                let off3 = 3.0*offset;
                let off4 = 4.0*offset;
                let off5 = 5.0*offset;

                let mainscale = (1.0 / sqrt(2*PI*st_dev_sqr));
                let stddevscale = 1.0/ (2*st_dev_sqr);

                self.g1 =  pow(E, -((off1*off1)* stddevscale));
                self.g2 =  pow(E, -((off2*off2)* stddevscale));
                self.g3 =  pow(E, -((off3*off3)* stddevscale));
                self.g4 =  pow(E, -((off4*off4)* stddevscale));
                self.g5 =  pow(E, -((off5*off5)* stddevscale));

                self.gaussscale = 1.0/(1.0 +  (self.g1 + self.g2 + self.g3 + self.g4 + self.g5 )*2.0);

                let pos = self.clip_and_transform_vertex(self.rect_pos, self.rect_size);
                self.o0 = self.pos*0.5;

                self.o1a = self.o0 + vec2(off1*x,off1*y);
                self.o2a = self.o0 + vec2(off2*x,off2*y);
                self.o3a = self.o0 + vec2(off3*x,off3*y);
                self.o4a = self.o0 + vec2(off4*x,off4*y);
                self.o5a = self.o0 + vec2(off5*x,off5*y);

                self.o1b = self.o0 - vec2(off1*x,off1*y);
                self.o2b = self.o0 - vec2(off2*x,off2*y);
                self.o3b = self.o0 - vec2(off3*x,off3*y);
                self.o4b = self.o0 - vec2(off4*x,off4*y);
                self.o5b = self.o0 - vec2(off5*x,off5*y);

                return pos;
            }

            fn pixel(self) -> vec4
            {

                let col = sample2d_rt(self.image, self.o0) ;
                col +=  (sample2d_rt(self.image, self.o1a) + sample2d_rt(self.image, self.o1b)) * self.g1;
                col +=  (sample2d_rt(self.image, self.o2a) + sample2d_rt(self.image, self.o2b)) * self.g2 ;
                col +=  (sample2d_rt(self.image, self.o3a) + sample2d_rt(self.image, self.o3b)) * self.g3 ;
                col +=  (sample2d_rt(self.image, self.o4a) + sample2d_rt(self.image, self.o4b)) * self.g4 ;
                col +=  (sample2d_rt(self.image, self.o5a) + sample2d_rt(self.image, self.o5b)) * self.g5 ;
                col = col * self.gaussscale;

                return col ;
            }
        }
    }

    BlurBitmapTextItem = {{BlurBitmapTextItem}} {
        width: Fill,
        height: 400

        step4 = <BlurStage>{
            width: Fill,
            height: Fill,
            draw_bg:{blury: 0.0, blurx: 10.0}
            step3 = <BlurStage>{
                width: Fill,
                height: Fill,
                draw_bg:{blury: 10.0, blurx: 0.0}
                step2 = <BlurStage>{
                    width: Fill,
                    height: Fill,
                    draw_bg:{blury: 7.07, blurx: 7.07}
                    step1 = <BlurStage>{
                        width: 300,
                        height: 300,

                        padding: 40.0,
                        draw_bg:{blury: -7.07, blurx: 7.07}

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
        }

        animator: {
            play = {
                default: init,
                show = {
                    redraw: true,
                    from: {all: BounceLoop {duration: 2.0, end: 1.0}}
                    apply: {
                        step4 = {
                            draw_bg: {blursize: 0.8}
                            step3 = {
                                draw_bg: {blursize: 0.8}
                                step2 = {
                                    draw_bg: {blursize: 0.8}
                                    step1 = {
                                        draw_bg: {blursize: 0.8}
                                    }
                                }
                            }
                        }
                    }
                }
                init = {
                    from: {all: Snap}
                    apply: {
                        step4 = {
                            draw_bg: {blursize: 0.05}
                            step3 = {
                                draw_bg: {blursize: 0.05}
                                step2 = {
                                    draw_bg: {blursize: 0.05}
                                    step1 = {
                                        draw_bg: {blursize: 0.05}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    IMG_0 = dep("crate://self/resources/images/0.png")
    IMG_1 = dep("crate://self/resources/images/1.png")
    IMG_2 = dep("crate://self/resources/images/3.png")
    IMG_3 = dep("crate://self/resources/images/4.png")


    BlurImage = {{BlurImage}} {
        step4 = <BlurStage>{
            width: 60.0,
            height: 60.0,
            draw_bg:{blury: 0.0, blurx: 10.0}
            step3 = <BlurStage>{
                width: Fill,
                height: Fill,
                draw_bg:{blury: 10.0, blurx: 0.0}
                step2 = <BlurStage>{
                    width: Fill,
                    height: Fill,
                    draw_bg:{blury: 7.07, blurx: 7.07}
                    step1 = <BlurStage> {
                        image = <Image> {
                            width: 60.0,
                            height: 60.0,
                        }
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
                    apply: {
                        step4 = {
                            draw_bg: {blursize: 5.0}
                            step3 = {
                                draw_bg: {blursize: 5.0}
                                step2 = {
                                    draw_bg: {blursize: 5.0}
                                    step1 = {
                                        draw_bg: {blursize: 5.0}
                                    }
                                }
                            }
                        }
                    }
                }
                init = {
                    from: {all: Snap}
                    apply: {
                        step4 = {
                            draw_bg: {blursize: 1.0}
                            step3 = {
                                draw_bg: {blursize: 1.0}
                                step2 = {
                                    draw_bg: {blursize: 1.0}
                                    step1 = {
                                        draw_bg: {blursize: 1.0}
                                    }
                                }
                            }
                        }
                    }
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
            margin: { left: 25.0 }

            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_0) }}}}}
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_1) }}}}}
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_2) }}}}}
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <BlurImage> {
                    step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_3) }}}}}
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