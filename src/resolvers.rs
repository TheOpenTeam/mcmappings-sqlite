/*
 *
 *  * Created: 2026-2-23 9:42:0
 *  * File: resolvers.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
/// Resolvers.
/// Proguard 是 vanilla的，mojang于1.14.4+引入的映射表、、
/// Tiny 是Fabric的
/// Srg/Tsrg 是Forge的
pub mod proguard;
pub mod tiny;
pub mod srg;