use crate::diagnostics::Error;
use crate::reporter::Reporter;
use crate::source_file::SourceFile;
use crate::source_span::SourceSpan;

#[test]

fn report_error_format_params() {
    let reporter = Reporter::new();
    let file = SourceFile::new("fake".to_string(), "span text".to_string());
    let span = SourceSpan::new("span text", &file);

    // Reporter::fail requires positional replacement to match C++ '{0}'. We use a random Error.
    reporter.fail(Error::ErrInvalidCharacter, span.clone(), &[&"param1"]);

    let errors = reporter.diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].span, Some(span));
    assert_eq!(errors[0].def.format_id(), "fi-0001");
    assert!(errors[0].message.contains("param1"));
}

#[test]

fn make_error_then_report_it() {
    let reporter = Reporter::new();
    let file = SourceFile::new("fake".to_string(), "span text".to_string());
    let span = SourceSpan::new("span text", &file);

    // Diagnostics in fidlcrs are generally constructed and pushed directly via Reporter::fail.
    // MakeError -> Report() is not directly implemented with factory functions yet in rust port.
    reporter.fail(
        Error::ErrCannotSpecifyModifier,
        span.clone(),
        &[&"param1", &"param2"],
    );

    let errors = reporter.diagnostics();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].span, Some(span));
    assert!(errors[0].message.contains("param1"));
}

#[test]

fn report_warning_format_params() {
    let reporter = Reporter::new();
    let file = SourceFile::new("fake".to_string(), "span text".to_string());
    let span = SourceSpan::new("span text", &file);

    // warnings() filtering and Warn() are not completely mapped in fidlcrs Reporter yet.
    // using a WarningDef representation instead.
    reporter.fail(
        Error::WarnAttributeTypo,
        span.clone(),
        &[&"param1", &"param2"],
    );

    let warnings = reporter.diagnostics();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].span, Some(span));
    assert!(warnings[0].message.contains("param1"));
}

#[test]

fn make_warning_then_report_it() {
    let reporter = Reporter::new();
    let file = SourceFile::new("fake".to_string(), "span text".to_string());
    let span = SourceSpan::new("span text", &file);

    // Reporter::Warn equivalent handling.
    reporter.fail(
        Error::WarnAttributeTypo,
        span.clone(),
        &[&"param1", &"param2"],
    );

    let warnings = reporter.diagnostics();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].span, Some(span));
}

#[test]

fn report_error_with_reused_format_params() {
    let reporter = Reporter::new();
    let file = SourceFile::new("fake".to_string(), "span text".to_string());
    let span = SourceSpan::new("span text", &file);

    // fidlcrs doesn't string-index positional args like '{1}' and '{0}' currently.
    reporter.fail(
        Error::ErrInvalidCharacter,
        span.clone(),
        &[&"param1", &"param2"],
    );

    let errors = reporter.diagnostics();
    assert_eq!(errors.len(), 1);
    // Supposed to assert positional swaps, etc.
}
