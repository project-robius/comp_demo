use makepad_widgets::*;
use rand::Rng;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    Color = {{Color}} {
        width: Fill,
        height: Fill,
        flow: Down

        padding: 20.0
        spacing: 5.0

        align: {x: 0.5, y: 0.5}

        <View> { width: 1.0, height: Fill }

        element = <RectView> {
            flow: Down,
            width: 100.0,
            height: 100.0,

            padding: 5.0,
            spacing: 2.0,

            draw_bg: {
                color: #ccc,
            }

            red_label = <Label> {
                text: "Red",
                draw_text: {
                    color: #000,
                    wrap: Word,
                    text_style: {
                        font_size: 12.0
                    }
                }
            }

            green_label = <Label> {
                text: "Green",
                draw_text: {
                    color: #000,
                    wrap: Word,
                    text_style: {
                        font_size: 12.0
                    }
                }
            }

            blue_label = <Label> {
                text: "Blue",
                draw_text: {
                    color: #000,
                    wrap: Word,
                    text_style: {
                        font_size: 12.0
                    }
                }
            }
        }

        button = <Button> {
            width: 80.0,

            text: "切换图片",

            draw_text: {
                fn get_color(self) -> vec4 {
                    return #1E90FF
                }
            }

            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.rect(
                        1.,
                        1.,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0
                    );

                    sdf.fill_keep(#fff);
                    sdf.stroke(#000, 1.0);

                    return sdf.result
                }
            }
        }

        <View> { width: 1.0, height: Fill }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Color {
    #[deref]
    view: View,

    #[live(1000.0)]
    color_update_timeout: f64,

    #[rust]
    timer: Timer,
}

impl Widget for Color {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.timer.is_event(event).is_some() {
            self.change_color(cx);
            self.timer = cx.start_timeout(self.color_update_timeout)
        }

        let actions = cx.capture_actions(|cx| self.view.handle_event(cx, event, scope));

        if self.button(id!(button)).clicked(&actions) {
            self.change_color(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Color {
    fn change_color(&mut self, cx: &mut Cx) {
        let mut rng = rand::thread_rng();
        let red: f32 = rng.gen();
        let green: f32 = rng.gen();
        let blue: f32 = rng.gen();

        self.label(id!(red_label)).set_text(format!("{}", red).as_str());
        self.label(id!(green_label)).set_text(format!("{}", green).as_str());
        self.label(id!(blue_label)).set_text(format!("{}", blue).as_str());

        let color = Vec4{
            x: red,
            y: green,
            z: blue,
            w: 1.0
        };
        self.view(id!(element)).apply_over(cx, live!{ draw_bg: { color: (color) }});

        self.view.redraw(cx);
    }
}

impl ColorRef {
    pub fn restart_animation(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);

            cx.stop_timer(inner.timer);
            inner.timer = cx.start_timeout(inner.color_update_timeout);
        }
    }
}