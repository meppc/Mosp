
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

pub static CURRENT_LANGUAGE: Lazy<Language> = Lazy::new(|| Language::Chinese);

pub type TranslationKey = &'static str;

type TranslationDictionary = HashMap<TranslationKey, String>;

type LanguageDictionary = HashMap<Language, TranslationDictionary>;

static TRANSLATIONS: Lazy<LanguageDictionary> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    let mut en = HashMap::new();
    en.insert("app_title", "Mosp".to_string());
    en.insert("canvas_placeholder", "Canvas will be implemented here".to_string());
    en.insert("properties", "Properties".to_string());
    en.insert("tool_properties", "Tool properties will be shown here".to_string());
    en.insert("brush_tool", "Brush".to_string());
    en.insert("shape_tool", "Shape".to_string());
    en.insert("arrow_tool", "Arrow".to_string());
    en.insert("eraser_tool", "Eraser".to_string());
    en.insert("text_tool", "Text".to_string());
    en.insert("selection_tool", "Selection".to_string());
    
    let mut zh = HashMap::new();
    zh.insert("app_title", "墨司".to_string());
    zh.insert("canvas_placeholder", "画布将在此处实现".to_string());
    zh.insert("properties", "属性".to_string());
    zh.insert("tool_properties", "工具属性将在此处显示".to_string());
    zh.insert("brush_tool", "画笔".to_string());
    zh.insert("shape_tool", "形状".to_string());
    zh.insert("arrow_tool", "箭头".to_string());
    zh.insert("eraser_tool", "橡皮擦".to_string());
    zh.insert("text_tool", "文本".to_string());
    zh.insert("selection_tool", "选择".to_string());
    
    translations.insert(Language::English, en);
    translations.insert(Language::Chinese, zh);
    
    translations
});

pub fn get_text(key: TranslationKey) -> String {
    TRANSLATIONS
        .get(&*CURRENT_LANGUAGE)
        .and_then(|dict| dict.get(key))
        .cloned()
        .unwrap_or_else(|| key.to_string())
}
