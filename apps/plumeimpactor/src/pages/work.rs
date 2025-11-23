use wxdragon::prelude::*;

#[derive(Clone)]
pub struct WorkPage {
    pub panel: Panel,
    status_text: StaticText,
    back_button: Button,
}

pub fn create_work_page(frame: &Frame) -> WorkPage {
    let panel = Panel::builder(frame).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    sizer.add_stretch_spacer(1);

    let activity_indicator = ActivityIndicator::builder(&panel).build();
    activity_indicator.start();
    sizer.add(&activity_indicator, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 10);


    let status_text = StaticText::builder(&panel)
        .with_label("Idle")
        .with_style(StaticTextStyle::AlignCenterHorizontal)
        .with_size(Size { width: 300, height: 30 })
        .build();
    sizer.add(&status_text, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 10);

    sizer.add_stretch_spacer(1);

    let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let back_button = Button::builder(&panel)
        .with_label("Back")
        .build();

    back_button.enable(false);

    button_sizer.add(&back_button, 0, SizerFlag::All, 0);
    button_sizer.add_stretch_spacer(1);

    sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Bottom, 13);

    panel.set_sizer(sizer, true);

    WorkPage { 
        panel,
        status_text,
        back_button,
    }
}

impl WorkPage {
    pub fn set_status_text(&self, text: &str) {
        self.status_text.set_label(text);
    }

    pub fn enable_back_button(&self, enable: bool) {
        self.back_button.enable(enable);
    }

    pub fn set_back_handler(&self, on_back: impl Fn() + 'static) {
        self.back_button.on_click(move |_| {
            on_back();
        });
    }
}
