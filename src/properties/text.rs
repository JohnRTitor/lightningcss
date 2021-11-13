use cssparser::*;
use crate::traits::{Parse, ToCss};
use crate::macros::enum_property;
use crate::values::length::{Length, LengthPercentage};
use crate::values::color::CssColor;
use crate::printer::Printer;
use bitflags::bitflags;
use std::fmt::Write;

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#text-transform-property
enum_property!(TextTransformCase,
  None,
  Uppercase,
  Lowercase,
  Capitalize
);

impl Default for TextTransformCase {
  fn default() -> TextTransformCase {
    TextTransformCase::None
  }
}

bitflags! {
  pub struct TextTransformOther: u8 {
    const FullWidth    = 0b00000001;
    const FullSizeKana = 0b00000010;
  }
}

impl Parse for TextTransformOther {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    let location = input.current_source_location();
    let ident = input.expect_ident()?;
    match_ignore_ascii_case! { &ident,
      "full-width" => Ok(TextTransformOther::FullWidth),
      "full-size-kana" => Ok(TextTransformOther::FullSizeKana),
      _ => Err(location.new_unexpected_token_error(
        cssparser::Token::Ident(ident.clone())
      ))
    }
  }
}

impl ToCss for TextTransformOther {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    let mut needs_space = false;
    if self.contains(TextTransformOther::FullWidth) {
      dest.write_str("full-width")?;
      needs_space = true;
    }

    if self.contains(TextTransformOther::FullSizeKana) {
      if needs_space {
        dest.write_char(' ')?;
      }
      dest.write_str("full-size-kana")?;
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextTransform {
  case: TextTransformCase,
  other: TextTransformOther
}

impl Parse for TextTransform {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    let mut case = None;
    let mut other = TextTransformOther::empty();

    loop {
      if case.is_none() {
        if let Ok(c) = input.try_parse(TextTransformCase::parse) {
          case = Some(c);
          if c == TextTransformCase::None {
            other = TextTransformOther::empty();
            break
          }
          continue
        }
      }

      if let Ok(o) = input.try_parse(TextTransformOther::parse) {
        other |= o;
        continue
      }

      break
    }

    Ok(TextTransform {
      case: case.unwrap_or_default(),
      other
    })
  }
}

impl ToCss for TextTransform {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    let mut needs_space = false;
    if self.case != TextTransformCase::None || self.other.is_empty() {
      self.case.to_css(dest)?;
      needs_space = true;
    }

    if !self.other.is_empty() {
      if needs_space {
        dest.write_char(' ')?;
      }
      self.other.to_css(dest)?;
    }
    Ok(())
  }
}

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#white-space-property
enum_property!(WhiteSpace,
  ("normal", Normal),
  ("pre", Pre),
  ("nowrap", NoWrap),
  ("pre-wrap", PreWrap),
  ("break-spaces", BreakSpaces),
  ("pre-line", PreLine)
);

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#word-break-property
enum_property!(WordBreak,
  ("normal", Normal),
  ("keep-all", KeepAll),
  ("break-all", BreakAll),
  ("break-word", BreakWord)
);

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#line-break-property
enum_property!(LineBreak,
  Auto,
  Loose,
  Normal,
  Strict,
  Anywhere
);
// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#hyphenation
enum_property!(Hyphens,
  None,
  Manual,
  Auto
);

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#overflow-wrap-property
enum_property!(OverflowWrap,
  ("normal", Normal),
  ("break-word", BreakWord),
  ("anywhere", Anywhere)
);

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#text-align-property
enum_property!(TextAlign,
  ("start", Start),
  ("end", End),
  ("left", Left),
  ("right", Right),
  ("center", Center),
  ("justify", Justify),
  ("match-parent", MatchParent),
  ("justify-all", JustifyAll)
);

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#text-align-last-property
enum_property!(TextAlignLast,
  ("auto", Auto),
  ("start", Start),
  ("end", End),
  ("left", Left),
  ("right", Right),
  ("center", Center),
  ("justify", Justify),
  ("match-parent", MatchParent)
);

// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#text-justify-property
enum_property!(TextJustify,
  ("auto", Auto),
  ("none", None),
  ("inter-word", InterWord),
  ("inter-character", InterCharacter)
);

/// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#word-spacing-property
#[derive(Debug, Clone, PartialEq)]
pub enum Spacing {
  Normal,
  Length(Length)
}

impl Parse for Spacing {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    if input.try_parse(|input| input.expect_ident_matching("normal")).is_ok() {
      return Ok(Spacing::Normal)
    }

    let length = Length::parse(input)?;
    Ok(Spacing::Length(length))
  }
}

impl ToCss for Spacing {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    match self {
      Spacing::Normal => dest.write_str("normal"),
      Spacing::Length(len) => len.to_css(dest)
    }
  }
}

/// https://www.w3.org/TR/2021/CRD-css-text-3-20210422/#text-indent-property
#[derive(Debug, Clone, PartialEq)]
pub struct TextIndent {
  value: LengthPercentage,
  hanging: bool,
  each_line: bool
}

impl Parse for TextIndent {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    let mut value = None;
    let mut hanging = false;
    let mut each_line = false;

    loop {
      if value.is_none() {
        if let Ok(val) = input.try_parse(LengthPercentage::parse) {
          value = Some(val);
          continue
        }
      }

      if !hanging {
        if input.try_parse(|input| input.expect_ident_matching("hanging")).is_ok() {
          hanging = true;
          continue
        }
      }

      if !each_line {
        if input.try_parse(|input| input.expect_ident_matching("each-line")).is_ok() {
          each_line = true;
          continue
        }
      }

      break
    }

    if let Some(value) = value {
      Ok(TextIndent {
        value,
        hanging,
        each_line
      })
    } else {
      Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
    }
  }
}

impl ToCss for TextIndent {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    self.value.to_css(dest)?;
    if self.hanging {
      dest.write_str(" hanging")?;
    }
    if self.each_line {
      dest.write_str(" each-line")?;
    }
    Ok(())
  }
}

// https://www.w3.org/TR/2020/WD-css-text-decor-4-20200506/#text-decoration-line-property
bitflags! {
  pub struct TextDecorationLine: u8 {
    const Underline     = 0b00000001;
    const Overline      = 0b00000010;
    const LineThrough   = 0b00000100;
    const Blink         = 0b00001000;
    const SpellingError = 0b00010000;
    const GrammarError  = 0b00100000;
  }
}

impl Default for TextDecorationLine {
  fn default() -> TextDecorationLine {
    TextDecorationLine::empty()
  }
}

impl Parse for TextDecorationLine {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    let mut value = TextDecorationLine::empty();
    let mut any = false;

    loop {
      let flag: Result<_, ParseError<'i, ()>> = input.try_parse(|input| {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        Ok(match_ignore_ascii_case! { &ident,
          "none" if value.is_empty() => TextDecorationLine::empty(),
          "underline" => TextDecorationLine::Underline,
          "overline" => TextDecorationLine::Overline,
          "line-through" => TextDecorationLine::LineThrough,
          "blink" =>TextDecorationLine::Blink,
          "spelling-error" if value.is_empty() => TextDecorationLine::SpellingError,
          "grammar-error" if value.is_empty() => TextDecorationLine::GrammarError,
          _ => return Err(location.new_unexpected_token_error(
            cssparser::Token::Ident(ident.clone())
          ))
        })
      });

      if let Ok(flag) = flag {
        value |= flag;
        any = true;
      } else {
        break
      }
    }

    if !any {
      return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
    }

    Ok(value)
  }
}

impl ToCss for TextDecorationLine {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    if self.is_empty() {
      return dest.write_str("none")
    }

