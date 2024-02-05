use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::blur_image::*;

    IMG_BACKGROUND = dep("crate://self/resources/images/bg.jpg");

    FullBlur = <View> {
        blur_image_item = <BlurImage> {
            step4 = { step3 = { step2 = { step1 = { image = { source: (IMG_BACKGROUND) }}}}}

            animator: {
                play = {
                    default: init,
                    show = {
                        redraw: true,
                        from: {all: BounceLoop {duration: 2.0, end: 1.0}}
                        apply: {
                            step4 = {
                                draw_bg: {blursize: 0.5}
                                step3 = {
                                    draw_bg: {blursize: 0.5}
                                    step2 = {
                                        draw_bg: {blursize: 0.5}
                                        step1 = {
                                            draw_bg: {blursize: 0.5}
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
    }
}