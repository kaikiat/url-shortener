use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{json, Json};
use validator::{Validate, ValidationError, ValidationErrors};

// #[derive(Debug)]
// pub struct Errors {
//     errors: ValidationErrors,
// }

// impl<'r> Responder<'r, 'static> for Errors {
//     fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
//         use validator::ValidationErrorsKind::Field;

//         let mut errors = json!({});
//         for (field, field_errors) in self.errors.into_errors() {
//             if let Field(field_errors) = field_errors {
//                 errors[field] = field_errors
//                     .into_iter()
//                     .map(|field_error| field_error.code)
//                     .collect();
//             }
//         }

//         status::Custom(
//             Status::UnprocessableEntity,
//             Json(json!({ "errors": errors })),
//         )
//         .respond_to(req)
//     }
// }


#[derive(Debug)]
pub enum Errors {
    Forbidden,
    InternalServerError(String),
    BadGateway,
    ValidationErrors(ValidationErrors),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Errors::Forbidden => write!(f, "Forbidden"),
            Errors::InternalServerError(ref message) => write!(f, "Internal server error: {}", message),
            Errors::BadGateway => write!(f, "Bad gateway"),
            // Errors::ValidationErrors => write!(f, "Validation error"),
            Errors::ValidationErrors(ref err) => write!(f, "Validation error: {}", err),
        }
    }
}

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Errors {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            Errors::Forbidden => rocket::response::Response::build().status(rocket::http::Status::Forbidden).ok(),
            Errors::InternalServerError(message) => {
                rocket::response::Response::build()
                    .header(rocket::http::ContentType::Plain)
                    .sized_body(message.len(), std::io::Cursor::new(message))
                    .status(rocket::http::Status::InternalServerError)
                    .ok()
            },
            Errors::BadGateway => rocket::response::Response::build().status(rocket::http::Status::BadGateway).ok(),
            Errors::ValidationErrors(validation_errors) => {
                use validator::ValidationErrorsKind::Field;
                let mut errors = json!({});
                for (field, field_errors) in validation_errors.into_errors() {
                    if let Field(field_errors) = field_errors {
                        errors[field] = field_errors
                            .into_iter()
                            .map(|field_error| field_error.code)
                            .collect();
                    }
                }

                status::Custom(
                    Status::UnprocessableEntity,
                    Json(json!({ "errors": errors })),
                )
                .respond_to(req)
            },
        }
    }
}

pub struct FieldValidator {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    /// Convenience method to trigger early returns with ? operator.
    pub fn check(self) -> Result<(), Errors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(Errors::ValidationErrors(self.errors))
        }
    }

    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
    where
        T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be blank"));
            T::default()
        })
    }
}