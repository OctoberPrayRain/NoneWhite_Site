use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub code: u16,
    pub data: T,
    pub message: String,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            code: 0,
            data,
            message: message.into(),
        }
    }

    pub fn error(code: u16, data: T, message: impl Into<String>) -> Self {
        Self {
            code,
            data,
            message: message.into(),
        }
    }
}
