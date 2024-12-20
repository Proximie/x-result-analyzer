use handlebars::Context;
use handlebars::Handlebars;
use handlebars::Helper;
use handlebars::HelperResult;
use handlebars::JsonRender;
use handlebars::Output;
use handlebars::RenderContext;
use handlebars::RenderErrorReason;
use std::env;

pub fn result_to_emoji(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = helper
        .param(0)
        .ok_or(RenderErrorReason::ParamNotFoundForIndex(
            "result_to_emoji",
            0,
        ))?;

    let binding = param.value().to_string();
    let input = binding.trim_matches('"');

    if input == "Success" {
        out.write("✅")?;
    } else if input == "Failure" {
        out.write("❌")?;
    } else {
        out.write(param.value().render().as_ref())?;
    }

    Ok(())
}

pub fn get_file_name(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let location = helper
        .param(0)
        .map(|v| v.value().as_str())
        .unwrap_or_default();
    if let Some(file) = parse_file_location(location.unwrap_or("")) {
        let relative_file = strip_pwd(&file);
        out.write(&relative_file)?;
    }
    Ok(())
}

pub fn get_line(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let location = helper
        .param(0)
        .map(|v| v.value().as_str())
        .unwrap_or_default();
    if let Some(line) = parse_file_line(location.unwrap_or("")) {
        out.write(&line.to_string())?;
    }
    Ok(())
}

fn parse_file_line(location: &str) -> Option<u32> {
    let (locations, _) = location.split_once(";").unwrap_or((location, ""));
    let (_, line) = locations.split_once(':')?;
    line.parse().ok()
}

fn parse_file_location(location: &str) -> Option<String> {
    let (locations, _) = location.split_once(";").unwrap_or((location, ""));
    let (file, _) = locations.split_once(':')?;
    Some(file.to_string())
}

fn strip_pwd(file_path: &str) -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let path = std::path::Path::new(file_path);

    match path.strip_prefix(&current_dir) {
        Ok(relative_path) => relative_path.to_string_lossy().to_string(),
        Err(_) => file_path.to_string(),
    }
}
