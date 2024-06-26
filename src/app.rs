use makepad_widgets::*;

use crate::blur_image::{BlurImageWidgetRefExt, BlurImageSetWidgetRefExt};

use crate::rounded_corners::RoundedCornersItemSetWidgetRefExt;
use crate::rounded_images::RoundedImagesItemSetWidgetRefExt;
use crate::bitmap_text::BitmapTextItemWidgetRefExt;
use crate::vector_text::VectorTextItemWidgetRefExt;
use crate::transparency::TransparencyWidgetRefExt;
use crate::component_blur::BlurBitmapTextItemWidgetRefExt;
use crate::component_shadow::ComponentShadowItemSetWidgetRefExt;
use crate::complex_shadow::ComplexShadowItemWidgetRefExt;
use crate::component_stroke::ComponentStrokeItemSetWidgetRefExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::rounded_corners::RoundedCorners;
    import crate::rounded_images::RoundedImages;
    import crate::bitmap_text::BitmapText;
    import crate::vector_text::VectorText;
    import crate::transparency::Transparency;
    import crate::component_blur::ComponentBlur;
    import crate::full_blur::FullBlur;
    import crate::component_shadow::ComponentShadow;
    import crate::complex_shadow::ComplexShadow;
    import crate::component_stroke::ComponentStroke;

    import makepad_draw::shader::std::*;
    IMG_BACKGROUND = dep("crate://self/resources/images/bg.jpg");

    Cell = <View> {
        flow: Down
        height: Fit

        label = <LinkLabel> {
            width: Fill,
            height: 300.,
            text: "Hello",
            draw_text: {
                wrap: Word,
                text_style: {
                    font_size: 13.0,
                }

                fn get_color(self) -> vec4 {
                    return #006fff
                }
            }
        }
    }

    EmptyCell = <View> {
        width: Fill,
        height: Fit
    }

    Row = <View> {
        flow: Right
        spacing: 10.0
        height: 80.0
    }

    Grid = <View> {
        flow: Down
        spacing: 10.0
        height: Fit

        padding: 5.0
    }

    DemoStackNavigationView = <StackNavigationView> {
        header = {
            show_bg: false,
            content = {
                title_container = {
                    title = {
                        draw_text: {
                            color: #3c3c3c
                        }
                    }
                }
                button_container = {
                    left_button = {
                        draw_icon: {
                            color: #3c3c3c;
                        }
                    }
                }
            }
        }
    }

    App = {{App}} {
        ui: <Window> {
            window: {position: vec2(0, 0), inner_size: vec2(400, 800)},
            pass: {clear_color: #fff}
            //show_performance_view: true

            body = {
                width: Fill,
                height: Fill,

                navigation = <StackNavigation> {
                    root_view = {
                        width: Fill, height: Fill
                        flow: Down

                        <View> { width: Fill, height: Fill }

                        <Grid> {
                            <Row> {
                                rounded_corner_button = <Cell> {label = {text: "圆角矩形\nRounded\nCorner"}}
                                rounded_images_button = <Cell> {label = {text: "位图图片\nBitmap\nImage"}}
                                bitmap_text_button = <Cell> {label = {text: "点阵文字\nBitmap Text"}}
                                vector_text_button = <Cell> {label = {text: "矢量文字\nVector Text"}}
                            }

                            <Row> {
                                <Cell> {label = {text: "矢量图片\nVector\nImage"}}
                                full_blur_button = <Cell> {label = {text: "全屏模糊\nFull Blur"}}
                                component_blur_button = <Cell> {label = {text: "控件模糊\nControl\nBlur"}}
                                transparency_button = <Cell> {label = {text: "毛玻璃效果\nTrans-\nparency"}}
                            }

                            <Row> {
                                fuzzy_blur_button = <Cell> {label = {text: "模糊穿透?\nFuzzy Blur"}}
                                gradient_blur_button = <Cell> {label = {text: "渐变模糊?\nGradient\nBlur"}}
                                component_shadow_button = <Cell> {label = {text: "控件阴影\nControl\nShadow"}}
                                complex_shadow_button = <Cell> {label = {text: "复杂阴影\nPath\nShadow"}}
                            }

                            <Row> {
                                color_button = <Cell> {label = {text: "背景取色?\nBG Color"}}
                                component_stroke_button = <Cell> {label = {text: "控件描边\nControl\nStroke"}}
                                <EmptyCell> {}
                                <EmptyCell> {}
                            }
                        }

                        <View> { width: Fill, height: Fill }
                    }

                    rounded_corners_nav = <DemoStackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Rounded Corners"
                            }
                        }}}
                        body = {
                            rounded_corners_view = <RoundedCorners> {}
                        }
                    }

                    rounded_images_nav = <DemoStackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Bitmap Images"
                            }
                        }}}
                        body = {
                            rounded_images_view = <RoundedImages> {}
                        }
                    }

                    bitmap_text_nav = <DemoStackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Bitmap Text"
                            }
                        }}}
                        body = {
                            bitmap_text_view = <BitmapText> {}
                        }
                    }

                    vector_text_nav = <DemoStackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Vector Text"
                            }
                        }}}
                        body = {
                            vector_text_view = <VectorText> {}
                        }
                    }

                    transparency_nav = <DemoStackNavigationView> {
                        background = {
                            visible: true,
                            <Image> {
                                source: (IMG_BACKGROUND),
                                width: Fill,
                                height: Fill,
                            }
                        }   
                        header = {
                            show_bg: true,
                            content = {
                                title_container = {
                                    title = {
                                        text: "Transparency",
                                        draw_text: {
                                            color: #fff
                                        }
                                    }
                                }
                            }
                        }
                        
                        body = {
                            transparency_view = <Transparency> {}
                        }
                    }

                    full_blur_nav = <DemoStackNavigationView> {
                        header = {
                            show_bg: true,
                            content = {
                                title_container = {
                                    title = {
                                        text: "Full Blur",
                                        draw_text: {
                                            color: #fff
                                        }
                                    }
                                }
                                button_container = {
                                    left_button = {
                                        draw_icon: {
                                            color: #fff;
                                        }
                                    }
                                }
                            }
                        }
                        body = {
                            margin: { top: 0.0 }
                            full_blur_view = <FullBlur> {}
                        }
                    }

                    component_blur_nav = <DemoStackNavigationView> {
                        background = {
                            visible: true,
                            <Image> {
                                source: (IMG_BACKGROUND),
                                width: Fill,
                                height: Fill,
                            }
                        }
                        header = {
                            show_bg: true,
                            content = {
                                title_container = {
                                    title = {
                                        text: "Component Blur",
                                        draw_text: {
                                            color: #fff
                                        }
                                    }
                                }
                                button_container = {
                                    left_button = {
                                        draw_icon: {
                                            color: #fff;
                                        }
                                    }
                                }
                            }
                        }
                        body = {
                            component_blur_view = <ComponentBlur> {}
                        }
                    }

                    component_shadow_nav = <DemoStackNavigationView> {
                        header = {
                            show_bg: true,
                            content = {
                                title_container = {
                                    title = {
                                        text: "Component Shadow",
                                        draw_text: {
                                            color: #fff
                                        }
                                    }
                                }
                                button_container = {
                                    left_button = {
                                        draw_icon: {
                                            color: #fff;
                                        }
                                    }
                                }
                            }
                        }
                        body = {
                            component_shadow_view = <ComponentShadow> {}
                        }
                    }

                    complex_shadow_nav = <DemoStackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Complex Shadow"
                            }
                        }}}
                        body = {
                            complex_shadow_view = <ComplexShadow> {}
                        }
                    }

                    only_image_nav = <DemoStackNavigationView> {
                        background = {
                            visible: true,
                            <Image> {
                                source: (IMG_BACKGROUND),
                                width: Fill,
                                height: Fill,
                            }
                        }
                        header = {
                            show_bg: false,
                            content = {
                                title_container = {
                                    title = {
                                        text: "Title",
                                        draw_text: {
                                            color: #0000
                                        }
                                    }
                                }
                                button_container = {
                                    left_button = {
                                        draw_icon: {
                                            color: #fff;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    component_stroke_nav = <DemoStackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Component Stroke"
                            }
                        }}}
                        body = {
                            component_stroke_view = <ComponentStroke> {}
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);

        crate::blur_image::live_design(cx);

        crate::rounded_corners::live_design(cx);
        crate::rounded_images::live_design(cx);
        crate::bitmap_text::live_design(cx);
        crate::vector_text::live_design(cx);
        crate::transparency::live_design(cx);
        crate::full_blur::live_design(cx);
        crate::component_blur::live_design(cx);
        crate::component_shadow::live_design(cx);
        crate::complex_shadow::live_design(cx);
        crate::component_stroke::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl MatchEvent for App{
    fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
        if self.ui.link_label(id!(rounded_corner_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(rounded_corners_nav),
                cx
            );

            let rounded_corners = self.ui.view(id!(rounded_corners_view));
            let mut animated_items = rounded_corners.rounded_corners_item_set(
                ids!(rounded_corners_item)
            );
            animated_items.restart_animation(cx);
        }

        if self.ui.link_label(id!(rounded_images_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(rounded_images_nav),
                cx
            );

            let rounded_images = self.ui.view(id!(rounded_images_view));
            let mut animated_items = rounded_images.rounded_images_item_set(
                ids!(rounded_images_item)
            );
            animated_items.restart_animation(cx);
        }

        if self.ui.link_label(id!(bitmap_text_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(bitmap_text_nav),
                cx
            );

            let bitmap_text = self.ui.view(id!(bitmap_text_view));
            let mut animated_item = bitmap_text.bitmap_text_item(
                id!(bitmap_text_item)
            );
            animated_item.restart_animation(cx);
        }

        if self.ui.link_label(id!(vector_text_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(vector_text_nav),
                cx
            );

            let vector_text = self.ui.view(id!(vector_text_view));
            let mut animated_item = vector_text.vector_text_item(
                id!(vector_text_item)
            );
            animated_item.restart_animation(cx);
        }

        if self.ui.link_label(id!(transparency_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(transparency_nav),
                cx
            );

            let mut transparency = self.ui.transparency(id!(transparency_view));
            transparency.restart_animation(cx);
        }

        if self.ui.link_label(id!(full_blur_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(full_blur_nav),
                cx
            );

            let full_blur_view = self.ui.view(id!(full_blur_view));
            let mut animated_item = full_blur_view.blur_image(
                id!(blur_image_item)
            );
            animated_item.restart_animation(cx);
        }

        if self.ui.link_label(id!(component_blur_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(component_blur_nav),
                cx
            );

            let component_blur_view = self.ui.view(id!(component_blur_view));
            let mut animated_items = component_blur_view.blur_image_set(
                ids!(blur_image_item)
            );
            animated_items.restart_animation(cx);

            let mut animated_items = component_blur_view.blur_bitmap_text_item(
                id!(blur_bitmap_text_item)
            );
            animated_items.restart_animation(cx);
        }

        if self.ui.link_label(id!(component_shadow_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(component_shadow_nav),
                cx
            );

            let component_shadow_view = self.ui.view(id!(component_shadow_view));
            let mut animated_items = component_shadow_view.component_shadow_item_set(
                ids!(component_shadow_item)
            );
            animated_items.restart_animation(cx);
        }

        if self.ui.link_label(id!(complex_shadow_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(complex_shadow_nav),
                cx
            );

            let complex_shadow_view = self.ui.view(id!(complex_shadow_view));
            let mut animated_item = complex_shadow_view.complex_shadow_item(
                id!(complex_shadow_item)
            );
            animated_item.restart_animation(cx);
        }

        if self.ui.link_label(id!(fuzzy_blur_button.label)).pressed(&actions) ||
            self.ui.link_label(id!(gradient_blur_button.label)).pressed(&actions) {

            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(only_image_nav),
                cx
            );
        }

        if self.ui.link_label(id!(component_stroke_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(component_stroke_nav),
                cx
            );

            let component_stroke_view = self.ui.view(id!(component_stroke_view));
            let mut animated_items = component_stroke_view.component_stroke_item_set(
                ids!(component_stroke_item)
            );
            animated_items.restart_animation(cx);
        }
    }
}