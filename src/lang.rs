use std::collections::HashMap;

use qwreey_utility_rs::write_map;

use crate::cli::Cli;

macro_rules! create_lang_map {
    ( $cli:ident, { $($name:ident => $default:expr),* $(,)? } ) => {
        write_map!(HashMap::<&str, String>::new(), {
            "id" => "lang".to_string(),
            $(stringify!($name).trim_start_matches("lang_") => $cli.$name.to_owned().unwrap_or_else(|| String::from($default)),)*
        })
    };
}

pub fn get_lang_map(cli: &Cli) -> HashMap<&str, String> {
    create_lang_map!(cli, {
        lang_empty_field => "'{name}' 이 비어있습니다. 다시 작성해주세요.",
        lang_no_captcha => "캡챠가 진행되지 않았습니다. 캡챠를 진행해주세요.",
        lang_hcaptcha_title => "캡챠를 진행해주세요",
        lang_minecraft_name_title => "마인크래프트 유저 이름",
        lang_minecraft_name_description => "화이트리스트에 등록할 마인크래프트 유저 이름을 입력해주세요",
        lang_comfirm => "진행하기",
        lang_ok => "알겠어요",
        lang_dialog_err_title => "잠깐만요!",
        lang_dialog_ok_title => "성공적으로 처리되었어요",
        lang_result_already => "이미 화이트리스트에 있는것을 확인했어요",
        lang_result_ok => "정상적으로 처리되었어요. 이제 서버에 접속할 수 있어요",
        lang_result_name_err => "마인크래프트 이름 형식이 잘못되었어요",
        lang_result_answer_err => "잘못된 답을 입력했어요",
        lang_result_not_exist => "해당 마인크래프트 유저를 찾지 못했어요",
        lang_result_captcha_err => "캡챠 검증에 오류가 발생했어요",
        lang_result_unknown_err => "알 수 없는 오류가 발생했어요",
    })
}
