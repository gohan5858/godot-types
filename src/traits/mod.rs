//! Godot型との変換トレイト

/// godot-typesからGodot型への変換トレイト
pub trait ToGodot<T> {
    /// Godot型に変換する
    fn to_godot(self) -> T;
}

/// Godot型からgodot-typesへの変換トレイト
pub trait FromGodot<T> {
    /// Godot型から変換する
    fn from_godot(value: T) -> Self;
}
