use makepad_widgets::*;
use crate::stack_navigation::StackNavigationWidgetRefExt;
use crate::rounded_corners::RoundedCornersItemSetWidgetRefExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::rounded_corners::RoundedCorners;
    import crate::rounded_images::RoundedImages;
    import crate::stack_navigation::*;

    Cell = <View> {
        flow: Down
        height: Fit
        
        label = <LinkLabel> {
            width: Fill,
            height: Fit,
            text: "Hello",
            draw_text: {
                color: #1E90FF,
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
                navigation = <StackNavigation> {
                    root_view = {
                        width: Fill, height: Fill
                        flow: Down
                        
                        <View> { width: Fill, height: Fill }
                        
                        <Grid> {
                            <Row> {
                                rounded_corner_button = <Cell> {label = {text: "圆角矩形\n Rounded Corner"}}
                                rounded_images_button = <Cell> {label = {text: "位图图片\n Bitmap Image"}}
                                <Cell> {label = {text: "点阵文字\n Bitmap Text"}}
                                <Cell> {label = {text: "矢量文字\n Vector Text"}}
                            }
                            
                            <Row> {
                                <Cell> {label = {text: "矢量图片\n Vector Image"}}
                                <Cell> {label = {text: "全屏模糊\n Full Blur"}}
                                <Cell> {label = {text: "控件模糊\n Component Blur"}}
                                <Cell> {label = {text: "毛玻璃效果\n Trans-\n parency"}}
                            }
                            
                            <Row> {
                                <Cell> {label = {text: "模糊穿透?\n Fuzzy Blur"}}
                                <Cell> {label = {text: "渐变模糊?\n Gradient Blur"}}
                                <Cell> {label = {text: "控件阴影\n Component Shadow"}}
                                <Cell> {label = {text: "复杂阴影\n Complex Shadow"}}
                            }
                            
                            <Row> {
                                <Cell> {label = {text: "背景取色?\n BG Color"}}
                                <Cell> {label = {text: "控件描边\n Component Stroke"}}
                                <EmptyCell> {}
                                <EmptyCell> {}
                            }
                        }
                        
                        <View> { width: Fill, height: Fill }
                    }

                    rounded_corners_nav = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Rounded Corners"
                                    }
                                }
                            }
                        }
                        rounded_corners_view = <RoundedCorners> {}
                    }

                    rounded_images_nav = <StackNavigationView> {
                        header = {
                            content = {
                                title_container = {
                                    title = {
                                        text: "Rounded Images"
                                    }
                                }
                            }
                        }
                        <RoundedImages> {}
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
                ids!(rounded_corner_item)
            );
            animated_items.restart_animation(cx);
        }

        if self.ui.link_label(id!(rounded_images_button.label)).pressed(&actions) {
            let mut navigation = self.ui.stack_navigation(id!(navigation));
            navigation.show_stack_view_by_id(
                live_id!(rounded_images_nav),
                cx
            )
        }
    }
}