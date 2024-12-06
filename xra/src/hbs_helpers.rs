use handlebars::Context;
use handlebars::Handlebars;
use handlebars::Helper;
use handlebars::HelperResult;
use handlebars::JsonRender;
use handlebars::Output;
use handlebars::RenderContext;
use handlebars::RenderErrorReason;

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
    if let Some((file, _, _)) = parse_location(location.unwrap_or("")) {
        out.write(file)?;
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
    if let Some((_, line, _)) = parse_location(location.unwrap_or("")) {
        out.write(&line.to_string())?;
    }
    Ok(())
}

fn parse_location(location: &str) -> Option<(&str, u32, Option<u32>)> {
    let parts: Vec<&str> = location.split(':').collect();
    match parts.len() {
        2 => {
            let file = parts[0];
            let line = parts[1].parse::<u32>().ok()?;
            Some((file, line, None))
        }
        3 => {
            let file = parts[0];
            let line = parts[1].parse::<u32>().ok()?;
            let col = parts[2].parse::<u32>().ok()?;
            Some((file, line, Some(col)))
        }
        _ => None,
    }
}
