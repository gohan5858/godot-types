# godot-types

Godot の組み込み数学型を Rust の純粋な型として扱うための境界ライブラリです。

`domain/app -> godot-types -> godot` の依存方向を維持し、ドメイン層が `godot` クレートへ直接依存しない構成を作ることを目的にしています。

## 提供型

### 2D型
- `Vec2` (`Vector2` 互換) - 2D浮動小数点ベクトル
- `Vec2i` (`Vector2i` 互換) - 2D整数ベクトル
- `Rect` (`Rect2` 互換) - 2D長方形
- `Transform2` (`Transform2D` 互換) - 2D変換行列
- `Color` (`Color` 互換) - RGBA色

### 3D型
- `Vec3` (`Vector3` 互換) - 3D浮動小数点ベクトル
- `Vec3i` (`Vector3i` 互換) - 3D整数ベクトル
- `Transform3` (`Transform3D` 互換) - 3D変換行列
- `Quat` (`Quaternion` 互換) - クォータニオン
- `Aabb` (`Aabb` 互換) - 3D軸平行境界ボックス

すべて FFI を露出しない純粋なRust `struct` として提供します。

## 特徴

- Godot 互換の基本演算 API
- `ToGodot<T>` / `FromGodot<T>` による双方向変換
- `godot` feature で `to_domain()` 拡張トレイトを提供
- Godot との挙動差分を検証する Golden Tests を同梱

## インストール

公開前は `path` 依存で利用してください。

```toml
[dependencies]
godot-types = { path = "../godot-types" }
```

## Feature Flags

```toml
[dependencies]
godot-types = { path = "../godot-types", features = ["serde", "approx", "rand", "godot"] }
```

- `serde`: 各型に `Serialize` / `Deserialize` を付与
- `approx`: `AbsDiffEq` / `RelativeEq` / `UlpsEq` を実装
- `rand`: `Vec2::random_unit` / `Vec2::random_range` を有効化
- `godot`: `godot` クレート型との変換実装と拡張トレイトを有効化

## 基本的な使い方

```rust
use godot_types::prelude::*;

let position = Vec2::new(100.0, 200.0);
let velocity = Vec2::new(5.0, 3.0);

let next = position + velocity;
let direction = velocity.normalized();
let angle = direction.angle();

let rect = Rect::new(Vec2::ZERO, Vec2::new(64.0, 64.0));
let center = rect.center();

let tint = Color::rgba(0.2, 0.4, 0.8, 1.0);
let hex = tint.to_hex();
```

## Godot との相互変換

`godot` feature を有効にすると、`ToGodot` / `FromGodot` と `to_domain()` が使えます。

```rust
use godot::builtin::Vector2;
use godot_types::prelude::*;

let v = Vec2::new(1.0, 2.0);
let gv: Vector2 = v.to_godot();

let back = gv.to_domain();
assert_eq!(back, v);
```

## テスト

```bash
cargo test
cargo test --all-features
cargo test --features godot
```

`tests/godot_golden.rs` で Godot 側の型と挙動を比較し、互換性を検証しています。

## ライセンス

`MIT OR Apache-2.0`
