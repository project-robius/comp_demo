use makepad_widgets::*;
use crate::stack_navigation::StackNavigationWidgetRefExt;
use crate::rounded_corners::RoundedCornersItemSetWidgetRefExt;
use crate::rounded_images::RoundedImagesItemSetWidgetRefExt;
use crate::bitmap_text::BitmapTextItemWidgetRefExt;
use crate::transparency::TransparencyWidgetRefExt;
use crate::color::ColorWidgetRefExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::rounded_corners::RoundedCorners;
    import crate::rounded_images::RoundedImages;
    import crate::bitmap_text::BitmapText;
    import crate::transparency::Transparency;
    import crate::color::Color;
    import crate::stack_navigation::*;

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
                                <Cell> {label = {text: " 矢量文字\n Vector Text"}}
                            }
                            
                            <Row> {
                                <Cell> {label = {text: " 矢量图片\n Vector\n Image"}}
                                <Cell> {label = {text: " 全屏模糊\n Full Blur"}}
                                <Cell> {label = {text: " 控件模糊\n Component Blur"}}
                                transparency_button = <Cell> {label = {text: " 毛玻璃效果\n Trans-\n parency"}}
                            }
                            
                            <Row> {
                                <Cell> {label = {text: " 模糊穿透?\n Fuzzy Blur"}}
                                <Cell> {label = {text: " 渐变模糊?\n Gradient\n Blur"}}
                                <Cell> {label = {text: " 控件阴影\n Component\n Shadow"}}
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
                        body = {
                            header = { content = { title_container = {
                                title = {
                                    text: "Rounded Corners"
                                }
                            }}}
                            rounded_corners_view = <RoundedCorners> {}
                        }
                    }

                    rounded_images_nav = <StackNavigationView> {
                        body = {
                            header = { content = { title_container = {
                                title = {
                                    text: "Bitmap Images"
                                }
                            }}}
                            rounded_images_view = <RoundedImages> {}
                        }
                    }

                    bitmap_text_nav = <StackNavigationView> {
                        body = {
                            header = { content = { title_container = {
                                title = {
                                    text: "Bitmap Text"
                                }
                            }}}
                            bitmap_text_view = <BitmapText> {}
                        }
                    }

                    transparency_nav = <StackNavigationView> {
                        background = { draw_bg: {opacity: 1.0}}
                        body = {
                            header = { content = { title_container = {
                                title = {
                                    text: " "
                                }
                            }}}
                            transparency_view = <Transparency> {}
                        }
                    }

                    color_nav = <StackNavigationView> {
                        body = {
                            header = { content = { title_container = {
                                title = {
                                    text: "BG Color"
                                }
                            }}}
                            color_view = <Color> {}
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::stack_navigation::live_design(cx);
        crate::rounded_corners::live_design(cx);
        crate::rounded_images::live_design(cx);
        crate::bitmap_text::live_design(cx);
        crate::transparency::live_design(cx);
        crate::color::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }
        
        let actions = self.ui.handle_widget_event(cx, event);

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

        if self.ui.link_label(id!(transparency_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(transparency_nav),
                cx
            );

            let mut transparency = self.ui.transparency(id!(transparency_view));
            transparency.restart_animation(cx);
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