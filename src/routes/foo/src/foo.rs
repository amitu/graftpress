use crate::pages::BasicPage;
use crate::widgets::TextWidget;
use realm::{Page, Widget, HTML};



pub fn layout(in_: &In) -> acko_base::Result<AckoResponse> {
    Ok(AckoResponse::Page(
        Layout::FullWidth(
            Header::AckoTech(in_.into()),
            FullWidth::Responsive(Responsive::AboutAckoTech(in_.into())),
            FullWidth::Empty,
            FullWidth::Empty,
            FullWidth::Empty,
            FullWidth::Empty,
            FullWidth::Empty,
            FullWidth::Empty,
            FullWidth::Empty,
            Footer::AckoTech(in_.into()),
            LeftPopup::Empty,
            RightPopup::Empty,
            Modal::Empty,
        ),
        c_i18n::acko_tech_title(&in_),
        Some(reverse::about()),
    ))
}
