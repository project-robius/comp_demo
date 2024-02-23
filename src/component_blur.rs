use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import crate::blur_image::*;

    FONT_LABEL = {
        font_size: 10.0,
        font: {
            path: dep("crate://self/resources/fonts/Chusung.ttf")
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

    CustomBlurImage = <BlurImage> {
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
                blur_image_item = <CustomBlurImage> {
                    step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_0) }}}}}
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <CustomBlurImage> {
                    step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_1) }}}}}
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <CustomBlurImage> {
                    step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_2) }}}}}
                }
            }
            <View> {
                width: Fill,
                align: {x: 0.5}
                blur_image_item = <CustomBlurImage> {
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