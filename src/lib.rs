use neon::prelude::*;
use strsim::{
    damerau_levenshtein,
    // hamming,
    jaro,
    jaro_winkler,
    levenshtein,
    normalized_damerau_levenshtein,
    normalized_levenshtein,
    osa_distance,
    sorensen_dice,
};

macro_rules! strsim_fn {
    ($func_name:ident) => {
        |mut cx: FunctionContext| -> JsResult<JsNumber> {
            let result = $func_name(
                &*(cx.argument(0)? as Handle<JsString>).value(&mut cx),
                &*(cx.argument(1)? as Handle<JsString>).value(&mut cx),
            );
            Ok(cx.number(result as f64))
        }
    };
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("damerau_levenshtein", strsim_fn!(damerau_levenshtein))?;
    // TODO handle HammingResult
    // cx.export_function("hamming", strsim_fn!(hamming))?;
    cx.export_function("jaro_winkler", strsim_fn!(jaro_winkler))?;
    cx.export_function("jaro", strsim_fn!(jaro))?;
    cx.export_function("levenshtein", strsim_fn!(levenshtein))?;
    cx.export_function(
        "normalized_damerau_levenshtein",
        strsim_fn!(normalized_damerau_levenshtein),
    )?;
    cx.export_function("normalized_levenshtein", strsim_fn!(normalized_levenshtein))?;
    cx.export_function("osa_distance", strsim_fn!(osa_distance))?;
    cx.export_function("sorensen_dice", strsim_fn!(sorensen_dice))?;

    Ok(())
}
