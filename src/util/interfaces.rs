use std::path::Path;

use json::JsonValue;

#[macro_export]
macro_rules! Res {
    [$type:ty] => {
        Result<$type, String>
    };
}
pub trait Serializable<T, J>
{   
    fn from_path(path: &Path) -> Res![T];
    fn from_json_value(json_value: JsonValue) -> Res![J];
    fn to_json_value(&self) -> JsonValue;

}