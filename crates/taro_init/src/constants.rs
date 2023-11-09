use std::collections::HashMap;

use napi_derive::napi;
use once_cell::sync::Lazy;
use serde::Serialize;

pub static STYLE_EXT_MAP: Lazy<HashMap<&CSSType, &str>> = Lazy::new(|| {
  let mut map = HashMap::new();
  map.insert(&CSSType::Sass, "scss");
  map.insert(&CSSType::Stylus, "styl");
  map.insert(&CSSType::Less, "less");
  map.insert(&CSSType::None, "css");
  map
});

pub static FRAMEWORK_TYPE_MAP: Lazy<HashMap<&FrameworkType, &str>> = Lazy::new(|| {
  let mut map = HashMap::new();
  map.insert(&FrameworkType::Preact, "preact");
  map.insert(&FrameworkType::React, "react");
  map.insert(&FrameworkType::Vue, "vue");
  map.insert(&FrameworkType::Vue3, "vue3");
  map
});

pub static TEMPLATE_CREATOR: &str = "template_creator.js";

pub static FILE_FILTER: Lazy<Vec<&str>> =
  Lazy::new(|| vec![TEMPLATE_CREATOR, ".DS_Store", ".npmrc"]);

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[napi(string_enum)]
pub enum CSSType {
  None,
  Sass,
  Stylus,
  Less,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[napi(string_enum)]
pub enum FrameworkType {
  React,
  Preact,
  Vue,
  Vue3,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[napi(string_enum)]
pub enum NpmType {
  Yarn,
  Cnpm,
  Pnpm,
  Npm,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
#[napi(string_enum)]
pub enum CompilerType {
  Webpack4,
  Webpack5,
  Vite,
}
