# Welcome to Shanghai University Program Design and Training Platform

## Introduction

Currently `SHUpdtp` is a simple Restful API server written in Rust.
It assures running efficiency by using one of the fastest web application framework.

## Crates

### server-core

It's a **reuseable crate for actix-web project**, see more in [server-core README](crates/server-core/README.md)

### shupdtp-db

DB operation collection for SHUpdtp.

## Related Project

[online_judge](https://github.com/slhmy/online_judge)
Previous version of `SHUpdtp`. Tried some new things such as GrapQL etc.
But not well structured.

[ojFront](https://github.com/slhmy/ojFront)
Front end server for `online_judge`. More likely to be a mobile version.
Missing functionalities in creating problems or contest.
