use makepad_widgets::*;

use crate::stack_navigation::StackNavigationWidgetRefExt;
use crate::blur_image::{BlurImageWidgetRefExt, BlurImageSetWidgetRefExt};

use crate::rounded_corners::RoundedCornersItemSetWidgetRefExt;
use crate::rounded_images::RoundedImagesItemSetWidgetRefExt;
use crate::bitmap_text::BitmapTextItemWidgetRefExt;
use crate::vector_text::VectorTextItemWidgetRefExt;
use crate::transparency::TransparencyWidgetRefExt;
use crate::component_blur::BlurBitmapTextItemWidgetRefExt;
use crate::component_shadow::ComponentShadowItemSetWidgetRefExt;
use crate::color::ColorWidgetRefExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::rounded_corners::RoundedCorners;
    import crate::rounded_images::RoundedImages;
    import crate::bitmap_text::BitmapText;
    import crate::vector_text::VectorText;
    import crate::transparency::Transparency;
    import crate::color::Color;
    import crate::component_blur::ComponentBlur;
    import crate::full_blur::FullBlur;
    import crate::component_shadow::ComponentShadow;
    import crate::stack_navigation::*;

    import makepad_draw::shader::std::*;
    IMG_0 = dep("crate://self/resources/images/0.png")

    Cell = <View> {
        flow: Down
        height: Fit
        
        label = <LinkLabel> {
            width: Fill,
            height: Fit,
            text: "Hello",
            draw_text: {
                wrap: Word,
                text_style: {
                    font_size: 11.0,
                }

                fn get_color(self) -> vec4 {
                    return #1E90FF
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
        spacing: 15.0
        height: 80.0
    }

    Grid = <View> {
        flow: Down
        spacing: 15.0
        height: Fit
        
        padding: 5.0
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
                                rounded_corner_button = <Cell> {label = {text: " 圆角矩形\n Rounded\n Corner"}}
                                rounded_images_button = <Cell> {label = {text: " 位图图片\n Bitmap\n Image"}}
                                bitmap_text_button = <Cell> {label = {text: " 点阵文字\n Bitmap Text"}}
                                vector_text_button = <Cell> {label = {text: " 矢量文字\n Vector Text"}}
                            }
                            
                            <Row> {
                                <Cell> {label = {text: " 矢量图片\n Vector\n Image"}}
                                full_blur_button = <Cell> {label = {text: " 全屏模糊\n Full Blur"}}
                                component_blur_button = <Cell> {label = {text: " 控件模糊\n Component\n Blur"}}
                                transparency_button = <Cell> {label = {text: " 毛玻璃效果\n Trans-\n parency"}}
                            }
                            
                            <Row> {
                                <Cell> {label = {text: " 模糊穿透?\n Fuzzy Blur"}}
                                <Cell> {label = {text: " 渐变模糊?\n Gradient\n Blur"}}
                                component_shadow_button = <Cell> {label = {text: " 控件阴影\n Component\n Shadow"}}
                                <Cell> {label = {text: " 复杂阴影\n Complex\n Shadow"}}
                            }
                            
                            <Row> {
                                color_button = <Cell> {label = {text: " 背景取色?\n BG Color"}}
                                <Cell> {label = {text: " 控件描边\n Component\n Stroke"}}
                                <EmptyCell> {}
                                <EmptyCell> {}
                            }
                        }
                        
                        <View> { width: Fill, height: Fill }
                    }

                    rounded_corners_nav = <StackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Rounded Corners"
                            }
                        }}}
                        body = {
                            rounded_corners_view = <RoundedCorners> {}
                        }
                    }

                    rounded_images_nav = <StackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Bitmap Images"
                            }
                        }}}
                        body = {
                            rounded_images_view = <RoundedImages> {}
                        }
                    }

                    bitmap_text_nav = <StackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Bitmap Text"
                            }
                        }}}
                        body = {
                            bitmap_text_view = <BitmapText> {}
                        }
                    }

                    vector_text_nav = <StackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Vector Text"
                            }
                        }}}
                        body = {
                            vector_text_view = <VectorText> {}
                        }
                    }

                    transparency_nav = <StackNavigationView> {
                        background = { draw_bg: {opacity: 1.0}}
                        header = { content = { title_container = {
                            title = {
                                text: "Transparency"
                                draw_text: {
                                    color: #fff
                                } 
                            }
                        }}}
                        body = {
                            transparency_view = <Transparency> {}
                        }
                    }

                    color_nav = <StackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "BG Color"
                            }
                        }}}
                        body = {
                            color_view = <Color> {}
                        }
                    }

                    full_blur_nav = <StackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Full Blur",
                                draw_text: {
                                    color: #fff
                                }
                            }
                        }}}
                        body = {
                            margin: 0.0
                            full_blur_view = <FullBlur> {}
                        }
                    }

                    component_blur_nav = <StackNavigationView> {
                        background = { draw_bg: {opacity: 1.0}}
                        header = { content = { title_container = {
                            title = {
                                text: "Component Blur",
                                draw_text: {
                                    color: #fff
                                }
                            }
                        }}}
                        body = {
                            component_blur_view = <ComponentBlur> {}
                        }
                    }

                    component_shadow_nav = <StackNavigationView> {
                        header = { content = { title_container = {
                            title = {
                                text: "Component Shadow"
                            }
                        }}}
                        body = {
                            component_shadow_view = <ComponentShadow> {}
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

        crate::stack_navigation::live_design(cx);
        crate::blur_image::live_design(cx);

        crate::rounded_corners::live_design(cx);
        crate::rounded_images::live_design(cx);
        crate::bitmap_text::live_design(cx);
        crate::vector_text::live_design(cx);
        crate::transparency::live_design(cx);
        crate::full_blur::live_design(cx);
        crate::component_blur::live_design(cx);
        crate::component_shadow::live_design(cx);
        crate::color::live_design(cx);
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

        if self.ui.link_label(id!(color_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(color_nav),
                cx
            );

            let mut color = self.ui.color(id!(color_view));
            color.restart_animation(cx);
        }
    }
}