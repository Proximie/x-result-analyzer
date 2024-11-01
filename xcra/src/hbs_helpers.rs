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
