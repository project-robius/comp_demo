use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import crate::blur_image::*;

    SHADOW_BLUR_SIZE = 0.1
    SHADOW_BLUR_STD = 1.5

    ComplexShadowItem = {{ComplexShadowItem}} {
        flow: Overlay,
        width: 300,
        height: Fill,

        step4 = <BlurStage>{
            width: Fill,
            height: Fill,
            draw_bg:{blury: 0.0, blurx: 10.0, blursize: (SHADOW_BLUR_SIZE), blurstd: (SHADOW_BLUR_STD)}
            step3 = <BlurStage>{
                width: Fill,
                height: Fill,
                draw_bg:{blury: 10.0, blurx: 0.0, blursize: (SHADOW_BLUR_SIZE), blurstd: (SHADOW_BLUR_STD)}
                step2 = <BlurStage>{
                    width: Fill,
                    height: Fill,
                    draw_bg:{blury: 7.07, blurx: 7.07, blursize: (SHADOW_BLUR_SIZE), blurstd: (SHADOW_BLUR_STD)}
                    step1 = <BlurStage>{
                        width: Fill,
                        height: Fill,

                        draw_bg:{blury: -7.07, blurx: 7.07, blursize: (SHADOW_BLUR_SIZE), blurstd: (SHADOW_BLUR_STD)}

                        label = <Label> {
                            text: "长恨歌\n作者：白居易\n汉皇重色思倾国，御宇多年求不得。\n杨家有女初长成，养在深闺人未识。\n天生丽质难自弃，一朝选在君王侧。\n回眸一笑百媚生，六宫粉黛无颜色。\n春寒赐浴华清池，温泉水滑洗凝脂。\n侍儿扶起娇无力，始是新承恩泽时。\n云鬓花颜金步摇，芙蓉帐暖度春宵。\n春宵苦短日高起，从此君王不早朝。\n承欢侍宴无闲暇，春从春游夜专夜。\n后宫佳丽三千人，三千宠爱在一身。\n金屋妆成娇侍夜，玉楼宴罢醉和春。\n姊妹弟兄皆列土，可怜光彩生门户。\n遂令天下父母心，不重生男重生女。\n骊宫高处入青云，仙乐风飘处处闻。\n缓歌谩舞凝丝竹，尽日君王看不足。\n渔阳鼙鼓动地来，惊破霓裳羽衣曲。\n九重城阙烟尘生，千乘万骑西南行。\n翠华摇摇行复止，西出都门百余里。\n六军不发无奈何，宛转蛾眉马前死。\n花钿委地无人收，翠翘金雀玉搔头。\n君王掩面救不得，回看血泪相和流。\n黄埃散漫风萧索，云栈萦纡登剑阁。\n峨嵋山下少人行，旌旗无光日色薄。\n蜀江水碧蜀山青，圣主朝朝暮暮情。\n行宫见月伤心色，夜雨闻铃肠断声。\n天旋地转回龙驭，到此踌躇不能去。\n马嵬坡下泥土中，不见玉颜空死处。\n君臣相顾尽沾衣，东望都门信马归。\n归来池苑皆依旧，太液芙蓉未央柳。\n芙蓉如面柳如眉，对此如何不泪垂。\n春风桃李花开日，秋雨梧桐叶落时。\n西宫南内多秋草，落叶满阶红不扫。"
                            draw_text: {
                                color: #1E90FF,
                                wrap: Word,
                                text_style: {
                                    font_size: 10.0
                                }

                                fn get_color(self) -> vec4 {
                                    return #111
                                }
                            }
                        }
                    }
                }
            }
        }

        element = <View> {
            width: 300,
            height: Fill,
            label = <Label> {
                text: "长恨歌\n作者：白居易\n汉皇重色思倾国，御宇多年求不得。\n杨家有女初长成，养在深闺人未识。\n天生丽质难自弃，一朝选在君王侧。\n回眸一笑百媚生，六宫粉黛无颜色。\n春寒赐浴华清池，温泉水滑洗凝脂。\n侍儿扶起娇无力，始是新承恩泽时。\n云鬓花颜金步摇，芙蓉帐暖度春宵。\n春宵苦短日高起，从此君王不早朝。\n承欢侍宴无闲暇，春从春游夜专夜。\n后宫佳丽三千人，三千宠爱在一身。\n金屋妆成娇侍夜，玉楼宴罢醉和春。\n姊妹弟兄皆列土，可怜光彩生门户。\n遂令天下父母心，不重生男重生女。\n骊宫高处入青云，仙乐风飘处处闻。\n缓歌谩舞凝丝竹，尽日君王看不足。\n渔阳鼙鼓动地来，惊破霓裳羽衣曲。\n九重城阙烟尘生，千乘万骑西南行。\n翠华摇摇行复止，西出都门百余里。\n六军不发无奈何，宛转蛾眉马前死。\n花钿委地无人收，翠翘金雀玉搔头。\n君王掩面救不得，回看血泪相和流。\n黄埃散漫风萧索，云栈萦纡登剑阁。\n峨嵋山下少人行，旌旗无光日色薄。\n蜀江水碧蜀山青，圣主朝朝暮暮情。\n行宫见月伤心色，夜雨闻铃肠断声。\n天旋地转回龙驭，到此踌躇不能去。\n马嵬坡下泥土中，不见玉颜空死处。\n君臣相顾尽沾衣，东望都门信马归。\n归来池苑皆依旧，太液芙蓉未央柳。\n芙蓉如面柳如眉，对此如何不泪垂。\n春风桃李花开日，秋雨梧桐叶落时。\n西宫南内多秋草，落叶满阶红不扫。"
                draw_text: {
                    color: #1E90FF,
                    wrap: Word,
                    text_style: {
                        font_size: 10.0
                    }

                    fn get_color(self) -> vec4 {
                        return #111
                    }
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
                        element = {
                            label = { draw_text: { font_scale: 1.2 }}
                            margin: {left: 60.0, top: 30.0}
                        }
                        step4 = {
                            margin: {left: 60.0, top: 30.0}
                            step3 = { step2 = { step1 = { label = { draw_text: { font_scale: 1.2 }}}}}
                        }
                    }
                }
                init = {
                    from: {all: Snap}
                    apply: {
                        element = {
                            label = { draw_text: { font_scale: 1.0 }}
                            margin: {left: 80.0, top: 20.0}
                        }
                        step4 = {
                            margin: {left: 80.0, top: 20.0}
                            step3 = { step2 = { step1 = { label = { draw_text: { font_scale: 1.0 }}}}}
                        }
                    }
                }
            }
        }
    }

    ComplexShadow = <View> {
        width: Fill,
        height: Fill,
        flow: Down

        padding: 5.0

        complex_shadow_item = <ComplexShadowItem> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ComplexShadowItem {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for ComplexShadowItem {
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

impl ComplexShadowItemRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id!(play.init));
            inner.redraw(cx);
        }
    }
}