    if self.contains(TextDecorationLine::SpellingError) {
      return dest.write_str("spelling-error")
    }

    if self.contains(TextDecorationLine::GrammarError) {
      return dest.write_str("grammar-error")
    }

    let mut needs_space = false;
    macro_rules! val {
      ($val: ident, $str: expr) => {
        if self.contains(TextDecorationLine::$val) {
          if needs_space {
            dest.write_char(' ')?;
          }
          dest.write_str($str)?;
          needs_space = true;
        }
      };
    }

    val!(Underline, "underline");
    val!(Overline, "overline");
    val!(LineThrough, "line-through");
    val!(Blink, "blink");
    Ok(())
  }
}

// https://www.w3.org/TR/2020/WD-css-text-decor-4-20200506/#text-decoration-style-property
enum_property!(TextDecorationStyle,
  Solid,
  Double,
  Dotted,
  Dashed,
  Wavy
);

impl Default for TextDecorationStyle {
  fn default() -> TextDecorationStyle {
    TextDecorationStyle::Solid
  }
}

/// https://www.w3.org/TR/2020/WD-css-text-decor-4-20200506/#text-decoration-width-property
#[derive(Debug, Clone, PartialEq)]
pub enum TextDecorationThickness {
  Auto,
  FromFont,
  LengthPercentage(LengthPercentage)
}

impl Default for TextDecorationThickness {
  fn default() -> TextDecorationThickness {
    TextDecorationThickness::Auto
  }
}

impl Parse for TextDecorationThickness {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    if input.try_parse(|input| input.expect_ident_matching("auto")).is_ok() {
      return Ok(TextDecorationThickness::Auto)
    }

    if input.try_parse(|input| input.expect_ident_matching("from-font")).is_ok() {
      return Ok(TextDecorationThickness::FromFont)
    }

    let lp = LengthPercentage::parse(input)?;
    Ok(TextDecorationThickness::LengthPercentage(lp))
  }
}

impl ToCss for TextDecorationThickness {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    match self {
      TextDecorationThickness::Auto => dest.write_str("auto"),
      TextDecorationThickness::FromFont => dest.write_str("from-font"),
      TextDecorationThickness::LengthPercentage(lp) => lp.to_css(dest)
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextDecoration {
  line: TextDecorationLine,
  thickness: TextDecorationThickness,
  style: TextDecorationStyle,
  color: CssColor
}

impl Parse for TextDecoration {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    let mut line = None;
    let mut thickness = None;
    let mut style = None;
    let mut color = None;

    loop {
      macro_rules! prop {
        ($key: ident, $type: ident) => {
          if $key.is_none() {
            if let Ok(val) = input.try_parse($type::parse) {
              $key = Some(val);
              continue
            }
          }
        };
      }

      prop!(line, TextDecorationLine);
      prop!(thickness, TextDecorationThickness);
      prop!(style, TextDecorationStyle);
      prop!(color, CssColor);
      break
    }

    Ok(TextDecoration {
      line: line.unwrap_or_default(),
      thickness: thickness.unwrap_or_default(),
      style: style.unwrap_or_default(),
      color: color.unwrap_or(CssColor::current_color())
    })
  }
}

impl ToCss for TextDecoration {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    self.line.to_css(dest)?;
    if self.line.is_empty() {
      return Ok(())
    }

    let mut needs_space = true;
    if self.thickness != TextDecorationThickness::default() {
      dest.write_char(' ')?;
      self.thickness.to_css(dest)?;
      needs_space = true;
    }

    if self.style != TextDecorationStyle::default() {
      if needs_space {
        dest.write_char(' ')?;
      }
      self.style.to_css(dest)?;
      needs_space = true;
    }

    if self.color != CssColor::current_color() {
      if needs_space {
        dest.write_char(' ')?;
      }
      self.color.to_css(dest)?;
    }

    Ok(())
  }
}