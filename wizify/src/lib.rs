//! # wizify
//! 
//! Quick and simple tool to generate a creation wizard based directly off a struct. Uses your field name along with dialoguer to provide
//! basic creation and validation tasks. Currently, this only works with structs, but in the future, I want to add in enums for selection 
//! prompts.
//! 
//! ⚠️ WARNING ⚠️ This project is under active development. Each release may have breaking changes as I develop the library.
//! 
//! ## Usage
//! In order to generate a creation wizard from a struct, you need to use the `Wizard` derive macro. Here's an example:
//! 
//! ```rust
//! #[derive(Wizard, Debug)]
//! #[wizard(
//!     begin_msg = "✨ Hello, and welcome to the wizify creation wizard 🐧\n\n",
//!     closing_msg = "\nThat was the demonstration of the wizify creation wizard. 🌛",
//!     prefix = " ❓ => "
//! )]
//! struct Testing {
//!     #[wizard(prompt = "Enter your name (Optional)")]
//!     name: Option<String>,
//!     #[wizard(prompt = "Enter your favorite color")]
//!     favorite_color: String,
//!     #[wizard(prompt = "Enter your favorite number between 0 and 9", validation = input < 10)]
//!     favorite_number: i32,
//! }
//! ```
//! As of right now, this only supports basic primitives, but in the future I would like to support custom struct fields as well.
//!
//! #### *For information on the specifics of the derive options, see the derive macro documentation below.*


/// Struct macro for creating wizard
///
/// ## Derive
///
/// Using `#[derive(Wizard)]` on the stuct will give you basic functionality right out of the box, but there are some options to make a 
/// beautiful wizard with ease.
/// 
/// ### Struct Options
/// On the struct level, there are a couple options that will apply to the wizard as a whole.
/// 
/// #### Opening / Closing Messages
/// By adding `#[wizard(begin_msg = "<message>")]` or `#[wizard(begin_msg = <message>)]` to your struct parameters, you can add a message at the beginning or the
/// end of your creation wizard. These will each be printed once over the entire wizard. By default, these messages do not include any 
/// styling, so you will need to add any newlines that you want.
/// 
/// #### Prefixes for Your Prompts
/// If you want to add a custom prefix to all prompts in your wizard, you can do that by adding the `#[wizard(prefix = "<prefix>")]` to your
/// struct parameters. This will apply to the beginning of every prompt for your fields.
/// 
/// These two options added together will look like this when run:
/// 
/// ```rust
/// ✨ Hello, and welcome to the wizify creation wizard 🐧
/// 
///  ❓ => name: Test
///  ❓ => favorite_color: color
///  ❓ => favorite_number: 0
/// 
/// That was the demonstration of the wizify creation wizard. 🌛
/// ```
/// 
/// ### Field Options
/// Each of the fields can also be customized to use custom prompts and validation.
/// 
/// #### Custom Prompts
/// Using `#[wizard(prompt = "<prompt>")]` allows you to override the default which is to use the name of the field. This will only work for the
/// annotated field.
/// 
/// #### Custom Validation
/// Using `#[wizard(validation = <expression>)]` allows you to add in extra validation for your prompt. 
/// 
/// 📝 NOTE
/// By default, dialoguer will always check
/// the type of your prompt, so you do not need to include that in the validation. 
/// 
/// To write a validation, use `input` to mean what the user inputs into the dialoguer prompt, and your validation will be evaluated based on that.
/// For instance, in the example above we use `validation = input < 10`. This will get expanded into:
/// 
/// ```rust
/// if input < 10 {
///     Ok(())
/// } else {
///     Err("This input is not valid...")
/// }
/// ```
/// Putting these together, the above earlier example will show this as the full wizard:
/// 
/// ```rust
/// ✨ Hello, and welcome to the wizify creation wizard 🐧
/// 
///  ❓ => Enter your name (Optional): Test
///  ❓ => Enter your favorite color: color
///  ❓ => Enter your favorite number between 0 and 9: 0
/// 
/// That was the demonstration of the wizify creation wizard. 🌛
/// ```
pub use wizify_derive::*;

/// Trait is populated using the Wizard derive annotation
pub trait Wizard {
    fn wizard() -> Self;
}
